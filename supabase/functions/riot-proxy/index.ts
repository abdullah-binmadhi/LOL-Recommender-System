// Supabase Edge Function: riot-proxy
// Securely proxies Riot Games Developer API without exposing RIOT_API_KEY to browser WebAssembly clients
// Persists authentic Match-V5 telemetry into Supabase PostgreSQL (public.player_matches)

import { serve } from "https://deno.land/std@0.168.0/http/server.ts";
import { Client } from "https://deno.land/x/postgres@v0.17.0/mod.ts";

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

async function ensureDatabaseTables(client: Client) {
  try {
    await client.queryArray(`
      CREATE TABLE IF NOT EXISTS public.player_matches (
        id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
        match_id TEXT NOT NULL UNIQUE,
        puuid TEXT NOT NULL,
        riot_id TEXT NOT NULL,
        region TEXT NOT NULL,
        game_creation BIGINT NOT NULL,
        game_duration_mins INT NOT NULL,
        queue_id INT NOT NULL,
        queue_name TEXT NOT NULL,
        champion_name TEXT NOT NULL,
        role TEXT NOT NULL,
        win BOOLEAN NOT NULL,
        kills INT NOT NULL DEFAULT 0,
        deaths INT NOT NULL DEFAULT 0,
        assists INT NOT NULL DEFAULT 0,
        cs INT NOT NULL DEFAULT 0,
        cs_per_min DOUBLE PRECISION NOT NULL DEFAULT 0.0,
        gold_diff_15 INT NOT NULL DEFAULT 0,
        damage_dealt INT NOT NULL DEFAULT 0,
        vision_score INT NOT NULL DEFAULT 0,
        items JSONB NOT NULL DEFAULT '[]'::jsonb,
        lane_opponent_champion TEXT,
        allied_team JSONB NOT NULL DEFAULT '[]'::jsonb,
        enemy_team JSONB NOT NULL DEFAULT '[]'::jsonb,
        created_at TIMESTAMPTZ NOT NULL DEFAULT timezone('utc'::text, now())
      );

      ALTER TABLE public.player_matches
        ADD COLUMN IF NOT EXISTS queue_name TEXT,
        ADD COLUMN IF NOT EXISTS game_creation BIGINT,
        ADD COLUMN IF NOT EXISTS lane_opponent_champion TEXT,
        ADD COLUMN IF NOT EXISTS allied_team JSONB,
        ADD COLUMN IF NOT EXISTS enemy_team JSONB;

      CREATE INDEX IF NOT EXISTS idx_player_matches_puuid_creation 
        ON public.player_matches (puuid, game_creation DESC);
    `);
  } catch (err) {
    console.warn("Table auto-migration notice:", err);
  }
}

async function resolveAccountPuuid(gameName: string, tagLine: string, preferredCluster: string, riotApiKey: string) {
  const clusters = [preferredCluster, "europe", "americas", "asia"].filter((v, i, a) => a.indexOf(v) === i);
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

  let dbClient: Client | null = null;
  const dbUrl = Deno.env.get("SUPABASE_DB_URL");
  if (dbUrl) {
    try {
      dbClient = new Client(dbUrl);
      await dbClient.connect();
      await ensureDatabaseTables(dbClient);
    } catch (e) {
      console.warn("Could not connect to Postgres DB:", e);
      dbClient = null;
    }
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
    if (!riotApiKey) {
      return new Response(
        JSON.stringify({
          error: "Missing Riot API Key.",
          matches: [],
        }),
        { status: 401, headers: corsHeaders }
      );
    }

    // 1. Resolve Summoner PUUID
    const accountData = await resolveAccountPuuid(gameName, tagLine, cluster, riotApiKey);
    if (!accountData) {
      return new Response(
        JSON.stringify({
          error: `Player '${riot_id}' not found on Riot servers.`,
          matches: [],
        }),
        { status: 404, headers: corsHeaders }
      );
    }

    const puuid = accountData.puuid;
    const safeStart = Math.max(0, Number(start) || 0);
    const safeCount = Math.min(30, Math.max(10, Number(count) || 20));

    // 2. Fetch match IDs from Riot Match-V5
    const matchesUrl = `https://${cluster}.api.riotgames.com/lol/match/v5/matches/by-puuid/${puuid}/ids?start=${safeStart}&count=${safeCount}`;
    const matchesRes = await fetch(matchesUrl, {
      headers: { "X-Riot-Token": riotApiKey },
    });

    const parsedNewMatches: any[] = [];
    if (matchesRes.ok) {
      const matchIds = (await matchesRes.json()) as string[];

      // Query which match IDs already exist in Postgres
      let existingIdsSet = new Set<string>();
      if (dbClient) {
        try {
          const checkRes = await dbClient.queryObject<{ match_id: string }>(
            `SELECT match_id FROM public.player_matches WHERE puuid = $1`,
            [puuid]
          );
          existingIdsSet = new Set(checkRes.rows.map((r) => r.match_id));
        } catch (e) {
          console.warn("Failed checking existing match IDs:", e);
        }
      }

      for (const matchId of matchIds) {
        if (existingIdsSet.has(matchId)) {
          continue; // Already persisted in Postgres
        }

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
            participant.item3, participant.item4, participant.item5, participant.item6
          ].filter((id: number) => id && id > 0);

          const alliedTeam = (mData.info.participants || [])
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

          const enemyTeam = (mData.info.participants || [])
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

          const queueId = Number(mData.info.queueId) || 0;
          const queueName = resolveQueueName(queueId);
          const gameCreation = Number(mData.info.gameCreation) || Date.now();

          const record = {
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
            items: JSON.stringify(items),
            lane_opponent_champion: laneOpponentChampion,
            allied_team: JSON.stringify(alliedTeam),
            enemy_team: JSON.stringify(enemyTeam),
          };

          parsedNewMatches.push(record);

          // Persist to Postgres
          if (dbClient) {
            try {
              await dbClient.queryArray(
                `INSERT INTO public.player_matches (
                  match_id, puuid, riot_id, region, game_creation, game_duration_mins,
                  queue_id, queue_name, champion_name, role, win, kills, deaths, assists,
                  cs, cs_per_min, gold_diff_15, damage_dealt, vision_score, items,
                  lane_opponent_champion, allied_team, enemy_team
                ) VALUES (
                  $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23
                ) ON CONFLICT (match_id) DO NOTHING`,
                [
                  record.match_id,
                  record.puuid,
                  record.riot_id,
                  record.region,
                  record.game_creation,
                  record.game_duration_mins,
                  record.queue_id,
                  record.queue_name,
                  record.champion_name,
                  record.role,
                  record.win,
                  record.kills,
                  record.deaths,
                  record.assists,
                  record.cs,
                  record.cs_per_min,
                  record.gold_diff_15,
                  record.damage_dealt,
                  record.vision_score,
                  record.items,
                  record.lane_opponent_champion,
                  record.allied_team,
                  record.enemy_team,
                ]
              );
            } catch (insErr) {
              console.warn("Failed inserting match to Postgres:", insErr);
            }
          }
        } catch (e) {
          console.warn("Failed to parse match", matchId, e);
        }
      }
    }

    // 3. Retrieve Authentic Matches from Postgres or Fallback In-Memory
    let rawMatches: any[] = [];
    if (dbClient) {
      try {
        const queryRes = await dbClient.queryObject<any>(
          `SELECT match_id, puuid, riot_id, region, game_creation, game_duration_mins,
                  queue_id, queue_name, champion_name, role, win, kills, deaths, assists,
                  cs, cs_per_min, gold_diff_15, damage_dealt, vision_score, items,
                  lane_opponent_champion, allied_team, enemy_team
           FROM public.player_matches
           WHERE puuid = $1
           ORDER BY game_creation DESC
           LIMIT $2 OFFSET $3`,
          [puuid, safeCount, safeStart]
        );
        rawMatches = queryRes.rows;
      } catch (e) {
        console.warn("Failed querying matches from Postgres:", e);
      }
    }

    if (rawMatches.length === 0) {
      rawMatches = parsedNewMatches;
    }

    // Transform into AdoptedMatch format with queue & timestamp labels
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

    // 4. Query Full Lifetime Champion Masteries (from Day 1 debut to latest game)
    const masteryUrl = `https://${platform}.api.riotgames.com/lol/champion-mastery/v4/champion-masteries/by-puuid/${puuid}`;
    const masteryRes = await fetch(masteryUrl, {
      headers: { "X-Riot-Token": riotApiKey },
    });

    let topMasteries: any[] = [];
    if (masteryRes.ok) {
      const allMasteries = await masteryRes.json();
      topMasteries = Array.isArray(allMasteries) ? allMasteries.slice(0, 25) : [];
    }

    return new Response(
      JSON.stringify({
        status: "success",
        data_source: "live_postgres",
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
        matches: adoptedMatches,
      }),
      { status: 200, headers: corsHeaders }
    );
  } catch (error) {
    return new Response(
      JSON.stringify({ error: (error as Error).message }),
      { status: 500, headers: corsHeaders }
    );
  } finally {
    if (dbClient) {
      try {
        await dbClient.end();
      } catch (_) {
        // Ignore close error
      }
    }
  }
});
