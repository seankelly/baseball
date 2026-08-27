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
                "tb" => context.add_variable("tb", self.tb)?,
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
                "xi" => context.add_variable("xi", self.xi)?,
                "pos" => context.add_variable("pos", self.pos.clone())?,
                _ => {},
            }
        }

        Ok(())
    }

    fn check_cel_variables(variables: &[&str]) -> bool {
        for name in variables {
            match *name {
                "ab" => {},
                "pa" => {},
                "r" => {},
                "h" => {},
                "tb" => {},
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
                "xi" => {},
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
                "gs" => context.add_variable("gs", self.gs)?,
                "o" => context.add_variable("o", self.o)?,
                "po" => context.add_variable("po", self.po)?,
                "tc" => context.add_variable("po", self.tc)?,
                "a" => context.add_variable("a", self.a)?,
                "e" => context.add_variable("e", self.e)?,
                "dp" => context.add_variable("dp", self.dp)?,
                "tp" => context.add_variable("tp", self.tp)?,
                "pb" => context.add_variable("pb", self.pb)?,
                "ci" => context.add_variable("ci", self.ci)?,
                _ => {},
            }
        }

        Ok(())
    }

    fn check_cel_variables(variables: &[&str]) -> bool {
        for name in variables {
            match *name {
                "pos" => {},
                "gs" => {},
                "o" => {},
                "po" => {},
                "tc" => {},
                "a" => {},
                "e" => {},
                "dp" => {},
                "tp" => {},
                "pb" => {},
                "ci" => {},
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
                "cg" => context.add_variable("cg", self.cg)?,
                "sho" => context.add_variable("sho", self.sho)?,
                "gf" => context.add_variable("gf", self.gf)?,
                "w" => context.add_variable("w", self.w)?,
                "l" => context.add_variable("l", self.l)?,
                "sv" => context.add_variable("sv", self.sv)?,
                "ipouts" => context.add_variable("ipouts", self.ipouts)?,
                "ab" => context.add_variable("ab", self.ab)?,
                "bf" => context.add_variable("bf", self.bf)?,
                "h" => context.add_variable("h", self.h)?,
                "r" => context.add_variable("r", self.r)?,
                "er" => context.add_variable("er", self.er)?,
                "hr" => context.add_variable("hr", self.hr)?,
                "bb" => context.add_variable("bb", self.bb)?,
                "ibb" => context.add_variable("ibb", self.ibb)?,
                "so" => context.add_variable("so", self.so)?,
                "wp" => context.add_variable("wp", self.wp)?,
                "bk" => context.add_variable("bk", self.bk)?,
                "hbp" => context.add_variable("hbp", self.hbp)?,
                "go" => context.add_variable("go", self.go)?,
                "ao" => context.add_variable("ao", self.ao)?,
                "p" => context.add_variable("p", self.p)?,
                "s" => context.add_variable("s", self.s)?,
                "decision" => context.add_variable("decision", self.decision.clone())?,
                _ => {},
            }
        }

        Ok(())
    }

    fn check_cel_variables(variables: &[&str]) -> bool {
        for name in variables {
            match *name {
                "gs" => {},
                "cg" => {},
                "sho" => {},
                "gf" => {},
                "w" => {},
                "l" => {},
                "sv" => {},
                "ipouts" => {},
                "ab" => {},
                "bf" => {},
                "h" => {},
                "r" => {},
                "er" => {},
                "hr" => {},
                "bb" => {},
                "ibb" => {},
                "so" => {},
                "wp" => {},
                "bk" => {},
                "hbp" => {},
                "go" => {},
                "ao" => {},
                "p" => {},
                "s" => {},
                "decision" => {},
                _ => return false,
            }
        }

        true
    }
}
