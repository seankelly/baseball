use std::path::Path;

use csv::ReaderBuilder;
use serde_derive::Deserialize;


#[derive(Clone, Debug, Deserialize)]
pub struct Transaction {
    pub primary_date: String,
    pub time: Option<u8>,
    pub approximate_indicator: Option<bool>,
    pub secondary_date: Option<String>,
    pub secondary_approximate_indicator: Option<bool>,
    pub transaction_id: u32,
    pub player: String,
    pub transaction_type: TransactionType,
    pub from_team: Option<String>,
    pub from_league: Option<String>,
    pub to_team: Option<String>,
    pub to_league: Option<String>,
    pub draft_type: Option<DraftType>,
    pub draft_round: Option<u8>,
    pub pick_number: Option<u16>,
    pub info: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub enum TransactionTime {
    BeforeAllGames,
    InBetweenGames,
    AfterAllGames,
}

#[derive(Clone, Debug, Deserialize)]
pub enum TransactionType {
    A, // assigned from one team to another without compensation
    C, // conditional deal
    Cr, // returned to original team after conditional deal
    D, // rule 5 draft pick
    Da, // amateur draft pick
    Df, // first year draft pick
    Dm, // minor league draft pick
    Dn, // selected in amateur draft but did not sign
    Dr, // returned to original team after draft selection
    Ds, // special draft pick
    Dv, // amateur draft pick voided
    F, // free agent signing
    Fa, // amateur free agent signing
    Fb, // amateur free agent "bonus baby" signing under the 1953-57 rule requiring player to stay on ML roster
    Fc, // free agent compensation pick
    Fg, // free agent granted
    Fo, // free agent signing with first ML team
    Fv, // free agent signing voided
    Hb, // went on the bereavement list
    Hbr, // came off the bereavement list
    Hd, // declared ineligible
    Hdr, // reinistated from the ineligible list
    Hf, // demoted to the minor league
    Hfr, // promoted from the minor league
    Hh, // held out
    Hhr, // ended hold out
    Hi, // went on the disabled list
    Hir, // came off the disabled list
    Hm, // went into military service
    Hmr, // returned from military service
    Hs, // suspended
    Hsr, // reinstated after a suspension
    Hu, // unavailable but not on DL
    Hur, // returned from being unavailable
    Hv, // voluntarity retired
    Hvr, // unretired
    J, // jumped teams
    Jr, // returned to original team after jumping
    L, // loaned to another team
    Lr, // returned to original team after loan
    M, // obtained rights when entering into working agreement with minor league team
    Mr, // rights returned when working agreement with minor league team ended
    P, // purchase
    Pr, // returned to original team after purchase
    Pv, // purchase voided
    R, // release
    T, // trade
    Tn, // traded but refused to report
    Tp, // added to trade (usually because one of the original players refused to report or retired)
    Tr, // returned to original team after trade
    Tv, // trade voided
    U, // unknown (could have been two separate transactions)
    Vg, // player assigned to league control
    V, // player purchased or assigned to team from league
    W, // waiver pick
    Wf, // first year waiver pick
    Wr, // returned to original team after waiver pick
    Wv, // waiver pick voided
    X, // expansion draft
    Xp, // added as expansion pick at a later date
    Z, // voluntarily retired
    Zr, // returned from voluntarily retired list
}

#[derive(Clone, Debug, Deserialize)]
pub enum DraftType {
    Regular,
    Secondary,
    SecondaryActive,
    AmericanLegion,
    Dominican,
}


impl Transaction {
    pub fn load_transactions(file: &Path) -> Vec<Transaction> {
        let mut csv_reader = ReaderBuilder::new()
                                .has_headers(false)
                                .from_path(file)
                                .expect("Couldn't open file.");

        let mut transactions = Vec::new();
        for record in csv_reader.deserialize() {
            let transaction: Transaction = record.expect("Couldn't decode transaction");
            transactions.push(transaction);
        }
        transactions
    }
}
