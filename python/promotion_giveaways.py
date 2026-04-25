#!/usr/bin/env python3

import argparse
import csv
import time
from dataclasses import dataclass
from datetime import datetime
from zoneinfo import ZoneInfo

import requests


SEASON_START = "{year}-03-15"
SEASON_END = "{year}-10-03"
SCHEDULE_API = 'https://statsapi.mlb.com/api/v1/schedule?lang=en&sportId=1&hydrate=team,game(promotions,sponsorships)&season={year}&startDate={start_date}&endDate={end_date}&teamId={team}&eventTypes=primary&scheduleTypes=games'

TEAM_IDS = {
    "ARI": 109,
    "ATH": 133,
    "ATL": 144,
    "BAL": 110,
    "BOS": 111,
    "CHC": 112,
    "CHW": 145,
    "CIN": 113,
    "CLE": 114,
    "COL": 115,
    "DET": 116,
    "HOU": 117,
    "KCR": 118,
    "LAA": 108,
    "LOS": 119,
    "MIA": 146,
    "MIL": 158,
    "MIN": 142,
    "NYM": 121,
    "NYY": 147,
    "PHI": 143,
    "PIT": 134,
    "SDP": 135,
    "SEA": 136,
    "SFG": 137,
    "STL": 138,
    "TBR": 139,
    "TEX": 140,
    "TOR": 141,
    "WAS": 120,
}


@dataclass(order=True)
class Game:
    date: str
    home: str
    away: str
    game_pk: int
    promotions: list

    @property
    def giveaway(self):
        has_giveaway = False
        for promotion in self.promotions:
            if promotion["offerType"] == "Giveaway":
                has_giveaway = True
        return has_giveaway

    def fields(self):
        promotions = [f"{promo["name"]}." for promo in self.promotions]
        promos = "  ".join(promotions)
        f = [self.date, self.home, self.away, promos, self.giveaway]
        return f


def fetch_schedule(team_id, season):
    options = {
        "team": team_id,
        "start_date": SEASON_START.format(year=season),
        "end_date": SEASON_END.format(year=season),
        "year": season,
    }
    schedule_url = SCHEDULE_API.format(**options)
    # print(f"{schedule_url=}")
    res = requests.get(schedule_url)
    res.raise_for_status()
    stats_sched = res.json()
    return stats_sched


def extract_team(team_dict) -> Dict:
    team_info = {
        "id": team_dict["team"]["id"],
        "name": team_dict["team"]["name"],
        "abbreviation": team_dict["team"]["abbreviation"],
    }
    return team_info


def process_schedule(team_id, schedule):
    games = []
    for date in schedule["dates"]:
        for game in date["games"]:
            game_type = game["gameType"]
            game_pk = game["gamePk"]
            # Only want regular season games.
            if game_type != "R":
                continue
            status = game["status"].get("codedGameState")
            # Only count games that finished, in preview, in progress, or scheduled.
            if status not in ("F", "P", "I", "S"):
                continue
            # Skip suspended games because they will show up as final on the original date.
            if "resumedFrom" in game:
                continue
            official_date = game["officialDate"]
            home = extract_team(game["teams"]["home"])
            away = extract_team(game["teams"]["away"])
            # Because checking every team, only process the home games for this team.
            if home["id"] != team_id:
                continue

            promotions = []
            for promotion in game.get("promotions", []):
                promo_info = {
                    "name": promotion.get("name", "Unknown name"),
                    "offerType": promotion.get("offerType"),
                }
                promotions.append(promo_info)
            game = Game(official_date, home["abbreviation"], away["abbreviation"], game_pk, promotions)
            games.append(game)
    games.sort()
    return games


def options():
    parser = argparse.ArgumentParser()
    parser.add_argument("season", help="Season to check giveaways")
    parser.add_argument("data", help="Save raw CSV data")
    args = parser.parse_args()
    return args


def main():
    args = options()

    games = []
    print("Checking team schedules: ", end="", flush=True)
    for team, team_id in TEAM_IDS.items():
        print(f"{team}.. ", end="", flush=True)
        team_schedule = fetch_schedule(team_id, args.season)
        games.extend(process_schedule(team_id, team_schedule))
        time.sleep(0.5)
    print()

    with open(args.data, "w", newline="") as raw_data:
        data = csv.writer(raw_data, lineterminator="\n")
        for game in games:
            data.writerow(game.fields())


if __name__ == '__main__':
    main()
