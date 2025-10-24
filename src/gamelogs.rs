use std::error::Error;
use std::default::Default;
use std::str;

use crate::search::CelSearch;

use baseball::chadwick;
use cel_interpreter::Context;
use quick_xml::events::BytesStart;
use serde::Serialize;
use serde_derive::Deserialize;


fn attribute_to_u8(attr: &quick_xml::events::attributes::Attribute) -> u8 {
    let attribute = str::from_utf8(attr.value.as_ref());
    u8::from_str_radix(attribute.unwrap_or("0"), 10).unwrap_or(0)
}


fn attribute_to_bool(attr: &quick_xml::events::attributes::Attribute) -> bool {
    let attribute = str::from_utf8(attr.value.as_ref());
    match attribute.unwrap_or("0") {
        "0" => false,
        "1" => true,
        _ => false,
    }
}


impl CelSearch for chadwick::gamelogs::BattingGamelog {
    fn add_cel_variables(&self, context: &mut Context) -> Result<(), Box<dyn Error>> {
        context.add_variable("AB", self.AB)?;
        context.add_variable("PA", self.PA)?;
        context.add_variable("R", self.R)?;
        context.add_variable("H", self.H)?;
        context.add_variable("D", self.D)?;
        context.add_variable("T", self.T)?;
        context.add_variable("HR", self.HR)?;
        context.add_variable("RBI", self.RBI)?;
        context.add_variable("SB", self.SB)?;
        context.add_variable("CS", self.CS)?;
        context.add_variable("BB", self.BB)?;
        context.add_variable("SO", self.SO)?;
        context.add_variable("IBB", self.IBB)?;
        context.add_variable("HBP", self.HBP)?;
        context.add_variable("SH", self.SH)?;
        context.add_variable("SF", self.SF)?;
        context.add_variable("GIDP", self.GIDP)?;
        context.add_variable("pos", self.POS.clone())?;
        Ok(())
    }
}


impl CelSearch for chadwick::gamelogs::FieldingGamelog {
    fn add_cel_variables(&self, context: &mut Context) -> Result<(), Box<dyn Error>> {
        context.add_variable("POS", self.POS)?;
        context.add_variable("O", self.O)?;
        context.add_variable("PO", self.PO)?;
        context.add_variable("A", self.A)?;
        context.add_variable("E", self.E)?;
        context.add_variable("DP", self.DP)?;
        context.add_variable("TP", self.TP)?;
        context.add_variable("BIP", self.BIP)?;
        context.add_variable("BF", self.BF)?;
        Ok(())
    }
}


impl CelSearch for chadwick::gamelogs::PitchingGamelog {
    fn add_cel_variables(&self, context: &mut Context) -> Result<(), Box<dyn Error>> {
        context.add_variable("GS", self.GS)?;
        context.add_variable("GF", self.GF)?;
        context.add_variable("CG", self.CG)?;
        context.add_variable("SHO", self.SHO)?;
        context.add_variable("IPOuts", self.IPouts)?;
        context.add_variable("H", self.H)?;
        context.add_variable("R", self.R)?;
        context.add_variable("ER", self.ER)?;
        context.add_variable("HR", self.HR)?;
        context.add_variable("BB", self.BB)?;
        context.add_variable("SO", self.SO)?;
        context.add_variable("IBB", self.IBB)?;
        context.add_variable("WP", self.WP)?;
        context.add_variable("HBP", self.HBP)?;
        context.add_variable("BK", self.BK)?;
        context.add_variable("BF", self.BF)?;
        context.add_variable("GF", self.GF)?;
        context.add_variable("P", self.P)?;
        context.add_variable("S", self.S)?;
        context.add_variable("decision", self.decision.clone())?;
        Ok(())
    }
}
