#!/usr/bin/env python3

import argparse
import collections
import glob
import pathlib
import random
import re
import statistics
import subprocess


class Game:
    def __init__(self, game_id):
        self.game_id = game_id
        self._events = []
        self._find_errors = re.compile(r"(FL)?([1-9])?E[1-9]")
        self._remove_br_outs = re.compile(r"\([123B]\)")
        self._remove_cs_outs = re.compile(r"\([1-9]+\)")
        self._find_pickoffs = re.compile(r"PO([123]|CS[23H])")
        self._k_plus_throws = re.compile(r"(K[1-9]+)")
        self.missing = False

    def add_event(self, fields):
        # Expecting four fields:
        # game id, event text, event type, outs on play
        event = ""
        event_text = fields[1].strip('"')
        # event_code = int(fields[2])

        # First split by "." then split the first field by "/".
        if "." in event_text:
            event_fields = event_text.split(".")
        else:
            event_fields = [event_text]

        if "/" in event_fields[0]:
            event_fields = event_fields[0].split("/") + event_fields[1:]
        for char in ("!", "?", "#", "-"):
            if char in event_fields[0]:
                event_fields[0] = event_fields[0].replace(char, "")
        if event_fields[0].endswith("+"):
            event_fields[0] = event_fields[0].rstrip("+")
        play_text = event_fields[0]

        if play_text.isnumeric():
            if play_text == "99":
                self.missing = True
            event = "-".join(list(play_text))
        elif play_text == "W" or play_text == "K":
            event = play_text
        elif (
            play_text.startswith("S") or
            play_text.startswith("D") or
            play_text.startswith("T")
        ):
            event = play_text[0]
        elif play_text.startswith("HR"):
            # Map over the fence and inside-the-park to the same.
            event = "HR"
        elif self._find_errors.search(play_text):
            if play_text.startswith("FL"):
                event = play_text[2:]
            else:
                event = play_text
        elif play_text == "IW" or play_text == "I":
            event = "W"
        elif play_text == "FC" or play_text.startswith("FC"):
            event = "FC"
        elif play_text == "HP":
            event = "HBP"
        elif play_text == "WP":
            event = "WP"
        elif play_text == "PB":
            event = "PB"
        elif match := self._find_pickoffs.search(play_text):
            event = match.group(1)
        elif play_text.startswith("SB"):
            event = play_text
        elif play_text.startswith("CS"):
            event = "CS"
        elif play_text == "OA" or play_text.startswith("OA."):
            event = "OA"
        # elif play_text.startswith("K.BX1"):
        #     print("K. happened!")
        #     event = "K"
        elif match := self._k_plus_throws.search(play_text):
            event = "K"
        elif play_text.startswith("W+") or play_text.startswith("IW+WP"):
            event = "W"
        elif play_text.startswith("K+"):
            if "CS" in play_text:
                event = self._remove_cs_outs.sub("", play_text)
            else:
                event = play_text
        elif play_text == "C":
            event = "C"
        elif play_text.startswith("BK"):
            event = "BK"
        elif self._remove_br_outs.search(play_text):
            no_br_event_text = self._remove_br_outs.sub("", play_text)
            if no_br_event_text.isnumeric():
                event = "-".join(list(no_br_event_text))

        if not event:
            print(f"Unknown event text: '{event_text}' => '{play_text}'")
            return

        self._events.append(event)

    def _events_out(self, fields):
        outs = str(int(fields[2] != "0"))
        self._events.append(outs)

    def sequence(self):
        return list(self._events)


class TrieNode:
    def __init__(self, game, sequence):
        self.children = {}
        self.game = game
        # This holds the remaining sequence IN REVERSE ORDER.
        self.sequence = sequence

    def split(self):
        assert len(self.children) == 0
        key = self.sequence.pop()
        new_node = TrieNode(self.game, self.sequence)
        self.children[key] = new_node
        self.game = None
        self.sequence = None

    def __repr__(self):
        return f"{self.__class__.__name__}(game={self.game}, sequence={self.sequence}, children={repr(self.children)})"


class Trie:
    def __init__(self):
        # Use place-holder value so the root won't get split on the first sequence.
        self._placeholder = "_"
        self._root = TrieNode(None, [self._placeholder])

    def add_sequence(self, game, sequence):
        # print(f"{game = } :: {sequence = }")
        node = self._root
        # Reverse the sequence to allow popping from the end and not require
        # moving every element forward one slot each time.
        sequence.reverse()
        while sequence:
            key = sequence.pop()
            if key in node.children:
                node = node.children[key]
            elif len(node.children) > 0:
                node.children[key] = TrieNode(game, sequence)
                break
            else:
                # Try again with the newly split node with the next item in the sequence.
                node.split()
                sequence.append(key)

    def find_novelty(self):
        if self._placeholder in self._root.children:
            del self._root.children[self._placeholder]
        minimum_prefixes = []
        # nodes = collections.deque(self._root.children.items())
        nodes = collections.deque()
        for key, node in self._root.children.items():
            nodes.append(([key], node))
        while nodes:
            # Sequence so far.
            sequence, node = nodes.popleft()
            if node.children:
                for key, node in node.children.items():
                    seq = sequence + [key]
                    nodes.append((seq, node))
            else:
                minimum_prefixes.append((len(sequence), sequence, node.game))

        shortest_count = minimum_prefixes[0][0]
        print(f"## Fastest most unique prefixes: {shortest_count}")
        for prefix in minimum_prefixes:
            if prefix[0] == shortest_count:
                print(prefix)
            else:
                break

        longest_count = minimum_prefixes[-1][0]
        print(f"## Longest unique prefixes: {longest_count}")
        for idx in range(len(minimum_prefixes) - 1, 0, -1):
            prefix = minimum_prefixes[idx]
            if prefix[0] == longest_count:
                print(prefix)
            else:
                break

        minimum_unique = [item[0] for item in minimum_prefixes]
        print(f"Shortest unique sequence: {min(minimum_unique)}")
        print(f"Longest unique sequence: {max(minimum_unique)}")
        print(f"Mean unique sequence: {statistics.mean(minimum_unique)}")
        print(f"Median unique sequence: {statistics.median(minimum_unique)}")

    def __repr__(self):
        return repr(self._root)


def cwevent(season_dir, season):
    event_files = glob.glob("*.EV[AN]", root_dir=season_dir)
    cmd = ["cwevent", "-q", "-y", season, "-f", "0,29,34,40"] + event_files
    proc = subprocess.run(cmd, check=True, cwd=season_dir, encoding='utf-8',
                          stdout=subprocess.PIPE)
    stdout = proc.stdout
    return stdout.splitlines()


def gen_sequence():
    games = 20
    max_outs = 12
    out_prop = 0.6
    for _ in range(games):
        year = random.randint(2000, 2025)
        month = random.randint(4, 9)
        day = random.randint(1, 30)
        game_id = f"{year}{month:>02}{day:>02}"
        sequence = []
        outs = 0
        while outs < max_outs:
            event = random.random()
            if event <= out_prop:
                sequence.append("1")
                outs += 1
            else:
                sequence.append("0")
        print(game_id, repr(sequence))
        yield game_id, sequence


def options():
    parser = argparse.ArgumentParser()
    parser.add_argument("--test", action="store_true")
    parser.add_argument("season_dir")
    parser.add_argument("season", nargs="+")
    args = parser.parse_args()
    return args


def main():
    args = options()

    trie = Trie()

    if args.test:
        for game_id, sequence in gen_sequence():
            trie.add_sequence(game_id, sequence)
        print(repr(trie))
        trie.find_novelty()
        return

    season_base_dir = pathlib.Path(args.season_dir)
    for season in args.season:
        game = None
        season_dir = season_base_dir / season
        for line in cwevent(season_dir, season):
            if ',' not in line:
                continue
            fields = line.split(',')
            game_id = fields[0].strip('"')
            if game and game_id != game.game_id:
                if not game.missing:
                    trie.add_sequence(game.game_id, game.sequence())
                game = Game(game_id)
            elif game is None:
                game = Game(game_id)
            game.add_event(fields)
    trie.find_novelty()

if __name__ == '__main__':
    main()
