use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub(crate) struct Summoner {
    pub account_id: String,
    pub profile_icon_id: u32,
    pub revision_date: u64,
    pub name: String,
    pub id: String,
    pub puuid: String,
    pub summoner_level: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub(crate) struct Match {
    pub metadata: Metadata,
    pub info: Info,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub(crate) struct Metadata {
    pub data_version: String,
    pub match_id: String,
    pub participants: Vec<String>,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub(crate) struct Info {
    pub game_creation: u64,
    pub game_duration: u64,
    pub game_end_timestamp: u64,
    pub game_id: u64,
    pub game_mode: String,
    pub game_name: String,
    pub game_start_timestamp: u64,
    pub game_type: String,
    pub game_version: String,
    pub map_id: u32,
    pub participants: Vec<Participant>,
    pub platform_id: String,
    pub queue_id: u32,
    pub teams: Vec<Team>,
    pub tournament_code: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub(crate) struct Participant {
    pub assists: u32,
    pub baron_kills: u32,
    pub bounty_level: u32,
    pub champ_experience: u32,
    pub champ_level: u32,
    pub champion_id: u32,
    pub champion_name: String,
    pub champion_transform: u32,
    pub consumables_purchased: u32,
    pub damage_dealt_to_buildings: u32,
    pub damage_dealt_to_objectives: u32,
    pub damage_dealt_to_turrets: u32,
    pub damage_self_mitigated: u32,
    pub deaths: u32,
    pub detector_wards_placed: u32,
    pub double_kills: u32,
    pub dragon_kills: u32,
    pub first_blood_assist: bool,
    pub first_blood_kill: bool,
    pub first_tower_assist: bool,
    pub first_tower_kill: bool,
    pub game_ended_in_early_surrender: bool,
    pub game_ended_in_surrender: bool,
    pub gold_earned: u32,
    pub gold_spent: u32,
    pub individual_position: String,
    pub inhibitor_kills: u32,
    pub inhibitor_takedowns: u32,
    pub inhibitors_lost: u32,
    pub item0: u32,
    pub item1: u32,
    pub item2: u32,
    pub item3: u32,
    pub item4: u32,
    pub item5: u32,
    pub item6: u32,
    pub items_purchased: u32,
    pub killing_sprees: u32,
    pub kills: u32,
    pub lane: String,
    pub largest_critical_strike: u32,
    pub largest_killing_spree: u32,
    pub largest_multi_kill: u32,
    pub longest_time_spent_living: u32,
    pub magic_damage_dealt: u32,
    pub magic_damage_dealt_to_champions: u32,
    pub magic_damage_taken: u32,
    pub neutral_minions_killed: u32,
    pub nexus_kills: u32,
    pub nexus_takedowns: u32,
    pub nexus_lost: u32,
    pub objectives_stolen: u32,
    pub objectives_stolen_assists: u32,
    pub participant_id: u32,
    pub penta_kills: u32,
    pub perks: Perks,
    pub physical_damage_dealt: u32,
    pub physical_damage_dealt_to_champions: u32,
    pub physical_damage_taken: u32,
    pub profile_icon: u32,
    pub puuid: String,
    pub quadra_kills: u32,
    pub riot_id_name: Option<String>,
    pub riot_id_tagline: String,
    pub role: String,
    pub sight_wards_bought_in_game: u32,
    pub spell1_casts: u32,
    pub spell2_casts: u32,
    pub spell3_casts: u32,
    pub spell4_casts: u32,
    pub summoner1_casts: u32,
    pub summoner1_id: u32,
    pub summoner2_casts: u32,
    pub summoner2_id: u32,
    pub summoner_id: String,
    pub summoner_level: u32,
    pub summoner_name: String,
    pub team_early_surrendered: bool,
    pub team_id: u32,
    pub team_position: String,
    #[serde(rename = "timeCCingOthers")]
    pub time_ccing_others: u32,
    pub time_played: u32,
    pub total_damage_dealt: u32,
    pub total_damage_dealt_to_champions: u32,
    pub total_damage_shielded_on_teammates: u32,
    pub total_damage_taken: u32,
    pub total_heal: u32,
    pub total_heals_on_teammates: u32,
    pub total_minions_killed: u32,
    #[serde(rename = "totalTimeCCDealt")]
    pub total_time_cc_dealt: u32,
    pub total_time_spent_dead: u32,
    pub total_units_healed: u32,
    pub triple_kills: u32,
    pub true_damage_dealt: u32,
    pub true_damage_dealt_to_champions: u32,
    pub true_damage_taken: u32,
    pub turret_kills: u32,
    pub turret_takedowns: u32,
    pub turrets_lost: u32,
    pub unreal_kills: u32,
    pub vision_score: u32,
    pub vision_wards_bought_in_game: u32,
    pub wards_killed: u32,
    pub wards_placed: u32,
    pub win: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub(crate) struct Team {
    pub bans: Vec<Ban>,
    pub objectives: Objectives,
    pub team_id: u32,
    pub win: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub(crate) struct Perks {
    pub stat_perks: PerkStats,
    pub styles: Vec<PerkStyle>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub(crate) struct PerkStats {
    pub defense: u32,
    pub flex: u32,
    pub offense: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub(crate) struct PerkStyle {
    pub description: String,
    pub selections: Vec<PerkStyleSelection>,
    pub style: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub(crate) struct PerkStyleSelection {
    pub perk: u32,
    pub var1: u32,
    pub var2: u32,
    pub var3: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub(crate) struct Ban {
    pub champion_id: u32,
    pub pick_turn: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub(crate) struct Objectives {
    pub baron: Objective,
    pub champion: Objective,
    pub dragon: Objective,
    pub inhibitor: Objective,
    pub rift_herald: Objective,
    pub tower: Objective,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub(crate) struct Objective {
    pub first: bool,
    pub kills: u32,
}
