//! Official Riot Games API regional routing, platform architecture, and endpoint models.
//!
//! Maps global League of Legends server platforms to Riot API regional clusters
//! (americas, asia, europe, sea) and exact platform IDs (e.g. ME1, SG2, NA1, EUW1).

use serde::{Deserialize, Serialize};

/// Global League of Legends server platforms and regions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RiotRegion {
    /// Middle East (ME / ME1) - Launched June 2024
    ME,
    /// Southeast Asia general hub (SEA / SG2)
    SEA,
    /// Singapore, Malaysia, Indonesia (SG / SG2)
    SG,
    /// Philippines (PH / PH2)
    PH,
    /// Vietnam (VN / VN2)
    VN,
    /// Taiwan, Hong Kong, Macau (TW / TW2)
    TW,
    /// Thailand (TH / TH2)
    TH,
    /// Europe West (EUW / EUW1)
    EUW,
    /// Europe Nordic & East (EUNE / EUN1)
    EUNE,
    /// North America (NA / NA1)
    NA,
    /// Korea (KR)
    KR,
    /// Japan (JP / JP1)
    JP,
    /// Brazil (BR / BR1)
    BR,
    /// Latin America North (LAN / LA1)
    LAN,
    /// Latin America South (LAS / LA2)
    LAS,
    /// Oceania (OCE / OC1)
    OCE,
    /// Turkey (TR / TR1)
    TR,
    /// Russia (RU / RU)
    RU,
    /// Public Beta Environment (PBE / PBE1)
    PBE,
}

impl RiotRegion {
    /// Array of all official supported server platforms.
    pub const ALL: [RiotRegion; 19] = [
        RiotRegion::ME,
        RiotRegion::SEA,
        RiotRegion::SG,
        RiotRegion::PH,
        RiotRegion::VN,
        RiotRegion::TW,
        RiotRegion::TH,
        RiotRegion::EUW,
        RiotRegion::EUNE,
        RiotRegion::NA,
        RiotRegion::KR,
        RiotRegion::JP,
        RiotRegion::BR,
        RiotRegion::LAN,
        RiotRegion::LAS,
        RiotRegion::OCE,
        RiotRegion::TR,
        RiotRegion::RU,
        RiotRegion::PBE,
    ];

    /// Short standard region code (e.g. "ME", "SEA", "EUW").
    pub fn code(&self) -> &'static str {
        match self {
            RiotRegion::ME => "ME",
            RiotRegion::SEA => "SEA",
            RiotRegion::SG => "SG",
            RiotRegion::PH => "PH",
            RiotRegion::VN => "VN",
            RiotRegion::TW => "TW",
            RiotRegion::TH => "TH",
            RiotRegion::EUW => "EUW",
            RiotRegion::EUNE => "EUNE",
            RiotRegion::NA => "NA",
            RiotRegion::KR => "KR",
            RiotRegion::JP => "JP",
            RiotRegion::BR => "BR",
            RiotRegion::LAN => "LAN",
            RiotRegion::LAS => "LAS",
            RiotRegion::OCE => "OCE",
            RiotRegion::TR => "TR",
            RiotRegion::RU => "RU",
            RiotRegion::PBE => "PBE",
        }
    }

    /// User-friendly display label with official platform ID notation.
    pub fn display_name(&self) -> &'static str {
        match self {
            RiotRegion::ME => "Middle East (ME / ME1)",
            RiotRegion::SEA => "Southeast Asia (SEA / SG2)",
            RiotRegion::SG => "Singapore / Malaysia / Indonesia (SG2)",
            RiotRegion::PH => "Philippines (PH2)",
            RiotRegion::VN => "Vietnam (VN2)",
            RiotRegion::TW => "Taiwan / Hong Kong / Macau (TW2)",
            RiotRegion::TH => "Thailand (TH2)",
            RiotRegion::EUW => "Europe West (EUW1)",
            RiotRegion::EUNE => "Europe Nordic & East (EUN1)",
            RiotRegion::NA => "North America (NA1)",
            RiotRegion::KR => "Korea (KR)",
            RiotRegion::JP => "Japan (JP1)",
            RiotRegion::BR => "Brazil (BR1)",
            RiotRegion::LAN => "Latin America North (LA1)",
            RiotRegion::LAS => "Latin America South (LA2)",
            RiotRegion::OCE => "Oceania (OC1)",
            RiotRegion::TR => "Turkey (TR1)",
            RiotRegion::RU => "Russia (RU)",
            RiotRegion::PBE => "Public Beta Environment (PBE1)",
        }
    }

    /// Official Riot Games Platform Routing ID (used for Summoner-V4, Champion-Mastery-V4, Spectator-V5).
    pub fn platform_id(&self) -> &'static str {
        match self {
            RiotRegion::ME => "me1",
            RiotRegion::SEA | RiotRegion::SG => "sg2",
            RiotRegion::PH => "ph2",
            RiotRegion::VN => "vn2",
            RiotRegion::TW => "tw2",
            RiotRegion::TH => "th2",
            RiotRegion::EUW => "euw1",
            RiotRegion::EUNE => "eun1",
            RiotRegion::NA => "na1",
            RiotRegion::KR => "kr",
            RiotRegion::JP => "jp1",
            RiotRegion::BR => "br1",
            RiotRegion::LAN => "la1",
            RiotRegion::LAS => "la2",
            RiotRegion::OCE => "oc1",
            RiotRegion::TR => "tr1",
            RiotRegion::RU => "ru",
            RiotRegion::PBE => "pbe1",
        }
    }

    /// Official Riot Games Regional Routing cluster (used for Account-V1, Match-V5).
    pub fn regional_routing(&self) -> &'static str {
        match self {
            // Middle East and Europe share the europe cluster
            RiotRegion::ME
            | RiotRegion::EUW
            | RiotRegion::EUNE
            | RiotRegion::TR
            | RiotRegion::RU => "europe",
            // Southeast Asia servers route to sea
            RiotRegion::SEA
            | RiotRegion::SG
            | RiotRegion::PH
            | RiotRegion::VN
            | RiotRegion::TW
            | RiotRegion::TH => "sea",
            // Pan-Americas
            RiotRegion::NA
            | RiotRegion::BR
            | RiotRegion::LAN
            | RiotRegion::LAS
            | RiotRegion::OCE
            | RiotRegion::PBE => "americas",
            // East Asia
            RiotRegion::KR | RiotRegion::JP => "asia",
        }
    }

    /// Parse a region code or platform ID string (case-insensitive).
    pub fn from_code(code: &str) -> Option<Self> {
        let clean = code.trim().to_uppercase();
        match clean.as_str() {
            "ME" | "ME1" => Some(RiotRegion::ME),
            "SEA" => Some(RiotRegion::SEA),
            "SG" | "SG2" => Some(RiotRegion::SG),
            "PH" | "PH2" => Some(RiotRegion::PH),
            "VN" | "VN2" => Some(RiotRegion::VN),
            "TW" | "TW2" => Some(RiotRegion::TW),
            "TH" | "TH2" => Some(RiotRegion::TH),
            "EUW" | "EUW1" => Some(RiotRegion::EUW),
            "EUNE" | "EUN1" | "EUN" => Some(RiotRegion::EUNE),
            "NA" | "NA1" => Some(RiotRegion::NA),
            "KR" => Some(RiotRegion::KR),
            "JP" | "JP1" => Some(RiotRegion::JP),
            "BR" | "BR1" => Some(RiotRegion::BR),
            "LAN" | "LA1" => Some(RiotRegion::LAN),
            "LAS" | "LA2" => Some(RiotRegion::LAS),
            "OCE" | "OC1" => Some(RiotRegion::OCE),
            "TR" | "TR1" => Some(RiotRegion::TR),
            "RU" => Some(RiotRegion::RU),
            "PBE" | "PBE1" => Some(RiotRegion::PBE),
            _ => None,
        }
    }

    /// Construct official Riot Games Account-V1 URL for looking up a Riot ID (gameName#tagLine).
    pub fn account_by_riot_id_url(&self, game_name: &str, tag_line: &str) -> String {
        let cluster = self.regional_routing();
        let encoded_name = url_encode(game_name);
        let encoded_tag = url_encode(tag_line);
        format!("https://{cluster}.api.riotgames.com/riot/account/v1/accounts/by-riot-id/{encoded_name}/{encoded_tag}")
    }

    /// Construct official Riot Games Summoner-V4 URL by PUUID.
    pub fn summoner_by_puuid_url(&self, puuid: &str) -> String {
        let platform = self.platform_id();
        format!("https://{platform}.api.riotgames.com/lol/summoner/v4/summoners/by-puuid/{puuid}")
    }

    /// Construct official Riot Games Match-V5 match list URL by PUUID.
    pub fn match_ids_url(&self, puuid: &str, count: usize) -> String {
        let cluster = self.regional_routing();
        format!("https://{cluster}.api.riotgames.com/lol/match/v5/matches/by-puuid/{puuid}/ids?count={count}")
    }
}

/// Simple percent-encoding for URI path segments.
fn url_encode(val: &str) -> String {
    let mut encoded = String::with_capacity(val.len() * 2);
    for b in val.bytes() {
        if b.is_ascii_alphanumeric() || b == b'-' || b == b'_' || b == b'.' || b == b'~' {
            encoded.push(b as char);
        } else if b == b' ' {
            encoded.push_str("%20");
        } else {
            encoded.push_str(&format!("%{:02X}", b));
        }
    }
    encoded
}
