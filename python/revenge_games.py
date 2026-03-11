#!/usr/bin/env python3

import argparse
import collections
import csv
import operator

GAME_LIMIT = 100


class Game:
    def __init__(self, game_fields, team_side):
        self._fields = game_fields
        self.date = game_fields[0]
        self.number_of_game = game_fields[1]
        if team_side == "home":
            self.team = game_fields[6]
            self.game_number = int(game_fields[8])
            self.team_score = int(game_fields[10])
            self.opponent_score = int(game_fields[9])
        elif team_side == "visitor":
            self.team = game_fields[3]
            self.game_number = int(game_fields[5])
            self.team_score = int(game_fields[9])
            self.opponent_score = int(game_fields[10])
        else:
            raise RuntimeError(f"Invalid team: {team_side}")

        self.run_diff = abs(self.team_score - self.opponent_score)
        if self.team_score > self.opponent_score:
            self.result = "W"
        elif self.team_score < self.opponent_score:
            self.result = "L"
        else:
            self.result = "T"

    @staticmethod
    def sentinel():
        fields = ["yyyymmdd", "0", "", "VIS", "VLG", "0", "HOME", "HLG", "0", "0", "0"]
        return Game(fields, "home")

    def __lt__(self, other):
        if self.date < other.date:
            return -1
        elif self.date > other.date:
            return 1
        if self.game_number < other.game_number:
            return -1
        elif self.game_number > other.game_number:
            return 1
        return 0

    def __str__(self):
        return f"{self.date} {self.team} {self.team_score}-{self.opponent_score}"


def process_gamelog(gamelog_file):
    team_games = collections.defaultdict(list)
    with open(gamelog_file, newline='') as gamelog_csv:
        gamelog = csv.reader(gamelog_csv)
        for game in gamelog:
            home = Game(game, "home")
            visitor = Game(game, "visitor")
            team_games[home.team].append(home)
            team_games[visitor.team].append(visitor)

    max_game_revenge = []
    min_lw_diff = 3
    for team, games in team_games.items():
        games.sort(key=operator.attrgetter("game_number"))
        last_game = Game.sentinel()
        for game in games:
            # if game.number_of_game == "2" and last_game.number_of_game == "1":
            if game.result == "W" and last_game.result == "L":
                if game.run_diff > last_game.run_diff > min_lw_diff:
                    max_game_revenge.append((last_game.run_diff, game.run_diff, last_game, game))
            last_game = game
    max_game_revenge.sort(reverse=True)
    return max_game_revenge


def options():
    parser = argparse.ArgumentParser()
    parser.add_argument('gamelog', nargs='+')
    args = parser.parse_args()
    return args


def main():
    args = options()
    games = []
    for gamelog in args.gamelog:
        games.extend(process_gamelog(gamelog))
        games.sort(reverse=True)
        games[GAME_LIMIT:] = []
    for i in range(min(10, len(games))):
        lost_diff, win_diff, last_game, game = games[i]
        print(f"{last_game.run_diff}:{game.run_diff}:\t{last_game}\t->\t{game}")


if __name__ == '__main__':
    main()
