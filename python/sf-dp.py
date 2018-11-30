#!/usr/bin/env python3

import argparse
import csv


def find_plays(eventfile):
    games = set()
    with open(eventfile, 'r') as event_fd:
        events = csv.reader(event_fd)
        active_game = None
        for event in events:
            if not event:
                continue
            elif event[0] == 'id':
                active_game = event[1]
                continue
            elif event[0] != 'play':
                continue
            plays = event[6]
            play_parts = plays.split('.')
            play_events = play_parts[0].split('/')
            if 'SF' in play_events and 'DP' in play_events:
                for play_event in play_events:
                    if play_event.startswith('E'):
                        # Found a sacrifice fly with an error and a double play.
                        games.add(active_game)
    return games


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('eventfiles', nargs='+')
    args = parser.parse_args()

    games = set()
    for eventfile in args.eventfiles:
        games.update(find_plays(eventfile))

    if games:
        print("Found these games: %s" % ', '.join(sorted(games)))
    else:
        print("Did not find any games")


if __name__ == '__main__':
    main()
