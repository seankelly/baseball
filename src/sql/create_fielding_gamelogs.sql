CREATE TABLE fielding_gamelogs (
    player_id TEXT NOT NULL,
    game_id TEXT NOT NULL,
    team_id TEXT NOT NULL,
    career_game INTEGER,
    season_game INTEGER,
    team_game INTEGER,
    pos INTEGER,
    gs BOOLEAN,
    o INTEGER,
    po INTEGER,
    tc INTEGER,
    a INTEGER,
    e INTEGER,
    dp INTEGER,
    tp INTEGER,
    pp INTEGER,
    ci INTEGER
)
