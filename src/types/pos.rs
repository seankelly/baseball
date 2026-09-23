use std::default::Default;

use rusqlite::types::{FromSql, FromSqlResult, ToSql, ToSqlOutput, ValueRef};


// The least significant bit is ignored so the N in the 1<<N matches the position number.
const POSITION_CATCHER: u16             = 1 << 1;
const POSITION_PITCHER: u16             = 1 << 2;
const POSITION_FIRST_BASE: u16          = 1 << 3;
const POSITION_SECOND_BASE: u16         = 1 << 4;
const POSITION_THIRD_BASE: u16          = 1 << 5;
const POSITION_SHORTSTOP: u16           = 1 << 6;
const POSITION_LEFT_FIELD: u16          = 1 << 7;
const POSITION_CENTER_FIELD: u16        = 1 << 8;
const POSITION_RIGHT_FIELD: u16         = 1 << 9;
const POSITION_DESIGNATED_HITTER: u16   = 1 << 10;
const POSITION_PINCH_HITTER: u16        = 1 << 11;
const POSITION_PINCH_RUNNER: u16        = 1 << 12;
const POSITION_INVALID: u16             = 1 << 13;


type PositionMap<'a> = (u16, &'a str);

const POSITIONS: [PositionMap; 12] = [
    (POSITION_CATCHER, "1"),
    (POSITION_PITCHER, "2"),
    (POSITION_FIRST_BASE, "3"),
    (POSITION_SECOND_BASE, "4"),
    (POSITION_THIRD_BASE, "5"),
    (POSITION_SHORTSTOP, "6"),
    (POSITION_LEFT_FIELD, "7"),
    (POSITION_CENTER_FIELD, "8"),
    (POSITION_RIGHT_FIELD, "9"),
    (POSITION_DESIGNATED_HITTER, "D"),
    (POSITION_PINCH_HITTER, "H"),
    (POSITION_PINCH_RUNNER, "R"),
];


#[derive(Clone, Copy, Default)]
pub struct Position {
    positions: u16,
}


impl From<Position> for String {
    fn from(input: Position) -> String {
        let popcnt = input.positions.count_ones() as usize;
        let mut s = String::with_capacity(popcnt);
        for position in POSITIONS {
            if (input.positions & position.0) != 0 {
                s.push_str(position.1);
            }
        }
        s
    }
}


impl From<String> for Position {
    fn from(input: String) -> Position {
        input.as_str().into()
    }
}


impl From<&str> for Position {
    fn from(input: &str) -> Position {
        let mut pos = Position { positions: 0 };
        for chr in input.chars() {
            match chr {
                '1' .. '9' => {
                    let c = chr as u32 - '0' as u32;
                    // Max value is 1 << 9 and that fits within u16.
                    let bit = (1 << c) as u16;
                    pos.positions |= bit;
                }
                'D' => { pos.positions |= POSITION_DESIGNATED_HITTER; }
                'H' => { pos.positions |= POSITION_PINCH_HITTER; }
                'R' => { pos.positions |= POSITION_PINCH_RUNNER; }
                _ => {}
            }
        }
        pos
    }
}


impl FromSql for Position {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        let mut pos = Position { positions: 0 };
        match value {
            ValueRef::Integer(i) => {
                if i < POSITION_INVALID as i64 {
                    pos.positions = i as u16;
                }
            }
            ValueRef::Text(t) => {
                if let Ok(s) = str::from_utf8(t) {
                    let p: Position = s.into();
                    pos.positions = p.positions;
                }
            }
            _ => {},
        };
        Ok(pos)
    }
}


impl ToSql for Position {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        let p = *self;
        let s: String = p.into();
        Ok(ToSqlOutput::from(s))
    }
}
