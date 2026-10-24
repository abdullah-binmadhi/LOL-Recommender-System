// Supabase Edge Function: riot-proxy
// Securely proxies Riot Games Developer API without exposing RIOT_API_KEY to browser WebAssembly clients

import { serve } from "https://deno.land/std@0.168.0/http/server.ts";

const corsHeaders = {
  "Access-Control-Allow-Origin": "*",
  "Access-Control-Allow-Headers": "authorization, x-client-info, apikey, content-type",
  "Content-Type": "application/json",
};

interface RiotProxyRequest {
  riot_id: string; // "GameName#TagLine"
  region?: string; // "me", "euw", "sea", "na", "kr", etc.
  api_key?: string; // Optional user-supplied developer API key
  recommended_champions?: string[]; // Top recommended champions to check against
  start?: number; // Pagination offset for "+ Reveal 10 More Games"
  count?: number; // Number of matches to fetch per page (default 20)
}

function getRegionalCluster(reg: string): string {
  const r = (reg || "me").toLowerCase();
  if (["me", "me1", "euw", "euw1", "eune", "eun1", "tr", "tr1", "ru"].includes(r)) return "europe";
  if (["kr", "jp", "jp1"].includes(r)) return "asia";
  if (["sea", "sg", "sg2", "ph", "ph2", "vn", "vn2", "tw", "tw2", "th", "th2"].includes(r)) return "sea";
  return "americas";
}

function getPlatformId(reg: string): string {
  const r = (reg || "me").toLowerCase();
  const map: Record<string, string> = {
    me: "me1",
    me1: "me1",
    euw: "euw1",
    euw1: "euw1",
    eune: "eun1",
    eun1: "eun1",
    na: "na1",
    na1: "na1",
    kr: "kr",
    jp: "jp1",
    jp1: "jp1",
    br: "br1",
    br1: "br1",
    lan: "la1",
    la1: "la1",
    las: "la2",
    la2: "la2",
    sea: "sg2",
    sg: "sg2",
    sg2: "sg2",
    ph: "ph2",
    ph2: "ph2",
    vn: "vn2",
    vn2: "vn2",
    tw: "tw2",
    tw2: "tw2",
    th: "th2",
    th2: "th2",
    oce: "oc1",
    oc1: "oc1",
    tr: "tr1",
    tr1: "tr1",
    ru: "ru",
  };
  return map[r] || "me1";
}

function getFallbackTelemetry(gameName: string, tagLine: string, region: string) {
  const isShen = gameName.toUpperCase().includes("SHEN") || tagLine.toUpperCase().includes("SHEN");
  const regUpper = (region || "ME").toUpperCase();
  const prefix = regUpper === "ME" || regUpper === "ME1" ? "ME1-" : `${regUpper}-`;

  const matches = isShen
    ? [
        {
          match_id: `${prefix}104928`,
          champion_name: "Ornn",
          role: "Top",
          win: true,
          kills: 5,
          deaths: 1,
          assists: 14,
          cs_per_min: 7.8,
          gold_diff_15: 840,
          is_calibrated: true,
          calibrated_rank: 1,
          lane_opponent_champion: "Vel'Koz",
          duration_mins: 31,
          user_damage_dealt: 21850,
          user_vision_score: 42,
          items: [3068, 6665, 3075, 3111, 2055, 3340],
          allied_team: [
            { champion_name: "Ornn", role: "Top", kills: 5, deaths: 1, assists: 14, damage_dealt: 21850, cs: 242, is_player: true },
            { champion_name: "Sejuani", role: "Jungle", kills: 4, deaths: 2, assists: 16, damage_dealt: 14200, cs: 178, is_player: false },
            { champion_name: "Ahri", role: "Mid", kills: 10, deaths: 3, assists: 8, damage_dealt: 26400, cs: 255, is_player: false },
            { champion_name: "Jinx", role: "ADC", kills: 9, deaths: 2, assists: 11, damage_dealt: 31200, cs: 284, is_player: false },
            { champion_name: "Thresh", role: "Support", kills: 1, deaths: 3, assists: 22, damage_dealt: 6800, cs: 38, is_player: false },
          ],
          enemy_team: [
            { champion_name: "Vel'Koz", role: "Top", kills: 3, deaths: 6, assists: 4, damage_dealt: 23500, cs: 215, is_player: false },
            { champion_name: "Jarvan IV", role: "Jungle", kills: 3, deaths: 6, assists: 7, damage_dealt: 12100, cs: 162, is_player: false },
            { champion_name: "Syndra", role: "Mid", kills: 4, deaths: 5, assists: 5, damage_dealt: 22100, cs: 240, is_player: false },
            { champion_name: "Kai'Sa", role: "ADC", kills: 1, deaths: 7, assists: 3, damage_dealt: 17400, cs: 230, is_player: false },
            { champion_name: "Nautilus", role: "Support", kills: 0, deaths: 5, assists: 6, damage_dealt: 5100, cs: 42, is_player: false },
          ],
        },
        {
          match_id: `${prefix}104855`,
          champion_name: "Shen",
          role: "Top",
          win: true,
          kills: 4,
          deaths: 2,
          assists: 18,
          cs_per_min: 6.9,
          gold_diff_15: 980,
          is_calibrated: true,
          calibrated_rank: 1,
        },
        {
          match_id: `${prefix}104712`,
          champion_name: "Shen",
          role: "Top",
          win: true,
          kills: 8,
          deaths: 0,
          assists: 11,
          cs_per_min: 7.8,
          gold_diff_15: 2100,
          is_calibrated: true,
          calibrated_rank: 1,
        },
        {
          match_id: `${prefix}104640`,
          champion_name: "Darius",
          role: "Top",
          win: false,
          kills: 3,
          deaths: 6,
          assists: 2,
          cs_per_min: 6.2,
          gold_diff_15: -1150,
          is_calibrated: false,
          calibrated_rank: null,
        },
        {
          match_id: `${prefix}104519`,
          champion_name: "Shen",
          role: "Top",
          win: true,
          kills: 5,
          deaths: 3,
          assists: 16,
          cs_per_min: 7.1,
          gold_diff_15: 850,
          is_calibrated: true,
          calibrated_rank: 1,
        },
        {
          match_id: `${prefix}104402`,
          champion_name: "Ornn",
          role: "Top",
          win: true,
          kills: 3,
          deaths: 2,
          assists: 15,
          cs_per_min: 7.3,
          gold_diff_15: 640,
          is_calibrated: true,
          calibrated_rank: 2,
        },
        {
          match_id: `${prefix}104331`,
          champion_name: "Aatrox",
          role: "Top",
          win: false,
          kills: 4,
          deaths: 7,
          assists: 3,
          cs_per_min: 6.5,
          gold_diff_15: -890,
          is_calibrated: false,
          calibrated_rank: null,
        },
        {
          match_id: `${prefix}104215`,
          champion_name: "Shen",
          role: "Top",
          win: true,
          kills: 7,
          deaths: 1,
          assists: 13,
          cs_per_min: 7.6,
          gold_diff_15: 1750,
          is_calibrated: true,
          calibrated_rank: 1,
        },
        {
          match_id: `${prefix}104108`,
          champion_name: "K'Sante",
          role: "Top",
          win: true,
          kills: 5,
          deaths: 2,
          assists: 8,
          cs_per_min: 7.9,
          gold_diff_15: 1100,
          is_calibrated: true,
          calibrated_rank: 3,
        },
        {
          match_id: `${prefix}103980`,
          champion_name: "Shen",
          role: "Top",
          win: true,
          kills: 4,
          deaths: 1,
          assists: 12,
          cs_per_min: 7.2,
          gold_diff_15: 1200,
          is_calibrated: true,
          calibrated_rank: 1,
        },
      ]
    : [
        {
          match_id: `${prefix}901234`,
          champion_name: "Ahri",
          role: "Mid",
          win: true,
          kills: 9,
          deaths: 2,
          assists: 11,
          cs_per_min: 8.4,
          gold_diff_15: 1350,
          is_calibrated: true,
          calibrated_rank: 1,
        },
      ];

  return {
    status: "simulated_telemetry",
    data_source: "fallback",
    summoner: {
      gameName,
      tagLine,
      region,
      primary_role: isShen ? "Top" : "Mid",
      top_champions: isShen ? ["Shen", "Ornn", "K'Sante"] : ["Ahri", "Hwei", "Zed"],
    },
    matches,
  };
}

async function resolveAccountPuuid(gameName: string, tagLine: string, preferredCluster: string, riotApiKey: string) {
  // Try preferred cluster first, then fallback to americas / asia
  const clusters = [preferredCluster, "americas", "asia", "europe"].filter((v, i, a) => a.indexOf(v) === i);
  for (const cluster of clusters) {
    try {
      const url = `https://${cluster}.api.riotgames.com/riot/account/v1/accounts/by-riot-id/${encodeURIComponent(
        gameName
      )}/${encodeURIComponent(tagLine)}`;
      const res = await fetch(url, { headers: { "X-Riot-Token": riotApiKey } });
      if (res.ok) {
        const data = await res.json();
        return { puuid: data.puuid, gameName: data.gameName, tagLine: data.tagLine, workingCluster: cluster };
      }
    } catch (_) {
      // Continue to next cluster
    }
  }
  return null;
}

serve(async (req: Request) => {
  if (req.method === "OPTIONS") {
    return new Response("ok", { headers: corsHeaders });
  }

  try {
    const {
      riot_id,
      region = "me",
      api_key,
      recommended_champions = [],
      start = 0,
      count = 20,
    } = (await req.json()) as RiotProxyRequest;

    if (!riot_id || !riot_id.includes("#")) {
      return new Response(
        JSON.stringify({
          error: "Invalid Riot ID format. Expected 'GameName#TagLine' (e.g. SHENTRICK#SHEN)",
        }),
        { status: 400, headers: corsHeaders }
      );
    }

    const [gameName, tagLine] = riot_id.split("#");
    const cluster = getRegionalCluster(region);
    const platform = getPlatformId(region);
    const riotApiKey = api_key || Deno.env.get("RIOT_API_KEY");

    // Fallback if no Riot API key is configured
    if (!riotApiKey) {
      const fallback = getFallbackTelemetry(gameName, tagLine, region);
      return new Response(JSON.stringify(fallback), {
        status: 200,
        headers: corsHeaders,
      });
    }

    // 1. Resolve PUUID from Riot Account API with fallback clusters
    const accountData = await resolveAccountPuuid(gameName, tagLine, cluster, riotApiKey);
    if (!accountData) {
      console.warn("Riot Account API lookup failed across clusters, returning fallback telemetry");
      const fallback = getFallbackTelemetry(gameName, tagLine, region);
      return new Response(JSON.stringify(fallback), {
        status: 200,
        headers: corsHeaders,
      });
    }

    const puuid = accountData.puuid;

    // 2. Fetch Recent Match-V5 IDs (supports OP.GG-style pagination via start & count)
    const safeStart = Math.max(0, Number(start) || 0);
    const safeCount = Math.min(25, Math.max(10, Number(count) || 20));
    const matchesUrl = `https://${cluster}.api.riotgames.com/lol/match/v5/matches/by-puuid/${puuid}/ids?start=${safeStart}&count=${safeCount}`;
    const matchesRes = await fetch(matchesUrl, {
      headers: { "X-Riot-Token": riotApiKey },
    });

    const adoptedMatches: any[] = [];
    if (matchesRes.ok) {
      const matchIds = (await matchesRes.json()) as string[];
      for (const matchId of matchIds.slice(0, safeCount)) {
        try {
          const mRes = await fetch(
            `https://${cluster}.api.riotgames.com/lol/match/v5/matches/${matchId}`,
            { headers: { "X-Riot-Token": riotApiKey } }
          );
          if (!mRes.ok) continue;
          const mData = await mRes.json();
          const participant = mData.info?.participants?.find(
            (p: any) => p.puuid === puuid
          );
          if (!participant) continue;

          const durationMin = Math.max(1, (mData.info.gameDuration || 1800) / 60);
          const totalCs = (participant.totalMinionsKilled || 0) + (participant.neutralMinionsKilled || 0);
          const csPerMin = Math.round((totalCs / durationMin) * 10) / 10;
          const pos = participant.teamPosition || participant.individualPosition || "TOP";
          const role =
            pos === "UTILITY"
              ? "Support"
              : pos === "MIDDLE"
              ? "Mid"
              : pos === "TOP"
              ? "Top"
              : pos === "JUNGLE"
              ? "Jungle"
              : pos === "BOTTOM"
              ? "Bottom"
              : "Top";

          // Calculate real calibration rank based on user recommendations
          const champName = participant.championName || "Unknown";
          const recIdx = recommended_champions.findIndex(
            (c: string) => c.toLowerCase() === champName.toLowerCase()
          );
          const isCalibrated = recIdx !== -1;
          const calibratedRank = isCalibrated ? recIdx + 1 : null;

          const goldDiff15 = participant.challenges?.goldDiffAtMin15
            ? Math.round(participant.challenges.goldDiffAtMin15)
            : 0;

          // Extract authentic lane opponent
          const enemyOpponent = (matchData.info.participants || []).find(
            (p: any) => p.teamId !== participant.teamId && p.teamPosition === participant.teamPosition
          );
          const laneOpponentChampion = enemyOpponent ? (enemyOpponent.championName || null) : null;
          const durationMins = Math.round((matchData.info.gameDuration || 1800) / 60);
          const items = [
            participant.item0, participant.item1, participant.item2,
            participant.item3, participant.item4, participant.item5, participant.item6
          ].filter((id: number) => id && id > 0);

          const alliedTeam = (matchData.info.participants || [])
            .filter((p: any) => p.teamId === participant.teamId)
            .map((p: any) => ({
              champion_name: p.championName || "Unknown",
              role: p.teamPosition || "Unknown",
              kills: p.kills || 0,
              deaths: p.deaths || 0,
              assists: p.assists || 0,
              damage_dealt: p.totalDamageDealtToChampions || 0,
              cs: (p.totalMinionsKilled || 0) + (p.neutralMinionsKilled || 0),
              is_player: p.puuid === puuid,
            }));

          const enemyTeam = (matchData.info.participants || [])
            .filter((p: any) => p.teamId !== participant.teamId)
            .map((p: any) => ({
              champion_name: p.championName || "Unknown",
              role: p.teamPosition || "Unknown",
              kills: p.kills || 0,
              deaths: p.deaths || 0,
              assists: p.assists || 0,
              damage_dealt: p.totalDamageDealtToChampions || 0,
              cs: (p.totalMinionsKilled || 0) + (p.neutralMinionsKilled || 0),
              is_player: false,
            }));

          adoptedMatches.push({
            match_id: matchId,
            champion_name: champName,
            role,
            win: Boolean(participant.win),
            kills: participant.kills || 0,
            deaths: participant.deaths || 0,
            assists: participant.assists || 0,
            cs_per_min: csPerMin,
            gold_diff_15: goldDiff15,
            is_calibrated: isCalibrated,
            calibrated_rank: calibratedRank,
            lane_opponent_champion: laneOpponentChampion,
            duration_mins: durationMins,
            user_damage_dealt: participant.totalDamageDealtToChampions || 0,
            user_vision_score: participant.visionScore || 0,
            items,
            allied_team: alliedTeam,
            enemy_team: enemyTeam,
          });
        } catch (e) {
          console.warn("Failed to parse match", matchId, e);
        }
      }
    }

    // 3. Query Full Lifetime Champion Masteries (from Day 1 debut to latest game)
    const masteryUrl = `https://${platform}.api.riotgames.com/lol/champion-mastery/v4/champion-masteries/by-puuid/${puuid}`;
    const masteryRes = await fetch(masteryUrl, {
      headers: { "X-Riot-Token": riotApiKey },
    });

    let topMasteries: any[] = [];
    if (masteryRes.ok) {
      const allMasteries = await masteryRes.json();
      topMasteries = Array.isArray(allMasteries) ? allMasteries.slice(0, 25) : [];
    }

    // Determine final matches and data source
    const isLive = adoptedMatches.length > 0;
    const finalMatches = isLive
      ? adoptedMatches
      : getFallbackTelemetry(gameName, tagLine, region).matches;

    return new Response(
      JSON.stringify({
        status: "success",
        data_source: isLive ? "live" : "fallback",
        summoner: {
          puuid,
          gameName: accountData.gameName,
          tagLine: accountData.tagLine,
          region,
          top_masteries: topMasteries.map((m: any) => ({
            championId: m.championId,
            championLevel: m.championLevel,
            championPoints: m.championPoints,
          })),
        },
        matches: finalMatches,
      }),
      { status: 200, headers: corsHeaders }
    );
  } catch (error) {
    return new Response(
      JSON.stringify({ error: (error as Error).message }),
      { status: 500, headers: corsHeaders }
    );
  }
});
