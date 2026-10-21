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
  region?: string; // "americas", "europe", "asia", "sea"
}

serve(async (req: Request) => {
  if (req.method === "OPTIONS") {
    return new Response("ok", { headers: corsHeaders });
  }

  try {
    const { riot_id, region = "americas" } = (await req.json()) as RiotProxyRequest;

    if (!riot_id || !riot_id.includes("#")) {
      return new Response(
        JSON.stringify({
          error: "Invalid Riot ID format. Expected 'GameName#TagLine' (e.g. Faker#KR1)",
        }),
        { status: 400, headers: corsHeaders }
      );
    }

    const [gameName, tagLine] = riot_id.split("#");
    const riotApiKey = Deno.env.get("RIOT_API_KEY");

    // Fallback development response if API key is not yet configured in Supabase secrets
    if (!riotApiKey) {
      return new Response(
        JSON.stringify({
          status: "simulated",
          message: "RIOT_API_KEY secret not configured. Set RIOT_API_KEY in Supabase Vault.",
          summoner: {
            gameName,
            tagLine,
            primary_role: "Mid",
            top_champions: ["Ahri", "Hwei", "Zed"],
            suggested_difficulty: 6.5,
          },
        }),
        { status: 200, headers: corsHeaders }
      );
    }

    // 1. Resolve PUUID from Riot Account API
    const accountUrl = `https://${region}.api.riotgames.com/riot/account/v1/accounts/by-riot-id/${encodeURIComponent(
      gameName
    )}/${encodeURIComponent(tagLine)}`;

    const accountRes = await fetch(accountUrl, {
      headers: { "X-Riot-Token": riotApiKey },
    });

    if (!accountRes.ok) {
      const err = await accountRes.text();
      return new Response(
        JSON.stringify({ error: `Riot Account API error: ${err}` }),
        { status: accountRes.status, headers: corsHeaders }
      );
    }

    const accountData = await accountRes.json();
    const puuid = accountData.puuid;

    // 2. Query Top Champion Masteries
    const masteryPlatform = region === "europe" ? "euw1" : region === "asia" ? "kr" : "na1";
    const masteryUrl = `https://${masteryPlatform}.api.riotgames.com/lol/champion-mastery/v4/champion-masteries/by-puuid/${puuid}/top?count=5`;

    const masteryRes = await fetch(masteryUrl, {
      headers: { "X-Riot-Token": riotApiKey },
    });

    let topMasteries = [];
    if (masteryRes.ok) {
      topMasteries = await masteryRes.json();
    }

    return new Response(
      JSON.stringify({
        status: "success",
        summoner: {
          puuid,
          gameName: accountData.gameName,
          tagLine: accountData.tagLine,
          top_masteries: topMasteries.map((m: any) => ({
            championId: m.championId,
            championLevel: m.championLevel,
            championPoints: m.championPoints,
          })),
        },
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
