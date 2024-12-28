#!/usr/bin/env python3

import argparse
import csv
from collections import Counter, defaultdict, namedtuple


Slashline = namedtuple('Slashline', ['year', 'pa', 'avg', 'obp', 'slg', 'ops'])


def calculate_season_slash_line(season):
    # 0        1      2     3      4    5 6         7 8 9  10 11 12  13 14 15 16 17  18  19 20
    # playerID,yearID,stint,teamID,lgID,G,G_batting,AB,R,H,2B,3B,HR,RBI,SB,CS,BB,SO,IBB,HBP,SH,SF,
    #   GIDP,G_old
    year = int(season[1])
    if year in (1981, 1994, 1995, 2020):
        return
    if not season[5]:
        return
    if not season[7] or season[7] == '0':
        return
    ab = int(season[7])
    hits = int(season[9])
    h2 = int(season[10])
    h3 = int(season[11])
    hr = int(season[12])
    h1 = hits - h2 - h3 - hr
    bb = int(season[16])
    if season[19]:
        hbp = int(season[19])
    else:
        hbp = 0
    if season[20]:
        sh = int(season[20])
    else:
        sh = 0
    if season[21]:
        sf = int(season[21])
    else:
        sf = 0
    pa = ab + bb + hbp + sf + sh
    obp_denom = ab + bb + hbp + sf

    avg = hits / ab
    obp = (hits + bb + hbp) / obp_denom
    slg = (h1 + 2*h2 + 3*h3 + 4*hr) / ab
    ops = obp + slg
    ops3 = f"{ops:.3f}"

    slash_line = Slashline(year, pa, f'{avg:.3f}', f'{obp:.3f}', f'{slg:.3f}', ops3)
    return slash_line


def parse_batting_csv(batting_csv_path):
    batting_season = []
    player_slash_line = defaultdict(list)
    slash_line_counter = Counter()
    with open(batting_csv_path) as batting_csv:
        batting = csv.reader(batting_csv)
        for season in batting:
            if season[0] == 'playerID':
                continue
            slash_line = calculate_season_slash_line(season)
            if not slash_line:
                continue
            qualified = ((slash_line.year >= 1961 and slash_line.pa >= 502)
                         or (1900 <= slash_line.year < 1961 and slash_line.pa >= 475))
            if qualified:
                avg_obp_slg = slash_line.avg, slash_line.obp, slash_line.slg
                slash_line_counter[avg_obp_slg] += 1
                player_slash_line[avg_obp_slg].append((season[0], slash_line.year))
    print(f"Unique slash_line: {len(slash_line_counter)}")
    for avg_obp_slg, occurences in slash_line_counter.most_common(100):
        if occurences == 1:
            break
        print(f"{avg_obp_slg} ({occurences}): {player_slash_line[avg_obp_slg]}")


def options():
    parser = argparse.ArgumentParser()
    parser.add_argument('batting_csv')
    args = parser.parse_args()

    parse_batting_csv(args.batting_csv)


def main():
    args = options()


if __name__ == '__main__':
    main()
