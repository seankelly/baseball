#!/usr/bin/env python3

import argparse
import csv
import dataclasses
import glob
import os.path
import subprocess


# State 0 is bases empty, 1 is first base occupied, 2 is second base occupied,
# and 4 is third base occupied.
RE24_BASE_STATES = tuple(range(8))
RE24_OUT_STATES = (0, 1, 2)

# Ignore innings after the eighth because tactics change.
INNING_LIMIT = 8


@dataclasses.dataclass
class BaseOutState:
    bases: int
    outs: int
    runs: int = 0
    times: int = 0
    scoreless: int = 0


class Re24:

    def __init__(self):
        self.states = {}
        self._build_states()

    def _build_states(self):
        for base in RE24_BASE_STATES:
            for out in RE24_OUT_STATES:
                self.states[(base, out)] = BaseOutState(base, out)

    def increment_state(self, bases, outs, runs, times=1):
        key = (bases, outs)
        state = self.states.get(key)
        if state:
            state.runs += runs
            state.times += times

    def consume_states(self, states):
        for state in states:
            bases, outs = state.bases, state.outs
            internal_state = self.states[(bases, outs)]
            internal_state.runs += state.runs
            internal_state.times += 1
            if state.runs == 0:
                internal_state.scoreless += 1

    def consume_re24(self, re24):
        for key, state in self.states.items():
            other_state = re24.states[key]
            state.runs += other_state.runs
            state.times += other_state.times
            state.scoreless += other_state.scoreless

    def table(self):
        s = ["1B 2B 3B  0 Outs 1 Outs 2 Outs"]
        for bases in RE24_BASE_STATES:
            line = self._base_str(bases)
            for outs in RE24_OUT_STATES:
                state = self.states[(bases, outs)]
                if state.times > 0:
                    avg_scored = state.runs / state.times
                else:
                    avg_scored = state.runs
                col = f"  {avg_scored:0.3f}"
                line += col
            s.append(line)
        return "\n".join(s)

    def scored_table(self):
        s = ["1B 2B 3B  0 Outs 1 Outs 2 Outs"]
        for bases in RE24_BASE_STATES:
            line = self._base_str(bases)
            for outs in RE24_OUT_STATES:
                state = self.states[(bases, outs)]
                if state.times > 0:
                    scored = (state.times - state.scoreless) / state.times
                else:
                    scored = -1.0
                col = f"  {scored:0.3f}"
                line += col
            s.append(line)
        return "\n".join(s)

    @staticmethod
    def _base_str(base):
        match base:
            case 0:
                return "-- -- --"
            case 1:
                return "1B -- --"
            case 2:
                return "-- 2B --"
            case 3:
                return "1B 2B --"
            case 4:
                return "-- -- 3B"
            case 5:
                return "1B -- 3B"
            case 6:
                return "-- 2B 3B"
            case 7:
                return "1B 2B 3B"
            case _:
                return "-- -- --"


def process_year(retrosheet_dir, season):
    re24 = Re24()
    raw_events = cwevent(retrosheet_dir, season)
    events = csv.reader(raw_events)
    completed_states = []
    states = {'0': [], '1': []}
    last_gameid = ""
    for event in events:
        gameid, inning, outs, half_inning, half_inning_ends, base_state_start, runs = event
        # If the game has changed, clear everything to account for innings suddenly ending.
        if gameid != last_gameid:
            for half in states.values():
                half.clear()
        inning = int(inning)
        if inning > INNING_LIMIT:
            continue
        outs = int(outs)
        base_state_start = int(base_state_start)
        runs = int(runs)
        # If any runs scored on the play, then all previous states of this
        # half-inning need to update with those new runs.
        if runs > 0:
            for state in states[half_inning]:
                state.runs += runs
        new_state = BaseOutState(base_state_start, outs, runs)
        states[half_inning].append(new_state)
        if half_inning_ends == 'T':
            # Check when it's the bottom half of the inning completing.
            if half_inning == '1':
                for half in states.values():
                    re24.consume_states(half)
                    half.clear()
        last_gameid = gameid
    return re24


def cwevent(season_base_dir, season):
    season_dir = os.path.join(season_base_dir, season)
    event_files = glob.glob(f'{season}???.EV[AN]', root_dir=season_dir)
    cmd = ['cwevent', '-q', '-y', season, '-f', '0,2,4', '-x', '3,5,13,45'] + event_files
    proc = subprocess.run(cmd, check=True, cwd=season_dir, encoding='utf-8',
                          stdout=subprocess.PIPE)
    stdout = proc.stdout
    return stdout.splitlines()


def options():
    parser = argparse.ArgumentParser()
    parser.add_argument('season_dir')
    parser.add_argument('season', nargs='+')
    args = parser.parse_args()
    return args


def main():
    args = options()
    states = {}
    re24 = Re24()
    for season in args.season:
        seasonal_re24 = process_year(args.season_dir, season)
        re24.consume_re24(seasonal_re24)
    print(re24.table())
    print(re24.scored_table())


if __name__ == '__main__':
    main()
