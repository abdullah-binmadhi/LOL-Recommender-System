use recommender_core::riot_api::RiotRegion;

#[test]
fn test_all_regions_have_unique_codes_and_platform_ids() {
    assert_eq!(RiotRegion::ALL.len(), 19);

    let mut codes = std::collections::HashSet::new();
    let mut platforms = std::collections::HashSet::new();

    for r in &RiotRegion::ALL {
        assert!(codes.insert(r.code()), "duplicate code: {}", r.code());
        assert!(!r.display_name().is_empty());
        assert!(!r.platform_id().is_empty());
        assert!(!r.regional_routing().is_empty());
        platforms.insert(r.platform_id());
    }

    // SEA and SG share sg2 as the primary hub
    assert_eq!(platforms.len(), 18);
}

#[test]
fn test_middle_east_me1_routing() {
    let me = RiotRegion::from_code("ME").expect("ME exists");
    assert_eq!(me, RiotRegion::ME);
    assert_eq!(me.platform_id(), "me1");
    assert_eq!(me.regional_routing(), "europe");
    assert_eq!(me.display_name(), "Middle East (ME / ME1)");

    let url = me.account_by_riot_id_url("Faker", "T1");
    assert_eq!(
        url,
        "https://europe.api.riotgames.com/riot/account/v1/accounts/by-riot-id/Faker/T1"
    );

    let summoner_url = me.summoner_by_puuid_url("sample-puuid");
    assert_eq!(
        summoner_url,
        "https://me1.api.riotgames.com/lol/summoner/v4/summoners/by-puuid/sample-puuid"
    );
}

#[test]
fn test_southeast_asia_sea_routing() {
    let sea = RiotRegion::from_code("SEA").expect("SEA exists");
    assert_eq!(sea, RiotRegion::SEA);
    assert_eq!(sea.platform_id(), "sg2");
    assert_eq!(sea.regional_routing(), "sea");
    assert_eq!(sea.display_name(), "Southeast Asia (SEA / SG2)");

    let vn = RiotRegion::from_code("VN").expect("VN exists");
    assert_eq!(vn.platform_id(), "vn2");
    assert_eq!(vn.regional_routing(), "sea");

    let ph = RiotRegion::from_code("PH").expect("PH exists");
    assert_eq!(ph.platform_id(), "ph2");
    assert_eq!(ph.regional_routing(), "sea");

    let url = sea.account_by_riot_id_url("Levi", "GAM");
    assert_eq!(
        url,
        "https://sea.api.riotgames.com/riot/account/v1/accounts/by-riot-id/Levi/GAM"
    );
}

#[test]
fn test_case_insensitive_from_code() {
    assert_eq!(RiotRegion::from_code("me1"), Some(RiotRegion::ME));
    assert_eq!(RiotRegion::from_code("sea"), Some(RiotRegion::SEA));
    assert_eq!(RiotRegion::from_code("sg2"), Some(RiotRegion::SG));
    assert_eq!(RiotRegion::from_code("euw1"), Some(RiotRegion::EUW));
    assert_eq!(RiotRegion::from_code("na1"), Some(RiotRegion::NA));
    assert_eq!(RiotRegion::from_code("kr"), Some(RiotRegion::KR));
    assert_eq!(RiotRegion::from_code("UNKNOWN"), None);
}
