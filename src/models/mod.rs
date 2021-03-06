
#[derive(Debug, Copy, Clone)]
pub enum SkillLevel {
    Normal,
    High,
    VeryHigh,
}

impl From<SkillLevel> for i32 {
    fn from(skill_level: SkillLevel) -> i32 {
        match skill_level {
            SkillLevel::Normal => 1,
            SkillLevel::High => 2,
            SkillLevel::VeryHigh => 3,
        }
    }
}

#[derive(Default, Debug)]
pub struct MatchHistoryOptions {
    pub hero_id: Option<i32>, 
    pub skill: Option<SkillLevel>,
    /// Minimum date in UNIX time.
    pub date_min: Option<u64>,
    /// Maximum date in UNIX time.
    pub date_max: Option<u64>,
    pub account_id: Option<u32>,
    pub league_id: Option<u32>,
    pub start_at_match_id: Option<u64>,
    /// Number of matches to be requested. The maximum is 25.
    pub matches_requested: Option<i32>,
}

#[derive(Deserialize, Debug)]
pub struct MatchHistoryPlayer {
    /// This is ```None``` if the player is a bot.
    pub account_id: Option<i64>,
    pub player_slot: i32,
    pub hero_id: i32,
}

#[derive(Deserialize, Debug)]
pub struct Match {
    pub match_id: i64,
    pub match_seq_num: i64,
    pub start_time: i64,
    pub lobby_type: i32,
    pub radiant_team_id: i32,
    pub dire_team_id: i32,
    pub players: Vec<MatchHistoryPlayer>,
}

#[derive(Deserialize, Debug)]
pub struct MatchHistory {
    pub status: i32,
    pub num_results: i32,
    pub total_results: i32,
    pub results_remaining: i32,
    pub matches: Vec<Match>,
}

#[derive(Deserialize, Debug)]
pub struct MatchHistoryResult {
    pub result: MatchHistory,
}

#[derive(Deserialize, Debug)]
pub struct MatchDetails {
    pub players: Vec<MatchDetailsPlayer>,
    pub radiant_win: bool,
    pub duration: i64,
    pub pre_game_duration: i32,
    pub start_time: u64,
    pub match_id: u64,
    pub match_seq_num: u64,
    pub tower_status_radiant: i32,
    pub tower_status_dire: i32,
    pub barracks_status_radiant: i32,
    pub barracks_status_dire: i32,
    pub cluster: i32,
    pub first_blood_time: i32,
    pub lobby_type: i32,
    pub human_players: i32,
    pub leagueid: i64,
    pub positive_votes: i32,
    pub negative_votes: i32,
    pub game_mode: i32,
    pub flags: i32,
    pub engine: i32,
    pub radiant_score: i32,
    pub dire_score: i32,
}

#[derive(Deserialize, Debug)]
pub struct AbilityUpgrade {
    pub ability: i32,
    pub time: i32,
    pub level: i32,
}

#[derive(Deserialize, Debug)]
pub struct ApiErrorResult {
    pub result: ApiError,
}

/// Error returned by the Dota 2 web API
#[derive(Deserialize, Debug)]
pub struct ApiError {
    pub error: String,
}

#[derive(Deserialize, Debug)]
pub struct MatchDetailsPlayer {
    pub account_id: u64,
    pub player_slot: i32,
    pub hero_id: i32,
    pub item_0: i32,
    pub item_1: i32,
    pub item_2: i32,
    pub item_3: i32,
    pub item_4: i32,
    pub item_5: i32,
    pub backpack_0: i32,
    pub backpack_1: i32,
    pub backpack_2: i32,
    pub kills: i32,
    pub deaths: i32,
    pub assists: i32,
    pub leaver_status: i32,
    pub last_hits: i32,
    pub denies: i32,
    pub gold_per_min: i32,
    pub xp_per_min: i32,
    pub level: i32,
    pub hero_damage: i32,
    pub tower_damage: i32,
    pub hero_healing: i32,
    pub gold: i32,
    pub gold_spent: i32,
    pub scaled_hero_damage: i32,
    pub scaled_tower_damage: i32,
    pub scaled_hero_healing: i32,
    pub ability_upgrades: Vec<AbilityUpgrade>,
}

#[derive(Deserialize, Debug)]
pub struct MatchDetailsResult {
    pub result: MatchDetails,
}

#[derive(Deserialize, Debug)]
pub struct TopLiveGameList {
    pub game_list: Vec<TopLiveGame>,
}

#[derive(Deserialize, Debug)]
pub struct TopLiveGame {
    pub activate_time: i32,
    pub deactivate_time: i32,
    pub server_steam_id: String,
    pub lobby_id: String,
    pub league_id: i32,
    pub lobby_type: i32,
    pub game_time: i32,
    pub delay: i32,
    pub spectators: i32,
    pub game_mode: i32,
    pub average_mmr: i32,
    pub team_name_radiant: Option<String>,
    pub team_name_dire: Option<String>,
    pub team_logo_radiant: Option<String>,
    pub team_logo_dire: Option<String>,
    pub sort_score: i32,
    pub last_update_time: i64,
    pub radiant_lead: i32,
    pub radiant_score: i32,
    pub dire_score: i32,
    pub players: Option<Vec<TopLiveGamePlayer>>,
    pub building_state: i32,
}

#[derive(Deserialize, Debug)]
pub struct TopLiveGamePlayer {
    pub account_id: i32,
    pub hero_id: i32,
}
