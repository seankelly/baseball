use std::error::Error;

use crate::search::CelSearch;

use baseball::chadwick;
use cel_interpreter::Context;


impl CelSearch for chadwick::gamelogs::BattingGamelog {
    fn add_cel_variables(&self, context: &mut Context) -> Result<(), Box<dyn Error>> {
        context.add_variable("ab", self.ab)?;
        context.add_variable("pa", self.pa)?;
        context.add_variable("r", self.r)?;
        context.add_variable("h", self.h)?;
        context.add_variable("d", self.d)?;
        context.add_variable("t", self.t)?;
        context.add_variable("hr", self.hr)?;
        context.add_variable("rbi", self.rbi)?;
        context.add_variable("sb", self.sb)?;
        context.add_variable("cs", self.cs)?;
        context.add_variable("bb", self.bb)?;
        context.add_variable("so", self.so)?;
        context.add_variable("ibb", self.ibb)?;
        context.add_variable("hbp", self.hbp)?;
        context.add_variable("sh", self.sh)?;
        context.add_variable("sf", self.sf)?;
        context.add_variable("gidp", self.gidp)?;
        context.add_variable("pos", self.pos.clone())?;
        Ok(())
    }
}


impl CelSearch for chadwick::gamelogs::FieldingGamelog {
    fn add_cel_variables(&self, context: &mut Context) -> Result<(), Box<dyn Error>> {
        context.add_variable("pos", self.pos)?;
        context.add_variable("o", self.o)?;
        context.add_variable("po", self.po)?;
        context.add_variable("a", self.a)?;
        context.add_variable("e", self.e)?;
        context.add_variable("dp", self.dp)?;
        context.add_variable("tp", self.tp)?;
        context.add_variable("bip", self.bip)?;
        context.add_variable("bf", self.bf)?;
        Ok(())
    }
}


impl CelSearch for chadwick::gamelogs::PitchingGamelog {
    fn add_cel_variables(&self, context: &mut Context) -> Result<(), Box<dyn Error>> {
        context.add_variable("gs", self.gs)?;
        context.add_variable("gf", self.gf)?;
        context.add_variable("cg", self.cg)?;
        context.add_variable("sho", self.sho)?;
        context.add_variable("ipouts", self.ipouts)?;
        context.add_variable("h", self.h)?;
        context.add_variable("r", self.r)?;
        context.add_variable("er", self.er)?;
        context.add_variable("hr", self.hr)?;
        context.add_variable("bb", self.bb)?;
        context.add_variable("so", self.so)?;
        context.add_variable("ibb", self.ibb)?;
        context.add_variable("wp", self.wp)?;
        context.add_variable("hbp", self.hbp)?;
        context.add_variable("bk", self.bk)?;
        context.add_variable("bf", self.bf)?;
        context.add_variable("gf", self.gf)?;
        context.add_variable("p", self.p)?;
        context.add_variable("s", self.s)?;
        context.add_variable("decision", self.decision.clone())?;
        Ok(())
    }
}
