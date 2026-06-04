use std::error::Error;

use crate::search::CelEval;

use baseball::chadwick;
use cel::Context;


impl CelEval for chadwick::gamelogs::BattingGamelog {
    fn add_cel_variables(&self, context: &mut Context, variables: &[&str]) -> Result<(), Box<dyn Error>> {
        for name in variables {
            match *name {
                "ab" => context.add_variable("ab", self.ab)?,
                "pa" => context.add_variable("pa", self.pa)?,
                "r" => context.add_variable("r", self.r)?,
                "h" => context.add_variable("h", self.h)?,
                "d" => context.add_variable("d", self.d)?,
                "t" => context.add_variable("t", self.t)?,
                "hr" => context.add_variable("hr", self.hr)?,
                "rbi" => context.add_variable("rbi", self.rbi)?,
                "sb" => context.add_variable("sb", self.sb)?,
                "cs" => context.add_variable("cs", self.cs)?,
                "bb" => context.add_variable("bb", self.bb)?,
                "so" => context.add_variable("so", self.so)?,
                "ibb" => context.add_variable("ibb", self.ibb)?,
                "hbp" => context.add_variable("hbp", self.hbp)?,
                "sh" => context.add_variable("sh", self.sh)?,
                "sf" => context.add_variable("sf", self.sf)?,
                "gidp" => context.add_variable("gidp", self.gidp)?,
                "pos" => context.add_variable("pos", self.pos.clone())?,
                _ => {},
            }
        }

        Ok(())
    }

    fn check_cel_variables(&self, variables: &[&str]) -> bool {
        for name in variables {
            match *name {
                "ab" => {},
                "pa" => {},
                "r" => {},
                "h" => {},
                "d" => {},
                "t" => {},
                "hr" => {},
                "rbi" => {},
                "sb" => {},
                "cs" => {},
                "bb" => {},
                "so" => {},
                "ibb" => {},
                "hbp" => {},
                "sh" => {},
                "sf" => {},
                "gidp" => {},
                "pos" => {},
                _ => return false,
            }
        }

        true
    }
}


impl CelEval for chadwick::gamelogs::FieldingGamelog {
    fn add_cel_variables(&self, context: &mut Context, variables: &[&str]) -> Result<(), Box<dyn Error>> {
        for name in variables {
            match *name {
                "pos" => context.add_variable("pos", self.pos)?,
                "o" => context.add_variable("o", self.o)?,
                "po" => context.add_variable("po", self.po)?,
                "a" => context.add_variable("a", self.a)?,
                "e" => context.add_variable("e", self.e)?,
                "dp" => context.add_variable("dp", self.dp)?,
                "tp" => context.add_variable("tp", self.tp)?,
                "bip" => context.add_variable("bip", self.bip)?,
                "bf" => context.add_variable("bf", self.bf)?,
                _ => {},
            }
        }

        Ok(())
    }

    fn check_cel_variables(&self, variables: &[&str]) -> bool {
        for name in variables {
            match *name {
                "pos" => {},
                "o" => {},
                "po" => {},
                "a" => {},
                "e" => {},
                "dp" => {},
                "tp" => {},
                "bip" => {},
                "bf" => {},
                _ => return false,
            }
        }

        true
    }
}


impl CelEval for chadwick::gamelogs::PitchingGamelog {
    fn add_cel_variables(&self, context: &mut Context, variables: &[&str]) -> Result<(), Box<dyn Error>> {
        for name in variables {
            match *name {
                "gs" => context.add_variable("gs", self.gs)?,
                "gf" => context.add_variable("gf", self.gf)?,
                "cg" => context.add_variable("cg", self.cg)?,
                "sho" => context.add_variable("sho", self.sho)?,
                "ipouts" => context.add_variable("ipouts", self.ipouts)?,
                "h" => context.add_variable("h", self.h)?,
                "r" => context.add_variable("r", self.r)?,
                "er" => context.add_variable("er", self.er)?,
                "hr" => context.add_variable("hr", self.hr)?,
                "bb" => context.add_variable("bb", self.bb)?,
                "so" => context.add_variable("so", self.so)?,
                "ibb" => context.add_variable("ibb", self.ibb)?,
                "wp" => context.add_variable("wp", self.wp)?,
                "hbp" => context.add_variable("hbp", self.hbp)?,
                "bk" => context.add_variable("bk", self.bk)?,
                "bf" => context.add_variable("bf", self.bf)?,
                "p" => context.add_variable("p", self.p)?,
                "s" => context.add_variable("s", self.s)?,
                "decision" => context.add_variable("decision", self.decision.clone())?,
                _ => {},
            }
        }

        Ok(())
    }

    fn check_cel_variables(&self, variables: &[&str]) -> bool {
        for name in variables {
            match *name {
                "gs" => {},
                "gf" => {},
                "cg" => {},
                "sho" => {},
                "ipouts" => {},
                "h" => {},
                "r" => {},
                "er" => {},
                "hr" => {},
                "bb" => {},
                "so" => {},
                "ibb" => {},
                "wp" => {},
                "hbp" => {},
                "bk" => {},
                "bf" => {},
                "p" => {},
                "s" => {},
                "decision" => {},
                _ => return false,
            }
        }

        true
    }
}
