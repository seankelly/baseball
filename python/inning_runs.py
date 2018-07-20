#!/usr/bin/env python3

import argparse
import csv


HOME_TEAM = 6
GAME_DATE = 0
GAME_NUMBER = 1
VISITOR_LINE_SCORE = 19
HOME_LINE_SCORE = 20


class Maximum:

    def __init__(self, whoami):
        self.whoami = whoami
        self.maximum = 0
        self.games = []

    def __str__(self):
        stringify = ("%s: %d\n"
                     "Number of games: %d\n"
                     "Games: %s\n") % (
            self.whoami, self.maximum, len(self.games), ' '.join(self.games))
        return stringify

    def evaluate_game(self, game_id, number_innings):
        if number_innings > self.maximum:
            self.maximum = number_innings
            self.games = [game_id]
        elif number_innings == self.maximum and number_innings > 0:
            self.games.append(game_id)


def parse_line_score(line_score):
    """
    Parse a single line score from a game.
    """
    innings = []
    tenplus_runs = False
    runs = 0
    for inning in line_score:
        if inning.isdigit():
            if not tenplus_runs:
                innings.append(int(inning))
            else:
                runs = runs * 10 + int(inning)
        elif inning == '(':
            tenplus_runs = True
        elif inning == ')':
            innings.append(runs)
            tenplus_runs = False
            runs = 0
        elif inning == 'x':
            pass
        else:
            raise ValueError(f"Unknown character in line score: {inning}")
    return innings


def count_innings(line_score, minimum_runs):
    count = 0
    for inning in line_score:
        if inning >= minimum_runs:
            count += 1
    return count


def main():
    parser = argparse.ArgumentParser(
        description="Analyze number of runs scored in an inning")
    parser.add_argument('--minimum-runs', type=int, metavar='RUNS', default=4,
                        help="Minimum number of runs scored in an inning")
    parser.add_argument('gamelogs', nargs='+',
                        help="Game log files")
    args = parser.parse_args()

    maximum_combined_8 = Maximum('Both teams (8 innings)')
    maximum_one_team_8 = Maximum('One team (8 innings)')
    maximum_combined_9 = Maximum('Both teams (9 innings)')
    maximum_one_team_9 = Maximum('One team (9 innings)')
    maximum_combined_10 = Maximum('Both teams (extra innings)')
    maximum_one_team_10 = Maximum('One team (extra innings)')
    for gamelog_file in args.gamelogs:
        with open(gamelog_file, newline='') as gamelog_csv:
            gamelog = csv.reader(gamelog_csv)
            for game in gamelog:
                game_id = '%s%s%s' % (
                    game[HOME_TEAM], game[GAME_DATE], game[GAME_NUMBER])
                combined = 0
                home_line_score = parse_line_score(game[HOME_LINE_SCORE])
                visitor_line_score = parse_line_score(game[VISITOR_LINE_SCORE])
                home = count_innings(home_line_score, args.minimum_runs)
                visitor = count_innings(visitor_line_score, args.minimum_runs)
                combined = home + visitor
                innings = len(visitor_line_score)

                if innings == 9:
                    maximum_one_team_9.evaluate_game(game_id, home)
                    maximum_one_team_9.evaluate_game(game_id, visitor)
                    maximum_combined_9.evaluate_game(game_id, combined)
                elif innings > 9:
                    maximum_one_team_10.evaluate_game(game_id, home)
                    maximum_one_team_10.evaluate_game(game_id, visitor)
                    maximum_combined_10.evaluate_game(game_id, combined)
                else:
                    maximum_one_team_8.evaluate_game(game_id, home)
                    maximum_one_team_8.evaluate_game(game_id, visitor)
                    maximum_combined_8.evaluate_game(game_id, combined)

    print(f"Most {args.minimum_runs} run or more innings")
    print(str(maximum_one_team_9))
    print(str(maximum_combined_9))
    print(str(maximum_one_team_8))
    print(str(maximum_combined_8))
    print(str(maximum_one_team_10))
    print(str(maximum_combined_10))


if __name__ == '__main__':
    main()
