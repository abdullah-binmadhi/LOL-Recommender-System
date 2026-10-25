// Supabase Edge Function: riot-proxy
// Securely proxies Riot Games Developer API without exposing RIOT_API_KEY to browser WebAssembly clients
// Persists authentic Match-V5 telemetry into Supabase PostgreSQL (public.player_matches) via @supabase/supabase-js

import { serve } from "https://deno.land/std@0.168.0/http/server.ts";
import { createClient } from "https://esm.sh/@supabase/supabase-js@2.39.8";

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

function resolveQueueName(queueId?: number): string {
  switch (queueId) {
    case 420:
      return "Ranked Solo/Duo";
    case 440:
      return "Ranked Flex";
    case 400:
      return "Normal Draft";
    case 430:
      return "Normal Blind";
    case 450:
      return "ARAM";
    case 490:
      return "Quickplay";
    case 1700:
      return "Arena";
    case 1900:
      return "Pick URF";
    case 830:
    case 840:
    case 850:
      return "Co-op vs AI";
    default:
      return queueId ? `Queue #${queueId}` : "Normal / Custom";
  }
}

function formatPlayedAgo(timestampMs?: number): string {
  if (!timestampMs || timestampMs <= 0) return "Recent Match";
  const now = Date.now();
  const diffMs = Math.max(0, now - timestampMs);
  const diffMinutes = Math.floor(diffMs / (60 * 1000));
  const diffHours = Math.floor(diffMs / (60 * 60 * 1000));
  const diffDays = Math.floor(diffMs / (24 * 60 * 60 * 1000));

  if (diffMinutes < 60) {
    return `${Math.max(1, diffMinutes)} mins ago`;
  }
  if (diffHours < 24) {
    return `${diffHours} hour${diffHours > 1 ? "s" : ""} ago`;
  }
  if (diffDays === 1) {
    const d = new Date(timestampMs);
    const timeStr = d.toLocaleTimeString("en-US", { hour: "numeric", minute: "2-digit", hour12: true });
    return `Yesterday at ${timeStr}`;
  }
  if (diffDays < 7) {
    return `${diffDays} days ago`;
  }
  const d = new Date(timestampMs);
  return d.toLocaleDateString("en-US", { month: "short", day: "numeric", year: "numeric" });
}

async function fetchWithTimeout(url: string, options: RequestInit, timeoutMs = 6000): Promise<Response> {
  const controller = new AbortController();
  const timer = setTimeout(() => controller.abort(), timeoutMs);
  try {
    const res = await fetch(url, { ...options, signal: controller.signal });
    clearTimeout(timer);
    return res;
  } catch (err) {
    clearTimeout(timer);
    throw err;
  }
}

async function resolveAccountPuuid(gameName: string, tagLine: string, preferredCluster: string, riotApiKey: string) {
  const clusters = [preferredCluster, "europe", "americas", "asia"].filter((v, i, a) => a.indexOf(v) === i);
  for (const cluster of clusters) {
    try {
      const url = `https://${cluster}.api.riotgames.com/riot/account/v1/accounts/by-riot-id/${encodeURIComponent(
        gameName
      )}/${encodeURIComponent(tagLine)}`;
      const res = await fetchWithTimeout(url, { headers: { "X-Riot-Token": riotApiKey } }, 4000);
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

  // Initialize Supabase Client with Service Role or Anon Key
  const supabaseUrl = Deno.env.get("SUPABASE_URL") || "https://erfybpoymihcepxnaosj.supabase.co";
  const supabaseKey =
    Deno.env.get("SUPABASE_SERVICE_ROLE_KEY") ||
    Deno.env.get("SUPABASE_ANON_KEY") ||
    "";
  const supabase = createClient(supabaseUrl, supabaseKey);

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
          matches: [],
        }),
        { status: 400, headers: corsHeaders }
      );
    }

    const [gameName, tagLine] = riot_id.split("#");
    const cluster = getRegionalCluster(region);
    const platform = getPlatformId(region);

    const riotApiKey = api_key || Deno.env.get("RIOT_API_KEY");
    if (!riotApiKey) {
      return new Response(
        JSON.stringify({
          error: "Missing Riot API Key. Provide a developer key or enter it in the dashboard.",
          matches: [],
        }),
        { status: 401, headers: corsHeaders }
      );
    }

    // 1. Resolve Summoner PUUID (Check Supabase first to save Riot API quota & eliminate latency)
    let puuid: string | undefined = undefined;
    let resolvedGameName = gameName;
    let resolvedTagLine = tagLine;

    const { data: existingPlayer } = await supabase
      .from("player_matches")
      .select("puuid, riot_id")
      .ilike("riot_id", `${gameName}%`)
      .limit(1)
      .maybeSingle();

    if (existingPlayer && existingPlayer.puuid) {
      puuid = existingPlayer.puuid;
      const parts = (existingPlayer.riot_id || "").split("#");
      if (parts[0]) resolvedGameName = parts[0];
      if (parts[1]) resolvedTagLine = parts[1];
    } else {
      const accountData = await resolveAccountPuuid(gameName, tagLine, cluster, riotApiKey);
      puuid = accountData?.puuid;
      if (accountData?.gameName) resolvedGameName = accountData.gameName;
      if (accountData?.tagLine) resolvedTagLine = accountData.tagLine;
    }

    if (!puuid) {
      return new Response(
        JSON.stringify({
          status: "riot_account_not_found",
          error: `Player '${riot_id}' not found on Riot servers or developer key is invalid.`,
          matches: [],
        }),
        { status: 404, headers: corsHeaders }
      );
    }

    const safeStart = Math.max(0, Number(start) || 0);
    // Use an agile default batch size of 10 for lightning-fast sync without triggering 429 rate limits
    const safeCount = Math.min(20, Math.max(1, Number(count) || 10));

    // 2. Fetch match IDs from Riot Match-V5
    let apiKeyExpired = false;
    let rateLimited = false;
    let retryAfterSeconds = 0;
    let newMatchesCount = 0;
    const parsedNewMatches: any[] = [];

    const matchesUrl = `https://${cluster}.api.riotgames.com/lol/match/v5/matches/by-puuid/${puuid}/ids?start=${safeStart}&count=${safeCount}`;
    let matchIds: string[] = [];

    try {
      const matchesRes = await fetchWithTimeout(matchesUrl, {
        headers: { "X-Riot-Token": riotApiKey },
      }, 4000);

      if (matchesRes.status === 401 || matchesRes.status === 403) {
        apiKeyExpired = true;
      } else if (matchesRes.status === 429) {
        rateLimited = true;
        const retryHeader = matchesRes.headers.get("retry-after");
        retryAfterSeconds = retryHeader ? Math.max(1, parseInt(retryHeader, 10)) : 30;
      } else if (matchesRes.ok) {
        matchIds = ((await matchesRes.json()) as string[]) || [];
      }
    } catch (e) {
      console.warn("Match IDs fetch error/timeout:", e);
    }

    // If Riot Match IDs were obtained, check existing and ingest new matches concurrently
    if (matchIds.length > 0) {
      // Query existing match IDs in Supabase
      const { data: existingRows } = await supabase
        .from("player_matches")
        .select("match_id")
        .eq("puuid", puuid)
        .in("match_id", matchIds);

      const existingIdsSet = new Set((existingRows || []).map((r: any) => r.match_id));
      const neededIds = matchIds.filter((id) => !existingIdsSet.has(id));

      // Batch fetch match details in parallel chunks of 5 with concurrency control
      const CHUNK_SIZE = 5;
      for (let i = 0; i < neededIds.length; i += CHUNK_SIZE) {
        const chunk = neededIds.slice(i, i + CHUNK_SIZE);
        const chunkPromises = chunk.map(async (matchId) => {
          try {
            const mRes = await fetchWithTimeout(
              `https://${cluster}.api.riotgames.com/lol/match/v5/matches/${matchId}`,
              { headers: { "X-Riot-Token": riotApiKey } },
              6000
            );
            if (!mRes.ok) return null;
            const mData = await mRes.json();
            const participant = mData.info?.participants?.find((p: any) => p.puuid === puuid);
            if (!participant) return null;

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

            const champName = participant.championName || "Unknown";
            const goldDiff15 = participant.challenges?.goldDiffAtMin15
              ? Math.round(participant.challenges.goldDiffAtMin15)
              : 0;

            const enemyOpponent = (mData.info.participants || []).find(
              (p: any) => p.teamId !== participant.teamId && p.teamPosition === participant.teamPosition
            );
            const laneOpponentChampion = enemyOpponent ? (enemyOpponent.championName || null) : null;
            const durationMins = Math.round((mData.info.gameDuration || 1800) / 60);
            const items = [
              participant.item0, participant.item1, participant.item2,
              participant.item3, participant.item4, participant.item5, participant.item6,
            ].filter((id: number) => id && id > 0);

            const teamDamageTotal =
              (mData.info.participants || [])
                .filter((p: any) => p.teamId === participant.teamId)
                .reduce((sum: number, p: any) => sum + (p.totalDamageDealtToChampions || 0), 0) || 1;
            const enemyDamageTotal =
              (mData.info.participants || [])
                .filter((p: any) => p.teamId !== participant.teamId)
                .reduce((sum: number, p: any) => sum + (p.totalDamageDealtToChampions || 0), 0) || 1;

            const alliedTeam = (mData.info.participants || [])
              .filter((p: any) => p.teamId === participant.teamId)
              .map((p: any) => {
                const pCs = (p.totalMinionsKilled || 0) + (p.neutralMinionsKilled || 0);
                const pDamage = p.totalDamageDealtToChampions || 0;
                const pName = p.riotIdGameName
                  ? `${p.riotIdGameName}#${p.riotIdTagline || ""}`
                  : p.summonerName || p.championName || "Summoner";
                return {
                  champion_name: p.championName || "Unknown",
                  role: p.teamPosition || "Unknown",
                  summoner_name: pName,
                  kills: p.kills || 0,
                  deaths: p.deaths || 0,
                  assists: p.assists || 0,
                  damage_dealt: pDamage,
                  cs: pCs,
                  cs_per_min: durationMin > 0 ? Math.round((pCs / durationMin) * 10) / 10 : 0,
                  gold_earned: p.goldEarned || 0,
                  damage_share_pct: Math.round((pDamage / teamDamageTotal) * 1000) / 10,
                  vision_score: p.visionScore || 0,
                  is_user: p.puuid === puuid,
                  is_player: p.puuid === puuid,
                };
              });

            const enemyTeam = (mData.info.participants || [])
              .filter((p: any) => p.teamId !== participant.teamId)
              .map((p: any) => {
                const pCs = (p.totalMinionsKilled || 0) + (p.neutralMinionsKilled || 0);
                const pDamage = p.totalDamageDealtToChampions || 0;
                const pName = p.riotIdGameName
                  ? `${p.riotIdGameName}#${p.riotIdTagline || ""}`
                  : p.summonerName || p.championName || "Summoner";
                return {
                  champion_name: p.championName || "Unknown",
                  role: p.teamPosition || "Unknown",
                  summoner_name: pName,
                  kills: p.kills || 0,
                  deaths: p.deaths || 0,
                  assists: p.assists || 0,
                  damage_dealt: pDamage,
                  cs: pCs,
                  cs_per_min: durationMin > 0 ? Math.round((pCs / durationMin) * 10) / 10 : 0,
                  gold_earned: p.goldEarned || 0,
                  damage_share_pct: Math.round((pDamage / enemyDamageTotal) * 1000) / 10,
                  vision_score: p.visionScore || 0,
                  is_user: false,
                  is_player: false,
                };
              });

            const queueId = Number(mData.info.queueId) || 0;
            const queueName = resolveQueueName(queueId);
            const gameCreation = Number(mData.info.gameCreation) || Date.now();

            return {
              match_id: matchId,
              puuid,
              riot_id,
              region,
              game_creation: gameCreation,
              game_duration_mins: durationMins,
              queue_id: queueId,
              queue_name: queueName,
              champion_name: champName,
              role,
              win: Boolean(participant.win),
              kills: participant.kills || 0,
              deaths: participant.deaths || 0,
              assists: participant.assists || 0,
              cs: totalCs,
              cs_per_min: csPerMin,
              gold_diff_15: goldDiff15,
              damage_dealt: participant.totalDamageDealtToChampions || 0,
              vision_score: participant.visionScore || 0,
              items: items,
              lane_opponent_champion: laneOpponentChampion,
              allied_team: alliedTeam,
              enemy_team: enemyTeam,
            };
          } catch (e) {
            console.warn("Match parse failed for", matchId, e);
            return null;
          }
        });

        const chunkResults = await Promise.all(chunkPromises);
        for (const item of chunkResults) {
          if (item) parsedNewMatches.push(item);
        }
      }

      // Batch persist newly discovered matches to Supabase PostgreSQL
      if (parsedNewMatches.length > 0) {
        newMatchesCount = parsedNewMatches.length;
        const { error: upsertErr } = await supabase
          .from("player_matches")
          .upsert(parsedNewMatches, { onConflict: "match_id" });
        if (upsertErr) {
          console.warn("Supabase upsert error:", upsertErr);
        }
      }
    }

    // 3. Query Authentic Matches from Supabase
    const { data: dbMatches, count: totalCount, error: queryErr } = await supabase
      .from("player_matches")
      .select("*", { count: "exact" })
      .eq("puuid", puuid)
      .order("game_creation", { ascending: false })
      .range(safeStart, safeStart + safeCount - 1);

    if (queryErr) {
      console.warn("Supabase query error:", queryErr);
    }

    const rawMatches = dbMatches && dbMatches.length > 0 ? dbMatches : parsedNewMatches;

    // Transform into AdoptedMatch format for Leptos client
    const adoptedMatches = rawMatches.map((m: any) => {
      const champName = m.champion_name;
      const recIdx = recommended_champions.findIndex(
        (c: string) => c.toLowerCase() === champName.toLowerCase()
      );
      const isCalibrated = recIdx !== -1;
      const calibratedRank = isCalibrated ? recIdx + 1 : null;

      let items = m.items;
      if (typeof items === "string") {
        try { items = JSON.parse(items); } catch (_) { items = []; }
      }
      let alliedTeam = m.allied_team;
      if (typeof alliedTeam === "string") {
        try { alliedTeam = JSON.parse(alliedTeam); } catch (_) { alliedTeam = []; }
      }
      let enemyTeam = m.enemy_team;
      if (typeof enemyTeam === "string") {
        try { enemyTeam = JSON.parse(enemyTeam); } catch (_) { enemyTeam = []; }
      }

      const gameCreation = Number(m.game_creation) || 0;

      return {
        match_id: m.match_id,
        champion_name: champName,
        role: m.role,
        win: Boolean(m.win),
        kills: Number(m.kills) || 0,
        deaths: Number(m.deaths) || 0,
        assists: Number(m.assists) || 0,
        cs_per_min: Number(m.cs_per_min) || 0.0,
        gold_diff_15: Number(m.gold_diff_15) || 0,
        is_calibrated: isCalibrated,
        calibrated_rank: calibratedRank,
        lane_opponent_champion: m.lane_opponent_champion || null,
        duration_mins: Number(m.game_duration_mins) || 20,
        user_damage_dealt: Number(m.damage_dealt) || 0,
        user_vision_score: Number(m.vision_score) || 0,
        items: items || [],
        allied_team: alliedTeam || [],
        enemy_team: enemyTeam || [],
        game_creation_timestamp: gameCreation,
        queue_id: Number(m.queue_id) || 0,
        queue_name: m.queue_name || resolveQueueName(m.queue_id),
        played_ago_label: formatPlayedAgo(gameCreation),
      };
    });

    // 4. Query Champion Masteries
    let topMasteries: any[] = [];
    try {
      const masteryUrl = `https://${platform}.api.riotgames.com/lol/champion-mastery/v4/champion-masteries/by-puuid/${puuid}`;
      const masteryRes = await fetchWithTimeout(masteryUrl, {
        headers: { "X-Riot-Token": riotApiKey },
      }, 4000);
      if (masteryRes.ok) {
        const allMasteries = await masteryRes.json();
        topMasteries = Array.isArray(allMasteries) ? allMasteries.slice(0, 25) : [];
      }
    } catch (_) {
      // Non-fatal
    }

    let responseStatus = "success";
    let statusMsg: string | null = null;
    let riotApiStatus = "connected";

    if (apiKeyExpired) {
      responseStatus = "api_key_expired";
      statusMsg = "Riot Developer API Key expired (24h limit) or invalid. Enter a fresh RGAPI key to ingest latest games.";
      riotApiStatus = "key_expired";
    } else if (rateLimited) {
      responseStatus = "rate_limited";
      statusMsg = `Riot API rate limit reached (100 req/2min). Cooldown in progress (ready in ${retryAfterSeconds}s).`;
      riotApiStatus = "rate_limited";
    }

    return new Response(
      JSON.stringify({
        status: responseStatus,
        data_source: "live_postgres",
        new_matches_synced: newMatchesCount,
        total_matches_in_db: totalCount || rawMatches.length,
        riot_api_status: riotApiStatus,
        retry_after: retryAfterSeconds,
        error: statusMsg,
        summoner: {
          puuid,
          gameName: resolvedGameName,
          tagLine: resolvedTagLine,
          region,
          top_masteries: topMasteries.map((m: any) => ({
            championId: m.championId,
            championLevel: m.championLevel,
            championPoints: m.championPoints,
          })),
        },
        matches: adoptedMatches,
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
