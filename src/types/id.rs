use std::default::Default;

use rusqlite::types::{FromSql, FromSqlResult, ToSql, ToSqlOutput, ValueRef};


pub struct RetroId<const N: usize> {
    id: [u8; N],
}

pub type RetroPlayerId = RetroId<8>;
pub type RetroTeamId = RetroId<3>;
pub type RetroGameId = RetroId<12>;


impl<const N: usize> RetroId<N> {
    pub fn as_str(&self) -> &str {
        self.into()
    }

    pub fn is_empty(&self) -> bool {
        self.id == [0; N]
    }

    pub fn to_string(&self) -> String {
        self.into()
    }

    fn read_slice(&mut self, slice: &[u8]) {
        if slice.len() >= N {
            self.id.copy_from_slice(&slice[..N]);
        }
        else {
            for i in 0..slice.len() {
                self.id[i] = slice[i];
            }
        }
    }
}


impl<const N: usize> Default for RetroId<N> {
    fn default() -> Self {
        Self { id: [0; N] }
    }
}


impl<const N: usize> PartialEq<&RetroId<N>> for RetroId<N> {
    fn eq(&self, other: &&RetroId<N>) -> bool {
        self.id == other.id
    }
}


impl<const N: usize> PartialEq<RetroId<N>> for &RetroId<N> {
    fn eq(&self, other: &RetroId<N>) -> bool {
        self.id == other.id
    }
}


impl<'a, const N: usize> From<&'a RetroId<N>> for &'a str {
    fn from(input: &'a RetroId<N>) -> &'a str {
        str::from_utf8(&input.id).expect("Invalid UTF-8")
    }
}


impl<const N: usize> From<&RetroId<N>> for String {
    fn from(input: &RetroId<N>) -> String {
        let mut s = Vec::with_capacity(N);
        s.extend(&input.id);
        String::from_utf8(s).expect("Invalid UTF-8")
    }
}


impl<const N: usize> From<String> for RetroId<N> {
    fn from(input: String) -> RetroId<N> {
        let mut id = RetroId { id: [0; N] };
        id.read_slice(input.as_bytes());
        id
    }
}


impl<const N: usize> FromSql for RetroId<N> {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        let mut id = RetroId { id: [0; N] };
        match value {
            ValueRef::Text(t) => {
                id.read_slice(t);
            }
            _ => {},
        };
        Ok(id)
    }
}


impl<const N: usize> ToSql for RetroId<N> {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        let s: &str = self.into();
        Ok(ToSqlOutput::from(s))
    }
}
