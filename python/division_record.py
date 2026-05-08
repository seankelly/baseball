#!/usr/bin/env python3


import argparse
import collections
import csv
import datetime
import glob
import os.path
from dataclasses import dataclass


DIVISIONS = {
    1901: {
        'AL': [
            'BLA',
            'BOS',
            'CHA',
            'CLE',
            'DET',
            'MLA',
            'PHA',
            'WS1',
        ],
        'NL': [
            'BRO',
            'BSN',
            'CHN',
            'CIN',
            'NY1',
            'PHI',
            'PIT',
            'SLN',
        ],
    },
    1902: {
        'AL': [
            'BLA',
            'BOS',
            'CHA',
            'CLE',
            'DET',
            'PHA',
            'SLA',
            'WS1',
        ],
        'NL': [
            'BRO',
            'BSN',
            'CHN',
            'CIN',
            'NY1',
            'PHI',
            'PIT',
            'SLN',
        ],
    },
    1903: {
        'AL': [
            'BOS',
            'CHA',
            'CLE',
            'DET',
            'NYA',
            'PHA',
            'SLA',
            'WS1',
        ],
        'NL': [
            'BRO',
            'BSN',
            'CHN',
            'CIN',
            'NY1',
            'PHI',
            'PIT',
            'SLN',
        ],
    },
    1914: {
        'AL': [
            'BOS',
            'CHA',
            'CLE',
            'DET',
            'NYA',
            'PHA',
            'SLA',
            'WS1',
        ],
        'NL': [
            'BRO',
            'BSN',
            'CHN',
            'CIN',
            'NY1',
            'PHI',
            'PIT',
            'SLN',
        ],
        'FL': [
            'BLF',
            'BRF',
            'BUF',
            'CHF',
            'IND',
            'KCF',
            'PTF',
            'SLF',
        ],
    },
    1915: {
        'AL': [
            'BOS',
            'CHA',
            'CLE',
            'DET',
            'NYA',
            'PHA',
            'SLA',
            'WS1',
        ],
        'NL': [
            'BRO',
            'BSN',
            'CHN',
            'CIN',
            'NY1',
            'PHI',
            'PIT',
            'SLN',
        ],
        'FL': [
            'BLF',
            'BRF',
            'BUF',
            'CHF',
            'KCF',
            'NEW',
            'PTF',
            'SLF',
        ],
    },
    1916: {
        'AL': [
            'BOS',
            'CHA',
            'CLE',
            'DET',
            'NYA',
            'PHA',
            'SLA',
            'WS1',
        ],
        'NL': [
            'BRO',
            'BSN',
            'CHN',
            'CIN',
            'NY1',
            'PHI',
            'PIT',
            'SLN',
        ],
    },
    1953: {
        'AL': [
            'BOS',
            'CHA',
            'CLE',
            'DET',
            'NYA',
            'PHA',
            'SLA',
            'WS1',
        ],
        'NL': [
            'BRO',
            'CHN',
            'CIN',
            'MLN',
            'NY1',
            'PHI',
            'PIT',
            'SLN',
        ],
    },
    1954: {
        'AL': [
            'BAL',
            'BOS',
            'CHA',
            'CLE',
            'DET',
            'NYA',
            'PHA',
            'WS1',
        ],
        'NL': [
            'BRO',
            'CHN',
            'CIN',
            'MLN',
            'NY1',
            'PHI',
            'PIT',
            'SLN',
        ],
    },
    1955: {
        'AL': [
            'BAL',
            'BOS',
            'CHA',
            'CLE',
            'DET',
            'KC1',
            'NYA',
            'WS1',
        ],
        'NL': [
            'BRO',
            'CHN',
            'CIN',
            'MLN',
            'NY1',
            'PHI',
            'PIT',
            'SLN',
        ],
    },
    1958: {
        'AL': [
            'BAL',
            'BOS',
            'CHA',
            'CLE',
            'DET',
            'KC1',
            'NYA',
            'WS1',
        ],
        'NL': [
            'CHN',
            'CIN',
            'LAN',
            'MLN',
            'PHI',
            'PIT',
            'SFN',
            'SLN',
        ],
    },
    1961: {
        'AL': [
            'BAL',
            'BOS',
            'CHA',
            'CLE',
            'DET',
            'KC1',
            'LAA',
            'MIN',
            'NYA',
            'WS2',
        ],
        'NL': [
            'CHN',
            'CIN',
            'LAN',
            'MLN',
            'PHI',
            'PIT',
            'SFN',
            'SLN',
        ],
    },
    1962: {
        'AL': [
            'BAL',
            'BOS',
            'CHA',
            'CLE',
            'DET',
            'KC1',
            'LAA',
            'MIN',
            'NYA',
            'WS2',
        ],
        'NL': [
            'CHN',
            'CIN',
            'HOU',
            'LAN',
            'MLN',
            'NYN',
            'PHI',
            'PIT',
            'SFN',
            'SLN',
        ],
    },
    1965: {
        'AL': [
            'BAL',
            'BOS',
            'CHA',
            'CLE',
            'DET',
            'KC1',
            'CAL',
            'MIN',
            'NYA',
            'WS2',
        ],
        'NL': [
            'CHN',
            'CIN',
            'HOU',
            'LAN',
            'MLN',
            'NYN',
            'PHI',
            'PIT',
            'SFN',
            'SLN',
        ],
    },
    1966: {
        'AL': [
            'BAL',
            'BOS',
            'CHA',
            'CLE',
            'DET',
            'KC1',
            'CAL',
            'MIN',
            'NYA',
            'WS2',
        ],
        'NL': [
            'ATL',
            'CHN',
            'CIN',
            'HOU',
            'LAN',
            'NYN',
            'PHI',
            'PIT',
            'SFN',
            'SLN',
        ],
    },
    1968: {
        'AL': [
            'BAL',
            'BOS',
            'CHA',
            'CLE',
            'DET',
            'OAK',
            'CAL',
            'MIN',
            'NYA',
            'WS2',
        ],
        'NL': [
            'ATL',
            'CHN',
            'CIN',
            'HOU',
            'LAN',
            'NYN',
            'PHI',
            'PIT',
            'SFN',
            'SLN',
        ],
    },
    1969: {
        'ALE': [
            'BAL',
            'BOS',
            'CLE',
            'DET',
            'NYA',
            'WS2',
        ],
        'ALw': [
            'CAL',
            'CHA',
            'KCA',
            'MIN',
            'OAK',
            'SE1',
        ],
        'NLE': [
            'CHN',
            'MON',
            'NYN',
            'PHI',
            'PIT',
            'SLN',
        ],
        'NL': [
            'ATL',
            'CIN',
            'HOU',
            'LAN',
            'SDN',
            'SFN',
        ],
    },
    1970: {
        'ALE': [
            'BAL',
            'BOS',
            'CLE',
            'DET',
            'NYA',
            'WS2',
        ],
        'ALW': [
            'CAL',
            'CHA',
            'KCA',
            'MIL',
            'MIN',
            'OAK',
        ],
        'NLE': [
            'CHN',
            'MON',
            'NYN',
            'PHI',
            'PIT',
            'SLN',
        ],
        'NLW': [
            'ATL',
            'CIN',
            'HOU',
            'LAN',
            'SDN',
            'SFN',
        ],
    },
    1972: {
        'ALE': [
            'BAL',
            'BOS',
            'CLE',
            'DET',
            'MIL',
            'NYA',
        ],
        'ALW': [
            'CAL',
            'CHA',
            'KCA',
            'MIN',
            'OAK',
            'TEX',
        ],
        'NLE': [
            'CHN',
            'MON',
            'NYN',
            'PHI',
            'PIT',
            'SLN',
        ],
        'NLW': [
            'ATL',
            'CIN',
            'HOU',
            'LAN',
            'SDN',
            'SFN',
        ],
    },
    1977: {
        'ALE': [
            'BAL',
            'BOS',
            'CLE',
            'DET',
            'MIL',
            'NYA',
            'TOR',
        ],
        'ALW': [
            'CAL',
            'CHA',
            'KCA',
            'MIN',
            'OAK',
            'SEA',
            'TEX',
        ],
        'NLE': [
            'CHN',
            'MON',
            'NYN',
            'PHI',
            'PIT',
            'SLN',
        ],
        'NLW': [
            'ATL',
            'CIN',
            'HOU',
            'LAN',
            'SDN',
            'SFN',
        ],
    },
    1993: {
        'ALE': [
            'BAL',
            'BOS',
            'CLE',
            'DET',
            'MIL',
            'NYA',
            'TOR',
        ],
        'ALW': [
            'CAL',
            'CHA',
            'KCA',
            'MIN',
            'OAK',
            'SEA',
            'TEX',
        ],
        'NLE': [
            'CHN',
            'FLO',
            'MON',
            'NYN',
            'PHI',
            'PIT',
            'SLN',
        ],
        'NLW': [
            'ATL',
            'CIN',
            'COL',
            'HOU',
            'LAN',
            'SDN',
            'SFN',
        ],
    },
    1994: {
        'ALE': [
            'BAL',
            'BOS',
            'DET',
            'NYA',
            'TOR',
        ],
        'ALC': [
            'CHA',
            'CLE',
            'KCA',
            'MIL',
            'MIN',
        ],
        'ALW': [
            'CAL',
            'OAK',
            'SEA',
            'TEX',
        ],
        'NLE': [
            'ATL',
            'FLO',
            'MON',
            'NYN',
            'PHI',
        ],
        'NLC': [
            'CHN',
            'CIN',
            'HOU',
            'PIT',
            'SLN',
        ],
        'NLW': [
            'COL',
            'LAN',
            'SDN',
            'SFN',
        ],
    },
    1997: {
        'ALE': [
            'BAL',
            'BOS',
            'DET',
            'NYA',
            'TOR',
        ],
        'ALC': [
            'CHA',
            'CLE',
            'KCA',
            'MIL',
            'MIN',
        ],
        'ALW': [
            'ANA',
            'OAK',
            'SEA',
            'TEX',
        ],
        'NLE': [
            'ATL',
            'FLO',
            'MON',
            'NYN',
            'PHI',
        ],
        'NLC': [
            'CHN',
            'CIN',
            'HOU',
            'PIT',
            'SLN',
        ],
        'NLW': [
            'COL',
            'LAN',
            'SDN',
            'SFN',
        ],
    },
    1998: {
        'ALE': [
            'BAL',
            'BOS',
            'NYA',
            'TBA',
            'TOR',
        ],
        'ALC': [
            'CHA',
            'CLE',
            'DET',
            'KCA',
            'MIN',
        ],
        'ALW': [
            'ANA',
            'OAK',
            'SEA',
            'TEX',
        ],
        'NLE': [
            'ATL',
            'FLO',
            'MON',
            'NYN',
            'PHI',
        ],
        'NLC': [
            'CHN',
            'CIN',
            'HOU',
            'MIL',
            'PIT',
            'SLN',
        ],
        'NLW': [
            'ARI',
            'COL',
            'LAN',
            'SDN',
            'SFN',
        ],
    },
    2005: {
        'ALE': [
            'BAL',
            'BOS',
            'NYA',
            'TBA',
            'TOR',
        ],
        'ALC': [
            'CHA',
            'CLE',
            'DET',
            'KCA',
            'MIN',
        ],
        'ALW': [
            'ANA',
            'OAK',
            'SEA',
            'TEX',
        ],
        'NLE': [
            'ATL',
            'FLO',
            'NYN',
            'PHI',
            'WAS',
        ],
        'NLC': [
            'CHN',
            'CIN',
            'HOU',
            'MIL',
            'PIT',
            'SLN',
        ],
        'NLW': [
            'ARI',
            'COL',
            'LAN',
            'SDN',
            'SFN',
        ],
    },
    2012: {
        'ALE': [
            'BAL',
            'BOS',
            'NYA',
            'TBA',
            'TOR',
        ],
        'ALC': [
            'CHA',
            'CLE',
            'DET',
            'KCA',
            'MIN',
        ],
        'ALW': [
            'ANA',
            'OAK',
            'SEA',
            'TEX',
        ],
        'NLE': [
            'ATL',
            'MIA',
            'NYN',
            'PHI',
            'WAS',
        ],
        'NLC': [
            'CHN',
            'CIN',
            'HOU',
            'MIL',
            'PIT',
            'SLN',
        ],
        'NLW': [
            'ARI',
            'COL',
            'LAN',
            'SDN',
            'SFN',
        ],
    },
    2013: {
        'ALE': [
            'BAL',
            'BOS',
            'NYA',
            'TBA',
            'TOR',
        ],
        'ALC': [
            'CHA',
            'CLE',
            'DET',
            'KCA',
            'MIN',
        ],
        'ALW': [
            'ANA',
            'HOU',
            'OAK',
            'SEA',
            'TEX',
        ],
        'NLE': [
            'ATL',
            'MIA',
            'NYN',
            'PHI',
            'WAS',
        ],
        'NLC': [
            'CHN',
            'CIN',
            'MIL',
            'PIT',
            'SLN',
        ],
        'NLW': [
            'ARI',
            'COL',
            'LAN',
            'SDN',
            'SFN',
        ],
    },
    2025: {
        'ALE': [
            'BAL',
            'BOS',
            'NYA',
            'TBA',
            'TOR',
        ],
        'ALC': [
            'CHA',
            'CLE',
            'DET',
            'KCA',
            'MIN',
        ],
        'ALW': [
            'ANA',
            'HOU',
            'ATH',
            'SEA',
            'TEX',
        ],
        'NLE': [
            'ATL',
            'MIA',
            'NYN',
            'PHI',
            'WAS',
        ],
        'NLC': [
            'CHN',
            'CIN',
            'MIL',
            'PIT',
            'SLN',
        ],
        'NLW': [
            'ARI',
            'COL',
            'LAN',
            'SDN',
            'SFN',
        ],
    },
}


class Divisions:

    def __init__(self, division_history):
        self._divisions = {}
        self._expand_divisions(division_history)

    def _expand_divisions(self, division_history):
        starting_years = sorted(division_history.keys())
        previous_year = starting_years[0]
        for year in starting_years[1:]:
            # update_years = list(range(previous_year, year))
            for season in range(previous_year, year):
                self._divisions[str(season)] = division_history[previous_year]
            previous_year = year

        # Cover last change to today.
        today = datetime.date.today()
        for season in range(previous_year, today.year):
            self._divisions[str(season)] = division_history[year]

    def division(self, year):
        return self._divisions.get(year)


@dataclass
class Schedule:
    date: str
    visitor: str
    home: str


@dataclass
class Team:
    team: str
    wins: int = 0
    losses: int = 0
    ties: int = 0


def load_schedule(schedule_path):
    schedule = []
    with open(schedule_path, newline='') as schedule_csv:
        reader = csv.reader(schedule_csv)
        for game in reader:
            date = game[0]
            visitor = game[3]
            home = game[6]
            makeup = game[-1]
            if makeup:
                if makeup.isnumeric():
                    date = makeup
                else:
                    fields = makeup.split()
                    if fields[-1].isnumeric():
                        date = fields[-1]
            if not date.isnumeric():
                continue
            schedule.append(Schedule(date, visitor, home))
    return schedule


def load_gamelog(gamelog_path):
    games = []
    with open(gamelog_path, newline='') as gamelog:
        reader = csv.reader(gamelog)
        for game in reader:
            date = game[0]
            visitor = game[3]
            visitor_score = int(game[9])
            home = game[6]
            home_score = int(game[10])
            games.append((date, visitor, visitor_score, home, home_score))
    games.sort()
    return games


# Check if every team in the division has the same record.
def check_divisions(divisions):
    matches = []
    for division_name, division_teams in divisions.items():
        records = set()
        for team in division_teams.values():
            records.add((team.wins, team.losses))
        # Check if there's only one record from all the teams and it's not the
        # obvious "no one has played".
        if len(records) == 1 and (0, 0) not in records:
            record = records.pop()
            matches.append((division_name, record))
    return matches


# Check if one league has X best records overall (?).
def check_league1(divisions):
    check_teams = 5
    standings = []
    for division_name, division_teams in divisions.items():
        league = division_name[:2]
        for team in division_teams.values():
            if team.wins + team.losses > 0:
                win_percentage = team.wins / (team.wins + team.losses)
            else:
                win_percentage = 0.0
            standings.append((win_percentage, league, team.team))
    standings.sort(reverse=True)
    # Verify all X teams have actually played games.
    if standings[check_teams - 1][0] == 0.0:
        return False
    league_matches = set()
    for idx in range(check_teams):
        league_matches.add(standings[idx][1])
    # Check if there's only one league in the matches to indicate all X teams
    # have the same league.
    if len(league_matches) == 1:
        return league_matches.pop()
    return False


# Check number of winning records in a league compared to the other league.
# Goal is to check for seasons with unusually small/large number of teams with
# a winning record.
# NOTE: Returns a list that is expected to be sorted by caller.
def check_league_winning_records(divisions):
    # check_winning_percentage = 0.5
    # check_winning_total = 1

    winning_teams = collections.Counter()
    total_teams = collections.Counter()
    standings = []
    for division_name, division_teams in divisions.items():
        league = division_name[:2]
        for team in division_teams.values():
            total_teams[league] += 1
            if team.wins + team.losses > 0:
                win_percentage = team.wins / (team.wins + team.losses)
            else:
                # If any team hasn't played a game yet on this date, skip
                # further consideration. We only want days where every team has
                # played at least once.
                return
            if win_percentage > 0.5:
                winning_teams[league] += 1
    for league, number_winning in winning_teams.items():
        winning_team_percentage = number_winning / total_teams[league]
        standings.append((winning_team_percentage, league))
    return standings


def process_season(div, gamelog_path):
    teams = {}
    divisions = {}
    for division_name, division_teams in div.items():
        divisions[division_name] = {}
        for team_name in division_teams:
            team = Team(team_name)
            teams[team_name] = team
            divisions[division_name][team_name] = team

    games = load_gamelog(gamelog_path)
    days = sorted({g[0] for g in games})

    game_idx = 0
    # Only for check_league_winning_records.
    standings = []
    for day in days:
        if game_idx >= len(games):
            break
        try:
            game = games[game_idx]
        except IndexError:
            print(f"{day=} {game_idx=}")
            raise
        while game[0] == day:
            visitor_team = teams.get(game[1])
            visitor_score = game[2]
            home_team = teams.get(game[3])
            home_score = game[4]

            if visitor_score > home_score:
                visitor_team.wins += 1
                home_team.losses += 1
            elif home_score > visitor_score:
                visitor_team.losses += 1
                home_team.wins += 1
            else:
                visitor_team.ties += 1
                home_team.ties += 1

            game_idx += 1
            if game_idx >= len(games):
                break
            game = games[game_idx]

        # Add the day to each league's percentage so it can be tracked.
        days_standings = check_league_winning_records(divisions)
        if not days_standings:
            continue
        for record in days_standings:
            record = list(record)
            record.append(day)
            standings.append(tuple(record))

        # For other check_* functions. See git history.
        # matches = check_league_winning_records(divisions)
        # if matches:
        #     print(f"Found date matching condition on {day}: {matches}")
    standings.sort()
    return standings


def find_gamelog(yeardir):
    for gl_glob in ('gl*.txt', 'GL*.TXT'):
        pattern = os.path.join(yeardir, gl_glob)
        gamelogs = glob.glob(pattern)
        if gamelogs:
            return gamelogs[0]


def options():
    parser = argparse.ArgumentParser()
    parser.add_argument('--limit', '-l', type=int, default=50)
    parser.add_argument('retrosheet_dir')
    parser.add_argument('year', nargs='+')
    args = parser.parse_args()

    return args


def main():
    args = options()

    divisions = Divisions(DIVISIONS)

    retrosheet_dir = args.retrosheet_dir
    standings = []
    for year in args.year:
        print(f"Checking {year}")
        season_dir = os.path.join(retrosheet_dir, year)
        div = divisions.division(year)
        gamelog_path = find_gamelog(season_dir)
        standings.extend(process_season(div, gamelog_path))
        standings.sort()
    for idx in range(min(args.limit, len(standings))):
        print(standings[idx])


if __name__ == '__main__':
    main()
