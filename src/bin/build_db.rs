use std::cmp;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io;
use std::path;
use std::process::{ChildStdout, Command, Stdio};

use baseball::register::Person;
use baseball::retrosheet::game;
use baseball::chadwick::gamelogs::{gamelogs_from_boxscores, BattingGamelog, FieldingGamelog, PitchingGamelog};
use baseball_tools::games;
use baseball_tools::player;
use baseball_tools::internals::Guts;

use clap::Parser;
use csv::ReaderBuilder;
use rusqlite::{Connection, Result, Transaction, named_params};


#[derive(Parser)]
struct DatabaseArgs {
    #[arg(short, long)]
    database: Option<path::PathBuf>,

    #[arg(short, long)]
    init: bool,

    #[arg(short = 'G', long)]
    gamelogs: bool,

    #[arg(long)]
    count_career_games: bool,

    #[arg(short, long)]
    games: bool,

    #[arg(short = 'R', long)]
    register_dir: Option<path::PathBuf>,

    #[arg(short = 'r', long)]
    retrosheet_dir: Option<path::PathBuf>,

    seasons: Vec<String>,
}


struct GameLoader<'a> {
    conn: &'a mut Connection,
    retrosheet_dir: path::PathBuf,
}


impl<'a> GameLoader<'a> {
    fn new(conn: &'a mut Connection, retrosheet_dir: path::PathBuf) -> Self {
        Self {
            conn,
            retrosheet_dir
        }
    }

    // SQL interaction section.
    fn create_games_table(&mut self) -> Result<(), Box<dyn Error>> {
        let tx = self.conn.transaction().expect("Could not create transaction");
        tx.execute("DROP TABLE IF EXISTS games", ())?;
        tx.execute(include_str!("../sql/create_games.sql"), ())?;
        tx.commit()?;
        Ok(())
    }

    fn insert_games(tx: &Transaction, games: &Vec<games::GameLog>) -> Result<(), Box<dyn Error>> {
        let insert_sql = String::from(
            "INSERT INTO games VALUES (
                :game_id, :date, :number_of_game, :day_of_week, :visitor_team, :visitor_league,
                :visitor_team_game_number, :home_team, :home_league, :home_team_game_number,
                :visitor_score, :home_score, :number_of_outs, :day_night, :completion_info,
                :forfeit_info, :protest_info, :park_id, :attendance, :time_of_game,
                :visitor_line_score, :home_line_score, :visitor_ab, :visitor_hits,
                :visitor_doubles, :visitor_triples, :visitor_homeruns, :visitor_rbi,
                :visitor_sac_hits, :visitor_sac_flies, :visitor_hbp, :visitor_walks,
                :visitor_intentional_walks, :visitor_strikeouts, :visitor_stolen_bases,
                :visitor_caught_stealing, :visitor_gidp, :visitor_catcher_interference,
                :visitor_left_on_base, :visitor_pitchers_used, :visitor_individual_earned_runs,
                :visitor_team_earned_runs, :visitor_wild_pitches, :visitor_balks, :visitor_putouts,
                :visitor_assists, :visitor_errors, :visitor_passed_balls, :visitor_double_plays,
                :visitor_triple_plays, :home_ab, :home_hits, :home_doubles, :home_triples,
                :home_homeruns, :home_rbi, :home_sac_hits, :home_sac_flies, :home_hbp, :home_walks,
                :home_intentional_walks, :home_strikeouts, :home_stolen_bases,
                :home_caught_stealing, :home_gidp, :home_catcher_interference, :home_left_on_base,
                :home_pitchers_used, :home_individual_earned_runs, :home_team_earned_runs,
                :home_wild_pitches, :home_balks, :home_putouts, :home_assists, :home_errors,
                :home_passed_balls, :home_double_plays, :home_triple_plays,
                :home_plate_umpire_name, :home_plate_umpire_id, :first_base_umpire_name,
                :first_base_umpire_id, :second_base_umpire_name, :second_base_umpire_id,
                :third_base_umpire_name, :third_base_umpire_id, :left_field_umpire_name,
                :left_field_umpire_id, :right_field_umpire_name, :right_field_umpire_id,
                :visitor_manager_id, :visitor_manager_name, :home_manager_id, :home_manager_name,
                :winning_pitcher_name, :winning_pitcher_id, :losing_pitcher_name,
                :losing_pitcher_id, :saving_pitcher_name, :saving_pitcher_id, :gwrbi_player_name,
                :gwrbi_player_id, :visitor_starter_name, :visitor_starter_id, :home_starter_name,
                :home_starter_id, :visitor_1_id, :visitor_1_name, :visitor_1_pos, :visitor_2_id,
                :visitor_2_name, :visitor_2_pos, :visitor_3_id, :visitor_3_name, :visitor_3_pos,
                :visitor_4_id, :visitor_4_name, :visitor_4_pos, :visitor_5_id, :visitor_5_name,
                :visitor_5_pos, :visitor_6_id, :visitor_6_name, :visitor_6_pos, :visitor_7_id,
                :visitor_7_name, :visitor_7_pos, :visitor_8_id, :visitor_8_name, :visitor_8_pos,
                :visitor_9_id, :visitor_9_name, :visitor_9_pos, :home_1_id, :home_1_name,
                :home_1_pos, :home_2_id, :home_2_name, :home_2_pos, :home_3_id, :home_3_name,
                :home_3_pos, :home_4_id, :home_4_name, :home_4_pos, :home_5_id, :home_5_name,
                :home_5_pos, :home_6_id, :home_6_name, :home_6_pos, :home_7_id, :home_7_name,
                :home_7_pos, :home_8_id, :home_8_name, :home_8_pos, :home_9_id, :home_9_name,
                :home_9_pos, :additional_info, :acquisition_info)");

        let mut insert = tx.prepare(&insert_sql)?;
        for game in games {
            insert.execute(
                named_params! {
                    ":game_id": &game.game_id,
                    ":date": &game.date,
                    ":number_of_game": &game.number_of_game,
                    ":day_of_week": &game.day_of_week,
                    ":visitor_team": &game.visitor_team,
                    ":visitor_league": &game.visitor_league,
                    ":visitor_team_game_number": &game.visitor_team_game_number,
                    ":home_team": &game.home_team,
                    ":home_league": &game.home_league,
                    ":home_team_game_number": &game.home_team_game_number,
                    ":visitor_score": &game.visitor_score,
                    ":home_score": &game.home_score,
                    ":number_of_outs": &game.number_of_outs,
                    ":day_night": &game.day_night,
                    ":completion_info": &game.completion_info,
                    ":forfeit_info": &game.forfeit_info,
                    ":protest_info": &game.protest_info,
                    ":park_id": &game.park_id,
                    ":attendance": &game.attendance,
                    ":time_of_game": &game.time_of_game,
                    ":visitor_line_score": &game.visitor_line_score,
                    ":home_line_score": &game.home_line_score,
                    ":visitor_ab": &game.visitor_ab,
                    ":visitor_hits": &game.visitor_hits,
                    ":visitor_doubles": &game.visitor_doubles,
                    ":visitor_triples": &game.visitor_triples,
                    ":visitor_homeruns": &game.visitor_homeruns,
                    ":visitor_rbi": &game.visitor_rbi,
                    ":visitor_sac_hits": &game.visitor_sac_hits,
                    ":visitor_sac_flies": &game.visitor_sac_flies,
                    ":visitor_hbp": &game.visitor_hbp,
                    ":visitor_walks": &game.visitor_walks,
                    ":visitor_intentional_walks": &game.visitor_intentional_walks,
                    ":visitor_strikeouts": &game.visitor_strikeouts,
                    ":visitor_stolen_bases": &game.visitor_stolen_bases,
                    ":visitor_caught_stealing": &game.visitor_caught_stealing,
                    ":visitor_gidp": &game.visitor_gidp,
                    ":visitor_catcher_interference": &game.visitor_catcher_interference,
                    ":visitor_left_on_base": &game.visitor_left_on_base,
                    ":visitor_pitchers_used": &game.visitor_pitchers_used,
                    ":visitor_individual_earned_runs": &game.visitor_individual_earned_runs,
                    ":visitor_team_earned_runs": &game.visitor_team_earned_runs,
                    ":visitor_wild_pitches": &game.visitor_wild_pitches,
                    ":visitor_balks": &game.visitor_balks,
                    ":visitor_putouts": &game.visitor_putouts,
                    ":visitor_assists": &game.visitor_assists,
                    ":visitor_errors": &game.visitor_errors,
                    ":visitor_passed_balls": &game.visitor_passed_balls,
                    ":visitor_double_plays": &game.visitor_double_plays,
                    ":visitor_triple_plays": &game.visitor_triple_plays,
                    ":home_ab": &game.home_ab,
                    ":home_hits": &game.home_hits,
                    ":home_doubles": &game.home_doubles,
                    ":home_triples": &game.home_triples,
                    ":home_homeruns": &game.home_homeruns,
                    ":home_rbi": &game.home_rbi,
                    ":home_sac_hits": &game.home_sac_hits,
                    ":home_sac_flies": &game.home_sac_flies,
                    ":home_hbp": &game.home_hbp,
                    ":home_walks": &game.home_walks,
                    ":home_intentional_walks": &game.home_intentional_walks,
                    ":home_strikeouts": &game.home_strikeouts,
                    ":home_stolen_bases": &game.home_stolen_bases,
                    ":home_caught_stealing": &game.home_caught_stealing,
                    ":home_gidp": &game.home_gidp,
                    ":home_catcher_interference": &game.home_catcher_interference,
                    ":home_left_on_base": &game.home_left_on_base,
                    ":home_pitchers_used": &game.home_pitchers_used,
                    ":home_individual_earned_runs": &game.home_individual_earned_runs,
                    ":home_team_earned_runs": &game.home_team_earned_runs,
                    ":home_wild_pitches": &game.home_wild_pitches,
                    ":home_balks": &game.home_balks,
                    ":home_putouts": &game.home_putouts,
                    ":home_assists": &game.home_assists,
                    ":home_errors": &game.home_errors,
                    ":home_passed_balls": &game.home_passed_balls,
                    ":home_double_plays": &game.home_double_plays,
                    ":home_triple_plays": &game.home_triple_plays,
                    ":home_plate_umpire_name": &game.home_plate_umpire_name,
                    ":home_plate_umpire_id": &game.home_plate_umpire_id,
                    ":first_base_umpire_name": &game.first_base_umpire_name,
                    ":first_base_umpire_id": &game.first_base_umpire_id,
                    ":second_base_umpire_name": &game.second_base_umpire_name,
                    ":second_base_umpire_id": &game.second_base_umpire_id,
                    ":third_base_umpire_name": &game.third_base_umpire_name,
                    ":third_base_umpire_id": &game.third_base_umpire_id,
                    ":left_field_umpire_name": &game.left_field_umpire_name,
                    ":left_field_umpire_id": &game.left_field_umpire_id,
                    ":right_field_umpire_name": &game.right_field_umpire_name,
                    ":right_field_umpire_id": &game.right_field_umpire_id,
                    ":visitor_manager_id": &game.visitor_manager_id,
                    ":visitor_manager_name": &game.visitor_manager_name,
                    ":home_manager_id": &game.home_manager_id,
                    ":home_manager_name": &game.home_manager_name,
                    ":winning_pitcher_name": &game.winning_pitcher_name,
                    ":winning_pitcher_id": &game.winning_pitcher_id,
                    ":losing_pitcher_name": &game.losing_pitcher_name,
                    ":losing_pitcher_id": &game.losing_pitcher_id,
                    ":saving_pitcher_name": &game.saving_pitcher_name,
                    ":saving_pitcher_id": &game.saving_pitcher_id,
                    ":gwrbi_player_name": &game.gwrbi_player_name,
                    ":gwrbi_player_id": &game.gwrbi_player_id,
                    ":visitor_starter_name": &game.visitor_starter_name,
                    ":visitor_starter_id": &game.visitor_starter_id,
                    ":home_starter_name": &game.home_starter_name,
                    ":home_starter_id": &game.home_starter_id,
                    ":visitor_1_id": &game.visitor_1_id,
                    ":visitor_1_name": &game.visitor_1_name,
                    ":visitor_1_pos": &game.visitor_1_pos,
                    ":visitor_2_id": &game.visitor_2_id,
                    ":visitor_2_name": &game.visitor_2_name,
                    ":visitor_2_pos": &game.visitor_2_pos,
                    ":visitor_3_id": &game.visitor_3_id,
                    ":visitor_3_name": &game.visitor_3_name,
                    ":visitor_3_pos": &game.visitor_3_pos,
                    ":visitor_4_id": &game.visitor_4_id,
                    ":visitor_4_name": &game.visitor_4_name,
                    ":visitor_4_pos": &game.visitor_4_pos,
                    ":visitor_5_id": &game.visitor_5_id,
                    ":visitor_5_name": &game.visitor_5_name,
                    ":visitor_5_pos": &game.visitor_5_pos,
                    ":visitor_6_id": &game.visitor_6_id,
                    ":visitor_6_name": &game.visitor_6_name,
                    ":visitor_6_pos": &game.visitor_6_pos,
                    ":visitor_7_id": &game.visitor_7_id,
                    ":visitor_7_name": &game.visitor_7_name,
                    ":visitor_7_pos": &game.visitor_7_pos,
                    ":visitor_8_id": &game.visitor_8_id,
                    ":visitor_8_name": &game.visitor_8_name,
                    ":visitor_8_pos": &game.visitor_8_pos,
                    ":visitor_9_id": &game.visitor_9_id,
                    ":visitor_9_name": &game.visitor_9_name,
                    ":visitor_9_pos": &game.visitor_9_pos,
                    ":home_1_id": &game.home_1_id,
                    ":home_1_name": &game.home_1_name,
                    ":home_1_pos": &game.home_1_pos,
                    ":home_2_id": &game.home_2_id,
                    ":home_2_name": &game.home_2_name,
                    ":home_2_pos": &game.home_2_pos,
                    ":home_3_id": &game.home_3_id,
                    ":home_3_name": &game.home_3_name,
                    ":home_3_pos": &game.home_3_pos,
                    ":home_4_id": &game.home_4_id,
                    ":home_4_name": &game.home_4_name,
                    ":home_4_pos": &game.home_4_pos,
                    ":home_5_id": &game.home_5_id,
                    ":home_5_name": &game.home_5_name,
                    ":home_5_pos": &game.home_5_pos,
                    ":home_6_id": &game.home_6_id,
                    ":home_6_name": &game.home_6_name,
                    ":home_6_pos": &game.home_6_pos,
                    ":home_7_id": &game.home_7_id,
                    ":home_7_name": &game.home_7_name,
                    ":home_7_pos": &game.home_7_pos,
                    ":home_8_id": &game.home_8_id,
                    ":home_8_name": &game.home_8_name,
                    ":home_8_pos": &game.home_8_pos,
                    ":home_9_id": &game.home_9_id,
                    ":home_9_name": &game.home_9_name,
                    ":home_9_pos": &game.home_9_pos,
                    ":additional_info": &game.additional_info,
                    ":acquisition_info": &game.acquisition_info,
                }
            )?;
        }

        Ok(())
    }

    fn load_season_gamelog(&self, season: &String) -> Result<Vec<games::GameLog>, Box<dyn Error>> {
        let season_dir = self.retrosheet_dir.join(season);
        // Chadwick's Retrosheet seasons either have a GLYYYY.TXT or glYYYY.txt file.
        let gl_file_names = [format!("GL{}.TXT", season), format!("gl{}.txt", season)];
        let mut gamelog_file = None;
        for gl_file in gl_file_names {
            let gl_path = season_dir.join(gl_file);
            if gl_path.exists() {
                gamelog_file = Some(gl_path);
                break;
            }
        }

        let mut games = Vec::new();
        if let Some(gl_path) = gamelog_file {
            let mut reader = ReaderBuilder::new()
                .has_headers(false)
                .from_path(&gl_path)?;
            for result in reader.deserialize() {
                match result {
                    Ok(game) => {
                        let game: game::GameLog = game;
                        games.push(game.into());
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }
        }
        Ok(games)
    }

    fn load(&mut self, seasons: &Vec<String>, initialize: bool) -> Result<(), Box<dyn Error>> {
        if initialize {
            println!("Creating games tables");
            self.create_games_table()?;
        }

        for season in seasons {
            println!("Loading games from {} season", season);
            let games = self.load_season_gamelog(season)?;
            println!("Found {} games", games.len());
            // Iterate one more time through every pitching game to calculate the league ERA and the
            // unscaled FIP values to get the FIP constant for this season.
            let league_stats = games.iter().fold(PitcherStats::new_with_fip(0.0), |mut lgstats, g| {
                lgstats.add_team_gamelog(g);
                lgstats
            });
            let league_fip_constant = league_stats.era() - league_stats.fip();
            println!("Season {} ERA: {}, FIP constant: {}", season, league_stats.era(), league_fip_constant);
            let season_numeric = season.parse::<u16>()?;
            let mut guts = Guts::new(season_numeric);
            guts.fip_constant = league_fip_constant;

            let tx = self.conn.transaction().expect("Could not create transaction");
            update_fip_constant(&tx, &guts)?;
            Self::insert_games(&tx, &games)?;
            tx.commit().expect("Failed to commit transaction");
        }

        if initialize {
            println!("Creating game indexes");
            self.conn.execute_batch(
                "
                CREATE INDEX games_game_idx ON games (game_id);
                CREATE INDEX games_date_idx ON games (date);
                CREATE INDEX games_year_idx ON games (strftime('%Y', date));
                CREATE INDEX games_away_idx ON games (visitor_team);
                CREATE INDEX games_home_idx ON games (home_team);
                "
            )?;
        }

        Ok(())
    }
}

struct PlayerGamelogs<'a> {
    conn: &'a mut Connection,
    retrosheet_dir: path::PathBuf,
}


type DatedPlayerGamelogs<T> = (T, chrono::NaiveDate);


struct BattingSlashLine {
    ab: u16,
    h: u16,
    tb: u16,
    bb: u8,
    hbp: u8,
    sf: u8,
}


// Value scaled so they will work for the league totals.
struct PitcherStats {
    ipouts: u32,
    er: u16,
    hr: u16,
    bb: u16,
    hbp: u16,
    so: u16,
    fip_constant: f32,
}


#[derive(Eq, Hash, PartialEq)]
struct TeamGameLogKey {
    game_id: String,
    team_id: String,
}


struct TeamGameLogValue {
    date: chrono::NaiveDate,
    team_game_number: u16,
}


#[derive(Eq, PartialEq)]
struct CareerGame {
    player_id: String,
    game_id: String,
    date: chrono::NaiveDate,
}


impl<'a> PlayerGamelogs<'a> {
    fn new(conn: &'a mut Connection, retrosheet_dir: path::PathBuf) -> Self {
        Self {
            conn,
            retrosheet_dir
        }
    }

    fn create_batting_gamelogs_table(&mut self) -> Result<(), Box<dyn Error>> {
        let tx = self.conn.transaction().expect("Could not create transaction");
        tx.execute("DROP TABLE IF EXISTS batting_gamelogs", ())?;
        tx.execute(include_str!("../sql/create_batting_gamelogs.sql"), ())?;
        tx.commit()?;
        Ok(())
    }

    fn create_fielding_gamelogs_table(&mut self) -> Result<(), Box<dyn Error>> {
        let tx = self.conn.transaction().expect("Could not create transaction");
        tx.execute("DROP TABLE IF EXISTS fielding_gamelogs", ())?;
        tx.execute(include_str!("../sql/create_fielding_gamelogs.sql"), ())?;
        tx.commit()?;
        Ok(())
    }

    fn create_pitching_gamelogs_table(&mut self) -> Result<(), Box<dyn Error>> {
        let tx = self.conn.transaction().expect("Could not create transaction");
        tx.execute("DROP TABLE IF EXISTS pitching_gamelogs", ())?;
        tx.execute(include_str!("../sql/create_pitching_gamelogs.sql"), ())?;
        tx.commit()?;
        Ok(())
    }

    fn insert_batting_gamelogs(tx: &Transaction, gamelogs: &Vec<player::BattingGamelog>) -> Result<(), Box<dyn Error>> {
        let insert_sql = String::from(
            "INSERT INTO batting_gamelogs VALUES (
                :player_id, :game_id, :team_id, :career_game, :season_game, :team_game, :pa, :ab,
                :r, :h, :d, :t, :hr, :rbi, :rbi2out, :bb, :ibb, :so, :gidp, :hbp, :sh, :sf, :sb,
                :cs, :avg, :obp, :slg, :woba, :babip, :pos)");

        let mut insert = tx.prepare(&insert_sql)?;
        for game in gamelogs {
            insert.execute(
                named_params! {
                    ":player_id": &game.player_id,
                    ":game_id": &game.game_id,
                    ":team_id": &game.team_id,
                    ":career_game": &game.career_game,
                    ":season_game": &game.season_game,
                    ":team_game": &game.team_game,
                    ":pa": &game.pa,
                    ":ab": &game.ab,
                    ":r": &game.r,
                    ":h": &game.h,
                    ":d": &game.d,
                    ":t": &game.t,
                    ":hr": &game.hr,
                    ":rbi": &game.rbi,
                    ":rbi2out": &game.rbi2out,
                    ":bb": &game.bb,
                    ":ibb": &game.ibb,
                    ":so": &game.so,
                    ":gidp": &game.gidp,
                    ":hbp": &game.hbp,
                    ":sh": &game.sh,
                    ":sf": &game.sf,
                    ":sb": &game.sb,
                    ":cs": &game.cs,
                    ":avg": &game.avg,
                    ":obp": &game.obp,
                    ":slg": &game.slg,
                    ":woba": &game.woba,
                    ":babip": &game.babip,
                    ":pos": &game.pos,
                }
            )?;
        }

        Ok(())
    }

    fn insert_fielding_gamelogs(tx: &Transaction, gamelogs: &Vec<player::FieldingGamelog>) -> Result<(), Box<dyn Error>> {
        let insert_sql = String::from(
            "INSERT INTO fielding_gamelogs VALUES (
                :player_id, :game_id, :team_id, :career_game, :season_game, :team_game, :pos, :o,
                :po, :a, :e, :dp, :tp, :bip, :bf)");

        let mut insert = tx.prepare(&insert_sql)?;
        for game in gamelogs {
            insert.execute(
                named_params! {
                    ":player_id": &game.player_id,
                    ":game_id": &game.game_id,
                    ":team_id": &game.team_id,
                    ":career_game": &game.career_game,
                    ":season_game": &game.season_game,
                    ":team_game": &game.team_game,
                    ":pos": &game.pos,
                    ":o": &game.o,
                    ":po": &game.po,
                    ":a": &game.a,
                    ":e": &game.e,
                    ":dp": &game.dp,
                    ":tp": &game.tp,
                    ":bip": &game.bip,
                    ":bf": &game.bf,
                }
            )?;
        }

        Ok(())
    }

    fn insert_pitching_gamelogs(tx: &Transaction, gamelogs: &Vec<player::PitchingGamelog>) -> Result<(), Box<dyn Error>> {
        let insert_sql = String::from(
            "INSERT INTO pitching_gamelogs VALUES (
                :player_id, :game_id, :team_id, :career_game, :season_game, :team_game, :gs, :cg,
                :sho, :gf, :ipouts, :ab, :bf, :h, :r, :er, :hr, :bb, :ibb, :so, :wp, :bk, :hbp,
                :gb, :fb, :p, :s, :decision, :era, :fip)");

        let mut insert = tx.prepare(&insert_sql)?;
        for game in gamelogs {
            insert.execute(
                named_params! {
                    ":player_id": &game.player_id,
                    ":game_id": &game.game_id,
                    ":team_id": &game.team_id,
                    ":career_game": &game.career_game,
                    ":season_game": &game.season_game,
                    ":team_game": &game.team_game,
                    ":gs": &game.gs,
                    ":cg": &game.cg,
                    ":sho": &game.sho,
                    ":gf": &game.gf,
                    ":ipouts": &game.ipouts,
                    ":ab": &game.ab,
                    ":bf": &game.bf,
                    ":h": &game.h,
                    ":r": &game.r,
                    ":er": &game.er,
                    ":hr": &game.hr,
                    ":bb": &game.bb,
                    ":ibb": &game.ibb,
                    ":so": &game.so,
                    ":wp": &game.wp,
                    ":bk": &game.bk,
                    ":hbp": &game.hbp,
                    ":gb": &game.gb,
                    ":fb": &game.fb,
                    ":p": &game.p,
                    ":s": &game.s,
                    ":decision": &game.decision,
                    ":era": &game.era,
                    ":fip": &game.fip,
                }
            )?;
        }

        Ok(())
    }

    fn load_team_gamelogs(&self, season: &str) -> Result<HashMap<TeamGameLogKey, TeamGameLogValue>, Box<dyn Error>> {
        let mut statement = self.conn.prepare(
            "SELECT game_id, date, visitor_team, visitor_team_game_number, home_team, home_team_game_number
            FROM games
            WHERE strftime('%Y', games.date) = :season"
        )?;
        let mut games = HashMap::new();
        let mut rows = statement.query(&[(":season", season)])?;
        while let Some(row) = rows.next()? {
            let game_id: String = row.get(0)?;
            let date: chrono::NaiveDate = row.get(1)?;
            let home_team = TeamGameLogKey {
                game_id: game_id.clone(),
                team_id: row.get(4)?,
            };
            let home_team_value = TeamGameLogValue {
                date,
                team_game_number: row.get(5)?,
            };
            games.insert(home_team, home_team_value);

            let visitor_team = TeamGameLogKey {
                game_id: game_id.clone(),
                team_id: row.get(2)?,
            };
            let visitor_team_value = TeamGameLogValue {
                date,
                team_game_number: row.get(3)?,
            };
            games.insert(visitor_team, visitor_team_value);
        }
        Ok(games)
    }

    fn load_season_boxscores(&self, season: &String) -> Result<ChildStdout, Box<dyn Error>> {
        let season_dir = self.retrosheet_dir.join(season);
        let mut cwbox = Command::new("cwbox");
        cwbox.args(["-q", "-y", season, "-X"]).current_dir(&season_dir);
        cwbox.args(find_boxscore_files(&season_dir)?);
        let command = cwbox.stdin(Stdio::null()).stdout(Stdio::piped());
        match command.spawn() {
            Ok(mut child) => {
                let stdout = child.stdout.take().expect("cwbox stdout handle not available");
                Ok(stdout)
            }
            Err(err) => {
                Err(Box::new(err))
            }
        }
    }

    fn dated_gamelog_cmp<T: player::PlayerGamelog>(a: &DatedPlayerGamelogs<T>, b: &DatedPlayerGamelogs<T>) -> cmp::Ordering {
        let player_cmp = a.0.player_id().cmp(b.0.player_id());
        match player_cmp {
            cmp::Ordering::Equal => {},
            _ => { return player_cmp; }
        }
        let date_cmp = a.1.cmp(&b.1);
        match date_cmp {
            cmp::Ordering::Equal => {},
            _ => { return date_cmp; }
        }
        a.0.team_id().cmp(b.0.team_id())
    }

    fn order_dated_gamelogs<T, U>(season: i32, chadwick_gl: Vec<T>, games: &HashMap<TeamGameLogKey, TeamGameLogValue>) -> Vec<DatedPlayerGamelogs<U>>
        where U: player::PlayerGamelog + std::convert::From<T>
    {
        let game_count = chadwick_gl.len();
        let default_value = TeamGameLogValue {
            date: chrono::NaiveDate::from_ymd_opt(season, 1, 1).unwrap(),
            team_game_number: 0,
        };
        let mut internal_gamelogs: Vec<DatedPlayerGamelogs<U>> = Vec::with_capacity(game_count);
        for gl in chadwick_gl.into_iter() {
            let mut new_gl: U = gl.into();
            let key = TeamGameLogKey {
                game_id: new_gl.game_id().to_string(),
                team_id: new_gl.team_id().to_string(),
            };
            // Need date and team game number.
            let value = games.get(&key).unwrap_or(&default_value);
            new_gl.set_team_game(value.team_game_number);
            internal_gamelogs.push((new_gl, value.date));
        }
        internal_gamelogs.sort_unstable_by(Self::dated_gamelog_cmp);
        internal_gamelogs
    }

    fn order_batting_gamelogs(season: i32, chadwick_gl: Vec<BattingGamelog>, games: &HashMap<TeamGameLogKey, TeamGameLogValue>) -> Vec<player::BattingGamelog> {
        let dated_gamelogs = Self::order_dated_gamelogs(season, chadwick_gl, games);

        let mut prev_player = String::with_capacity(10);
        let mut slash_line = BattingSlashLine::new();
        let mut season_game = 1;
        let mut gamelogs = Vec::with_capacity(dated_gamelogs.len());
        for entry in dated_gamelogs.into_iter() {
            let mut gl: player::BattingGamelog = entry.0;
            if prev_player == gl.player_id {
                slash_line.add_gamelog(&gl);
                let stats = slash_line.slash_line();
                gl.season_game = season_game;
                gl.avg = stats.0;
                gl.obp = stats.1;
                gl.slg = stats.2;
                season_game += 1;
            }
            else {
                prev_player.clear();
                prev_player.push_str(&gl.player_id);
                slash_line.clear();
                slash_line.add_gamelog(&gl);
                let stats = slash_line.slash_line();
                gl.season_game = 1;
                gl.avg = stats.0;
                gl.obp = stats.1;
                gl.slg = stats.2;
                season_game = 2;
            }
            gamelogs.push(gl);
        }
        gamelogs
    }

    fn order_fielding_gamelogs(season: i32, chadwick_gl: Vec<FieldingGamelog>, games: &HashMap<TeamGameLogKey, TeamGameLogValue>) -> Vec<player::FieldingGamelog> {
        let dated_gamelogs = Self::order_dated_gamelogs(season, chadwick_gl, games);

        let mut prev_player = String::with_capacity(10);
        let mut season_game = 1;
        let mut gamelogs = Vec::with_capacity(dated_gamelogs.len());
        for entry in dated_gamelogs.into_iter() {
            let mut gl: player::FieldingGamelog = entry.0;
            if prev_player == gl.player_id {
                gl.season_game = season_game;
                season_game += 1;
            }
            else {
                prev_player.clear();
                prev_player.push_str(&gl.player_id);
                gl.season_game = 1;
                season_game = 2;
            }
            gamelogs.push(gl);
        }
        gamelogs
    }

    fn order_pitching_gamelogs(season: i32, fip_constant: f32, chadwick_gl: Vec<PitchingGamelog>, games: &HashMap<TeamGameLogKey, TeamGameLogValue>) -> Vec<player::PitchingGamelog> {
        let dated_gamelogs = Self::order_dated_gamelogs(season, chadwick_gl, games);

        let mut prev_player = String::with_capacity(10);
        let mut pitcher_stats = PitcherStats::new_with_fip(fip_constant);
        let mut season_game = 1;
        let mut gamelogs = Vec::with_capacity(dated_gamelogs.len());
        for entry in dated_gamelogs.into_iter() {
            let mut gl: player::PitchingGamelog = entry.0;
            if prev_player == gl.player_id {
                pitcher_stats.add_gamelog(&gl);
                gl.season_game = season_game;
                gl.era = pitcher_stats.era();
                gl.fip = pitcher_stats.fip();
                season_game += 1;
            }
            else {
                prev_player.clear();
                prev_player.push_str(&gl.player_id);
                pitcher_stats.clear();
                pitcher_stats.add_gamelog(&gl);
                gl.season_game = 1;
                gl.era = pitcher_stats.era();
                gl.fip = pitcher_stats.fip();
                season_game = 2;
            }
            gamelogs.push(gl);
        }
        gamelogs
    }

    fn load(&mut self, seasons: &Vec<String>, initialize: bool) -> Result<(), Box<dyn Error>> {
        if initialize {
            println!("Creating gamelog tables");
            self.create_batting_gamelogs_table()?;
            self.create_fielding_gamelogs_table()?;
            self.create_pitching_gamelogs_table()?;
        }

        for season in seasons {
            // Load team gamelogs.
            println!("Loading team game logs from {} season", season);
            let team_games = self.load_team_gamelogs(season)?;

            println!("Loading player game logs from {} season", season);
            let stdout = self.load_season_boxscores(season)?;
            let (batting_gamelogs, fielding_gamelogs, pitching_gamelogs) = gamelogs_from_boxscores(io::BufReader::new(stdout));

            // Transform Chadwick gamelogs into internal version for the database and sort to allow
            // marking which game number in the season this is for a player.
            let season_numeric = season.parse::<u16>()?;
            let fip_constant = get_fip_constant(self.conn, season_numeric)?.unwrap_or_default();
            let batting_gamelogs = Self::order_batting_gamelogs(season_numeric.into(), batting_gamelogs, &team_games);
            let fielding_gamelogs = Self::order_fielding_gamelogs(season_numeric.into(), fielding_gamelogs, &team_games);
            let pitching_gamelogs = Self::order_pitching_gamelogs(season_numeric.into(), fip_constant, pitching_gamelogs, &team_games);

            let tx = self.conn.transaction().expect("Could not create transaction");
            Self::insert_batting_gamelogs(&tx, &batting_gamelogs)?;
            Self::insert_fielding_gamelogs(&tx, &fielding_gamelogs)?;
            Self::insert_pitching_gamelogs(&tx, &pitching_gamelogs)?;
            tx.commit().expect("Failed to commit transaction");
        }

        if initialize {
            println!("Creating gamelog indexes");
            self.conn.execute_batch(
                "
                CREATE INDEX batting_gamelogs_player_idx ON batting_gamelogs (player_id);
                CREATE INDEX batting_gamelogs_game_idx ON batting_gamelogs (game_id);
                CREATE INDEX batting_gamelogs_team_idx ON batting_gamelogs (team_id);
                CREATE INDEX fielding_gamelogs_player_idx ON fielding_gamelogs (player_id);
                CREATE INDEX fielding_gamelogs_game_idx ON fielding_gamelogs (game_id);
                CREATE INDEX fielding_gamelogs_team_idx ON fielding_gamelogs (team_id);
                CREATE INDEX pitching_gamelogs_player_idx ON pitching_gamelogs (player_id);
                CREATE INDEX pitching_gamelogs_game_idx ON pitching_gamelogs (game_id);
                CREATE INDEX pitching_gamelogs_team_idx ON pitching_gamelogs (team_id);
                "
            )?;
        }

        Ok(())
    }

    fn order_career_games(&mut self, seasons: &[String]) -> Result<(), Box<dyn Error>> {
        if seasons.is_empty() {
            return Ok(());
        }
        // Convert the seasons to integers to ensure a consistent sort.
        let mut seasons: Vec<u16> = seasons.iter().flat_map(|s| s.parse::<u16>()).collect();
        seasons.sort_unstable();
        let oldest = seasons.first().expect("Expected a season");
        let newest = seasons.last().expect("Expected a season");
        let start_date = format!("{}-01-01", oldest);
        let end_date = format!("{}-12-31", newest);

        self.order_table("batting_gamelogs", &start_date, &end_date)?;
        self.order_table("fielding_gamelogs", &start_date, &end_date)?;
        self.order_table("pitching_gamelogs", &start_date, &end_date)?;

        Ok(())
    }

    fn order_table(&mut self, table: &str, start_date: &str, end_date: &str) -> Result<(), Box<dyn Error>> {
        let tx = self.conn.transaction()?;
        // Find all players affected by these seasons.
        let sql = format!(
            "SELECT DISTINCT(player_id)
            FROM {} AS gl JOIN games ON gl.game_id = games.game_id
            WHERE games.date BETWEEN :start AND :end",
            table);
        let mut statement = tx.prepare(&sql)?;
        let players: Vec<String> = statement.query_map(
            &[(":start", start_date), (":end", end_date)],
            |row| row.get(0)
        )?
            .flatten()
            .collect();
        drop(statement);

        // Update those players.
        println!("Ordering table {} with {} players", table, players.len());

        let select_player = format!(
            "SELECT gl.game_id, games.date
            FROM {} AS gl JOIN games ON gl.game_id = games.game_id
            WHERE player_id = :player",
            table);
        let update_player = format!(
            "UPDATE {} SET career_game = :game_number
            WHERE player_id = :player AND game_id = :game_id",
            table);
        let mut select_statement = tx.prepare(&select_player)?;
        let mut update_statement = tx.prepare(&update_player)?;
        let mut games_updated = 0;
        for player_id in &players {
            let mut games: Vec<CareerGame> = select_statement.query_map(
                &[(":player", player_id)],
                |row| Ok(CareerGame {
                    player_id: player_id.clone(),
                    game_id: row.get(0)?,
                    date: row.get(1)?,
                })
            )?
                .flatten()
                .collect();
            games.sort_unstable();

            // The fielding game log table will have a row for every position a player plays in a
            // game. Each career game should only increment for different games so update all
            // matching (player, game) options (even if they have a different team) and skip to the
            // next unique game.
            let mut game_number = 1;
            let mut last_game = "";
            for game in games.iter() {
                if game.game_id == last_game {
                    continue;
                }
                update_statement.execute(
                    named_params! {
                        ":game_number": game_number,
                        ":player": &player_id,
                        ":game_id": &game.game_id,
                    }
                )?;
                game_number += 1;
                games_updated += 1;
                last_game = game.game_id.as_str();
            }
        }
        drop(select_statement);
        drop(update_statement);
        tx.commit().expect("Failed to commit transaction");
        println!("Updated {} games for {}", games_updated, table);

        Ok(())
    }
}


impl BattingSlashLine {
    fn new() -> Self {
        Self {
            ab: 0,
            h: 0,
            tb: 0,
            bb: 0,
            hbp: 0,
            sf: 0,
        }
    }

    fn clear(&mut self) {
        self.h = 0;
        self.ab = 0;
        self.h = 0;
        self.tb = 0;
        self.bb = 0;
        self.hbp = 0;
        self.sf = 0;
    }

    fn add_gamelog(&mut self, gamelog: &player::BattingGamelog) {
        let h: u16 = gamelog.h.into();
        let ab: u16 = gamelog.ab.into();
        self.h += h;
        self.ab += ab;
        let d: u16 = gamelog.d.into();
        let t: u16 = gamelog.t.into();
        let hr: u16 = gamelog.hr.into();
        // The hits field includes extra-base hits so the game total for each stat includes the
        // number of bases beyond a single.
        self.tb += h + d + t * 2 + hr * 3;
        self.bb += gamelog.bb;
        self.hbp += gamelog.hbp;
        self.sf += gamelog.sf;
    }

    fn slash_line(&self) -> (f32, f32, f32) {
        let h = self.h as f32;
        let ab = self.ab as f32;
        let tb = self.tb as f32;
        let bb = self.bb as f32;
        let hbp = self.hbp as f32;
        let sf = self.sf as f32;

        let avg = if ab > 0.0 {
            h / ab
        }
        else {
            f32::NAN
        };

        let obp = if (ab + bb + hbp + sf) > 0.0 {
            (h + bb + hbp) / (ab + bb + hbp + sf)
        }
        else {
            f32::NAN
        };

        let slg = if ab > 0.0 {
            tb / ab
        }
        else {
            f32::NAN
        };

        (avg, obp, slg)
    }
}


impl PitcherStats {
    /*
    fn new() -> Self {
        Self::new_with_fip(3.20)
    }
    */

    fn new_with_fip(fip_constant: f32) -> Self {
        Self {
            ipouts: 0,
            er: 0,
            hr: 0,
            bb: 0,
            hbp: 0,
            so: 0,
            fip_constant,
        }
    }

    fn clear(&mut self) {
        self.ipouts = 0;
        self.er = 0;
        self.hr = 0;
        self.bb = 0;
        self.hbp = 0;
        self.so = 0;
    }

    fn add_team_gamelog(&mut self, gamelog: &games::GameLog) {
        macro_rules! unwrap_retro_option {
            ($field:expr) => {
                match $field {
                    games::RetrosheetOption::Some(v) => v,
                    _ => return,
                }
            }
        }

        let ipouts: u32 = match gamelog.number_of_outs {
            Some(outs) => outs.into(),
            None => return,
        };
        self.ipouts += ipouts;
        let v_i_er = unwrap_retro_option!(gamelog.visitor_individual_earned_runs);
        let h_i_er = unwrap_retro_option!(gamelog.home_individual_earned_runs);
        let er: u16 = (v_i_er + h_i_er).into();
        self.er += er;
        let v_hr = unwrap_retro_option!(gamelog.visitor_homeruns);
        let h_hr = unwrap_retro_option!(gamelog.home_homeruns);
        let hr: u16 = (v_hr + h_hr).into();
        self.hr += hr;
        let v_bb = unwrap_retro_option!(gamelog.visitor_walks);
        let h_bb = unwrap_retro_option!(gamelog.home_walks);
        let bb: u16 = (v_bb + h_bb).into();
        self.bb += bb;
        let v_hbp = unwrap_retro_option!(gamelog.visitor_hbp);
        let h_hbp = unwrap_retro_option!(gamelog.home_hbp);
        let hbp: u16 = (v_hbp + h_hbp).into();
        self.hbp += hbp;
        let v_so = unwrap_retro_option!(gamelog.visitor_strikeouts);
        let h_so = unwrap_retro_option!(gamelog.home_strikeouts);
        let so: u16 = (v_so + h_so).into();
        self.so += so;
    }

    fn add_gamelog(&mut self, gamelog: &player::PitchingGamelog) {
        let ipouts: u32 = gamelog.ipouts.into();
        self.ipouts += ipouts;
        let er: u16 = gamelog.er.into();
        self.er += er;
        let hr: u16 = gamelog.hr.into();
        self.hr += hr;
        let bb: u16 = gamelog.bb.into();
        self.bb += bb;
        let hbp: u16 = gamelog.hbp.into();
        self.hbp += hbp;
        let so: u16 = gamelog.so.into();
        self.so += so;
    }

    fn era(&self) -> f32 {
        let er = self.er as f32;
        let outs = self.ipouts as f32;
        if outs > 0.0 {
            er * 27.0 / outs
        }
        else if er > 0.0 {
            f32::INFINITY
        }
        else {
            f32::NAN
        }
    }

    fn fip(&self) -> f32 {
        let hr = self.hr as f32;
        let bb = self.bb as f32;
        let hbp = self.hbp as f32;
        let so = self.so as f32;
        let outs = self.ipouts as f32;

        if outs > 0.0 {
            (13.0 * hr + 3.0 * (bb + hbp) - 2.0 * so) / (outs / 3.0) + self.fip_constant
        }
        else if hr > 0.0 || bb > 0.0 || hbp > 0.0 || so > 0.0 {
            f32::INFINITY
        }
        else {
            f32::NAN
        }
    }
}

impl cmp::PartialOrd for CareerGame {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::Ord for CareerGame {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        let cmp = self.date.cmp(&other.date);
        match cmp {
            cmp::Ordering::Equal => {},
            _ => return cmp,
        };

        let cmp = self.player_id.cmp(&other.player_id);
        match cmp {
            cmp::Ordering::Equal => {},
            _ => return cmp,
        };

        self.game_id.cmp(&other.game_id)
    }
}


fn find_boxscore_files(season_dir: &path::Path) -> Result<Vec<String>, Box<dyn Error>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(season_dir)? {
        let entry = entry?;
        let path = entry.path();
        let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("");
        match extension {
            // Include only the boxscore files because they're simpler and enough to get the stats
            // for every player.
            "EBA" | "EBN" | "EBR" => {
                if let Some(path_str) = path.to_str() {
                    files.push(path_str.to_string());
                }
            }
            _ => {}
        }
    }

    Ok(files)
}


fn create_internal_tables(conn: &mut Connection) {
    if let Ok(table_exists) = conn.table_exists(Some("main"), "guts") && !table_exists {
        let res = conn.execute(include_str!("../sql/create_guts.sql"), ());
        if let Err(err) = res {
            eprintln!("Creation of guts table failed: {}", err);
        }
    }
}


fn get_fip_constant(conn: &mut Connection, season: u16) -> Result<Option<f32>, Box<dyn Error>> {
    let fip = conn.query_one(
         "SELECT fip_constant FROM guts WHERE season = :season",
        &[(":season", &season)],
        |row| row.get(0)
    )?;

    Ok(fip)
}


fn update_fip_constant(tx: &Transaction, guts: &Guts) -> Result<(), Box<dyn Error>> {
    let insert_sql = String::from(
        "INSERT INTO guts (season, fip_constant) VALUES (:season, :fip_constant)
         ON CONFLICT (season) DO UPDATE SET fip_constant=:fip_constant"
    );

    let mut insert = tx.prepare(&insert_sql)?;
    insert.execute(
        named_params! {
            ":season": &guts.season,
            ":fip_constant": &guts.fip_constant,
        }
    )?;

    Ok(())
}


fn load_people_file(people_csv: &path::Path) -> Result<Vec<Person>, Box<dyn Error>> {
    // Largest people file is a bit over 32k so give some room to grow.
    let mut people = Vec::with_capacity(34000);
    let file = fs::File::open(people_csv)?;
    let mut reader = ReaderBuilder::new().from_reader(file);
    for person in reader.deserialize().flatten() {
        people.push(person);
    }
    Ok(people)
}


fn load_people_files(conn: &mut Connection, register_dir: &path::Path, initialize: bool) {
    let data_dir = register_dir.join("data");

    println!("Preparing to load register");

    if initialize {
        if let Err(err) = conn.execute("DROP TABLE IF EXISTS people", ()) {
            eprintln!("Initialize of people table failed: {}", err);
            return;
        }
        let res = conn.execute(include_str!("../sql/create_people.sql"), ());
        if let Err(err) = res {
            eprintln!("Creation of people table failed: {}", err);
            return;
        }
    }

    let mut paths = Vec::with_capacity(16);
    for entry in data_dir.read_dir().expect("Failed to read register data directory").flatten() {
        let file_name = entry.file_name().into_string();
        if let Ok(file_name) = file_name && file_name.starts_with("people") {
            paths.push(entry.path());
        }
    }

    let insert_sql = String::from(
        "INSERT INTO people VALUES ( :key_person, :key_uuid, :key_mlbam, :key_retro, :key_bbref,
        :key_bbref_minors, :key_fangraphs, :key_npb, :key_sr_nfl, :key_sr_nba, :key_sr_nhl,
        :key_wikidata, :name_last, :name_first, :name_given, :name_suffix, :name_matrilineal,
        :name_nick, :birth_year, :birth_month, :birth_day, :death_year, :death_month, :death_day,
        :pro_played_first, :pro_played_last, :mlb_played_first, :mlb_played_last,
        :col_played_first, :col_played_last, :pro_managed_first, :pro_managed_last,
        :mlb_managed_first, :mlb_managed_last, :col_managed_first, :col_managed_last,
        :pro_umpired_first, :pro_umpired_last, :mlb_umpired_first, :mlb_umpired_last)");

    let tx = conn.transaction().expect("Could not create transaction");
    let mut insert = tx.prepare(&insert_sql).expect("Could not prepare INSERT");
    let mut people_loaded = 0;
    for path in &paths {
        for person in load_people_file(path).expect("Couldn't load people CSV file") {
            insert.execute(
                named_params! {
                    ":key_person": &person.key_person,
                    ":key_uuid": &person.key_uuid,
                    ":key_mlbam": &person.key_mlbam,
                    ":key_retro": &person.key_retro,
                    ":key_bbref": &person.key_bbref,
                    ":key_bbref_minors": &person.key_bbref_minors,
                    ":key_fangraphs": &person.key_fangraphs,
                    ":key_npb": &person.key_npb,
                    ":key_sr_nfl": &person.key_sr_nfl,
                    ":key_sr_nba": &person.key_sr_nba,
                    ":key_sr_nhl": &person.key_sr_nhl,
                    ":key_wikidata": &person.key_wikidata,
                    ":name_last": &person.name_last,
                    ":name_first": &person.name_first,
                    ":name_given": &person.name_given,
                    ":name_suffix": &person.name_suffix,
                    ":name_matrilineal": &person.name_matrilineal,
                    ":name_nick": &person.name_nick,
                    ":birth_year": &person.birth_year,
                    ":birth_month": &person.birth_month,
                    ":birth_day": &person.birth_day,
                    ":death_year": &person.death_year,
                    ":death_month": &person.death_month,
                    ":death_day": &person.death_day,
                    ":pro_played_first": &person.pro_played_first,
                    ":pro_played_last": &person.pro_played_last,
                    ":mlb_played_first": &person.mlb_played_first,
                    ":mlb_played_last": &person.mlb_played_last,
                    ":col_played_first": &person.col_played_first,
                    ":col_played_last": &person.col_played_last,
                    ":pro_managed_first": &person.pro_managed_first,
                    ":pro_managed_last": &person.pro_managed_last,
                    ":mlb_managed_first": &person.mlb_managed_first,
                    ":mlb_managed_last": &person.mlb_managed_last,
                    ":col_managed_first": &person.col_managed_first,
                    ":col_managed_last": &person.col_managed_last,
                    ":pro_umpired_first": &person.pro_umpired_first,
                    ":pro_umpired_last": &person.pro_umpired_last,
                    ":mlb_umpired_first": &person.mlb_umpired_first,
                    ":mlb_umpired_last": &person.mlb_umpired_last
                }
            ).expect("Failed to insert into people table");
            people_loaded += 1;
        }
    }

    drop(insert);

    tx.commit().expect("Failed to commit transaction");


    // Create index after importing data when initializing.
    if initialize {
        conn.execute_batch(
            "
            CREATE INDEX people_retro_idx ON people (key_retro);
            CREATE INDEX people_bbref_idx ON people (key_bbref);
            CREATE INDEX people_fangraphs_idx ON people (key_fangraphs);
            "
        ).expect("Failed to create people indexes");
    }

    println!("Loaded {} register entries", people_loaded);
}


fn run() -> Result<(), Box<dyn Error>> {
    let args = DatabaseArgs::parse();

    let database = args.database.unwrap_or(path::PathBuf::from("database.db"));
    let mut connection = Connection::open(database)?;
    connection.pragma_update(None, "temp_store", "memory")?;

    let seasons = args.seasons;

    if let Some(register_path) = args.register_dir {
        load_people_files(&mut connection, &register_path, args.init);
    }

    create_internal_tables(&mut connection);

    if args.games {
        if let Some(ref retrosheet_dir) = args.retrosheet_dir {
            let mut game_loader = GameLoader::new(&mut connection, retrosheet_dir.to_owned());
            game_loader.load(&seasons, args.init)?;
        }
        else {
            eprintln!("Cannot load games without retrosheet directory.");
        }
    }

    if args.gamelogs {
        if let Some(ref retrosheet_dir) = args.retrosheet_dir {
            let mut gamelogs = PlayerGamelogs::new(&mut connection, retrosheet_dir.to_owned());
            gamelogs.load(&seasons, args.init)?;
            if args.count_career_games {
                gamelogs.order_career_games(&seasons)?;
            }
        }
        else {
            eprintln!("Cannot load gamelogs without retrosheet directory.");
        }
    }

    // If initializing then tables changed and indexes were created. Run PRAGMA optimize to have
    // sqlite optimize its statistics.
    if args.init {
        println!("Optimizing database");
        connection.pragma_update(None, "optimize", "")?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    run()
}
