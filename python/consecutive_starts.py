#!/usr/bin/env python3

import argparse
import collections
import csv
from dataclasses import dataclass


MAX_STREAKS = 10


@dataclass
class StartingStreak:
    season: str
    playerid: str
    start: int
    end: int

    def __len__(self):
        return self.end - self.start + 1


#  4-5     Visiting team and league
#    6     Visiting team game number
#          For this and the home team game number, ties are counted as
#          games and suspended games are counted from the starting
#          rather than the ending date.
#  7-8     Home team and league
#    9     Home team game number
# 106-132   Visiting starting players ID, name and defensive position,
#           listed in the order (1-9) they appeared in the batting order.
# 133-159   Home starting players ID, name and defensive position
#           listed in the order (1-9) they appeared in the batting order.
def parse_gamelog(gamelog_path):
    season = ""
    player_games = collections.defaultdict(lambda: collections.defaultdict(lambda: collections.defaultdict(list)))

    with open(gamelog_path) as gamelog:
        gamelog_reader = csv.reader(gamelog)
        for game in gamelog_reader:
            if not season:
                season = game[0][0:4]
            visiting_game_number = int(game[5])
            teamid = game[3]
            for idx in range(105, 131, 3):
                playerid = game[idx]
                if playerid == "" or game[idx+2] == "":
                    continue
                position = int(game[idx+2])
                player_games[playerid][teamid][position].append(visiting_game_number)
                
            home_game_number = int(game[8])
            teamid = game[6]
            for idx in range(132, 158, 3):
                playerid = game[idx]
                if playerid == "" or game[idx+2] == "":
                    continue
                position = int(game[idx+2])
                player_games[playerid][teamid][position].append(home_game_number)

    return season, player_games


def order_games(season, playerid, player_season):
    streak = {}
    for team, positions in player_season.items():
        for position, games in positions.items():
            last_game = -1
            if position not in streak:
                streak[position] = StartingStreak(season, playerid, 0, 0)
            start_game, end_game = 0, 0
            for game in games:
                if game == (last_game + 1):
                    end_game = game
                else:
                    if end_game > start_game:
                        if (end_game - start_game + 1) > len(streak[position]):
                            streak[position] = StartingStreak(season, playerid, start_game, end_game)
                    start_game = game
                    end_game = game
                last_game = game
            if end_game > start_game:
                if (end_game - start_game + 1) > len(streak[position]):
                    streak[position] = StartingStreak(season, playerid, start_game, end_game)
    return streak


def prune(streaks):
    streaks.sort(key=len, reverse=True)
    last_position = streaks[MAX_STREAKS-1]
    for idx in range(len(streaks) - 1, MAX_STREAKS, -1):
        if len(streaks[idx]) < len(last_position):
            streaks.pop(idx)


def options():
    parser = argparse.ArgumentParser()
    parser.add_argument('--team', action='append', help="Team(s) to find game palindromes")
    parser.add_argument('--position', type=int, help="Limit to this position")
    parser.add_argument('--count', type=int, help="Include at least this many in the leaderboard")
    parser.add_argument('gamelog', nargs='+')
    args = parser.parse_args()
    return args


def main():
    args = options()

    include_teams = set()
    if args.team:
        include_teams.update(args.team)

    if args.count:
        global MAX_STREAKS
        MAX_STREAKS = args.count

    max_streak = {}
    if args.position:
        max_streak[args.position] = []
    else:
        for position in range(1, 11):
            max_streak[position] = []

    for gamelog in args.gamelog:
        season, player_games = parse_gamelog(gamelog)
        for playerid, player_season in player_games.items():
            streak = order_games(season, playerid, player_season)
            for position, games in streak.items():
                length = len(games)
                if length == 1 or position not in max_streak:
                    continue
                if len(max_streak[position]) < MAX_STREAKS:
                    max_streak[position].append(games)
                elif length >= len(max_streak[position][-1]):
                    max_streak[position].append(games)
                if len(max_streak[position]) > MAX_STREAKS:
                    prune(max_streak[position])

    for position, streaks in max_streak.items():
        print(f"Position {position}:")
        for streak in streaks:
            print(f"  {len(streak)}: {streak}")


if __name__ == '__main__':
    main()
