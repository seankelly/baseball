#!/usr/bin/env python3

import argparse
import collections
import csv
import multiprocessing
import operator
import os


OUTPUT_LIMIT = 100


SeasonPalindrome = collections.namedtuple(
        'SeasonPalindrome', ('season', 'team', 'palindrome', 'length', 'game_start', 'game_end'))


def process_gamelog(gamelog_path, include_teams=None):
    season_results = parse_gamelog(gamelog_path)
    season, teams = season_results

    palindromes = []
    for team, games in teams.items():
        if include_teams:
            if team not in include_teams:
                continue
        game_start, game_end, game_palindrome = find_longest_palindrome(games)
        # Strings are zero-indexed but games are one-indexed. Increment by one
        # to return the correct game start as a schedule would show.
        game_start += 1
        # Calculate length so it's easier to sort.
        length = len(game_palindrome)
        palindromes.append(SeasonPalindrome(season, team, game_palindrome,
                                            length, game_start, game_end))
    return palindromes


def parse_gamelog(gamelog_path):
    season = ""
    expanded_games = {}

    def add_team_game(games, team, number, score, other_score):
        number = int(number)
        score = int(score)
        other_score = int(other_score)
        if score > other_score:
            result = 'W'
        elif score < other_score:
            result = 'L'
        else:
            result = 'T'
        if team not in games:
            games[team] = []
        games[team].append((number, result))

    with open(gamelog_path) as gamelog:
        gamelog_reader = csv.reader(gamelog)
        for game in gamelog_reader:
            if not season:
                season = game[0][0:4]
            # Visiting team.
            add_team_game(expanded_games, game[3], game[5], game[9], game[10])
            # Home team.
            add_team_game(expanded_games, game[6], game[8], game[10], game[9])

    games = {}
    for team, results in expanded_games.items():
        ordered_games = sorted(results, key=operator.itemgetter(0))
        compact_games = ''.join(game[1] for game in ordered_games)
        games[team] = compact_games

    return season, games


# Find the longest palindrome using Manacher's algorithm.
def find_longest_palindrome(string, padding_character=' '):
    padded_string = padding_character + padding_character.join(string) + padding_character
    palindrome_radii = [0] * len(padded_string)
    center = 0
    radius = 0
    while center < len(padded_string):
        start = center - (radius + 1) 
        end = center + (radius + 1)
        while start >= 0 and end < len(padded_string) and padded_string[start] == padded_string[end]:
            start -= 1
            end += 1
            radius += 1

        palindrome_radii[center] = radius
        old_center = center
        old_radius = radius

        center += 1
        radius = 0
        while center <= (old_center + old_radius):
            mirrored_center = old_center - (center - old_center)
            max_mirrored_radius = old_center + old_radius - center
            if palindrome_radii[mirrored_center] < max_mirrored_radius:
                palindrome_radii[center] = palindrome_radii[mirrored_center]
                center += 1
            elif palindrome_radii[center] > max_mirrored_radius:
                palindrome_radii[center] = max_mirrored_radius
                center += 1
            else:
                radius = max_mirrored_radius
                break

    max_length = 0
    max_index = 0
    for idx, length in enumerate(palindrome_radii):
        if length > max_length:
            max_length = length
            max_index = idx

    palindrome = ''
    true_start = (max_index // 2) - (max_length // 2)
    true_end = true_start + max_length
    palindrome = string[true_start:true_end]
    return true_start, true_end, palindrome


def csv_output(output_path, palindromes, minimum_length):
    fields = ['year', 'team', 'length', 'palindrome', 'game_start', 'game_end', 'wins', 'losses',
              'ties']
    with open(output_path, 'w', newline='') as csvfile:
        writer = csv.DictWriter(csvfile, fieldnames=fields)
        writer.writeheader()
        for palindrome in palindromes:
            if palindrome.length < minimum_length:
                break
            wins = palindrome.palindrome.count('W')
            losses = palindrome.palindrome.count('L')
            ties = palindrome.palindrome.count('T')
            row = {
                'year': palindrome.season,
                'team': palindrome.team,
                'length': palindrome.length,
                'palindrome': palindrome.palindrome,
                'game_start': palindrome.game_start,
                'game_end': palindrome.game_end,
                'wins': wins,
                'losses': losses,
                'ties': ties,
            }
            writer.writerow(row)


def options():
    parser = argparse.ArgumentParser()
    parser.add_argument('--csv', metavar='FILE', help="Output CSV-formatted data")
    parser.add_argument('--team', action='append', help="Team(s) to find game palindromes")
    parser.add_argument('--limit', default=OUTPUT_LIMIT, type=int,
                        help="Number of palindromes to display (may go over because of ties)")
    parser.add_argument('gamelog', nargs='+')
    args = parser.parse_args()
    return args


def main():
    args = options()
    include_teams = set()
    if args.team:
        include_teams.update(args.team)
    palindromes = []
    available_cpus = len(os.sched_getaffinity(0))
    with multiprocessing.Pool(available_cpus) as pool:
        for team_palindromes in pool.imap_unordered(process_gamelog, args.gamelog):
            if not include_teams:
                palindromes.extend(team_palindromes)
            else:
                for palindrome in team_palindromes:
                    if palindrome.team in include_teams:
                        palindromes.append(palindrome)

    # Python sort is stable so sort by teams to get them in order within the
    # same season, then by season to go from first instance of this length to
    # most recent, and finally by palindrome length to go longest to shortest.
    palindromes.sort(key=operator.itemgetter(1))
    palindromes.sort(key=operator.itemgetter(0))
    palindromes.sort(key=operator.itemgetter(3), reverse=True)
    if args.limit <= len(palindromes):
        last_palindrome = palindromes[args.limit - 1]
    else:
        last_palindrome = palindromes[-1]
    minimum_length = last_palindrome.length

    if not args.csv:
        for palindrome in palindromes:
            if palindrome.length < minimum_length:
                break
            wins = palindrome.palindrome.count('W')
            losses = palindrome.palindrome.count('L')
            ties = palindrome.palindrome.count('T')
            game_range = f"{palindrome.game_start}-{palindrome.game_end}"
            print(f"{palindrome.season}: {palindrome.team}: {palindrome.length}"
                f" ({game_range: >7}): {palindrome.palindrome} ({wins}-{losses}-{ties})")
    else:
        csv_output(args.csv, palindromes, minimum_length)


if __name__ == '__main__':
    main()
