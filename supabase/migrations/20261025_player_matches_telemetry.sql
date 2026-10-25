-- Supabase Migration: Player Matches & Verified Telemetry Cache
-- Supports authentic Match-V5 persistence, incremental ingestion, queue-mode filtering, and fast cold-loads

-- 1. Player Matches Table
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

-- Idempotently ensure columns exist
ALTER TABLE public.player_matches
    ADD COLUMN IF NOT EXISTS queue_name TEXT,
    ADD COLUMN IF NOT EXISTS game_creation BIGINT,
    ADD COLUMN IF NOT EXISTS lane_opponent_champion TEXT,
    ADD COLUMN IF NOT EXISTS allied_team JSONB,
    ADD COLUMN IF NOT EXISTS enemy_team JSONB;

-- Performance Indexes
CREATE INDEX IF NOT EXISTS idx_player_matches_puuid_creation 
    ON public.player_matches (puuid, game_creation DESC);
CREATE INDEX IF NOT EXISTS idx_player_matches_riot_id 
    ON public.player_matches (riot_id);
CREATE INDEX IF NOT EXISTS idx_player_matches_champion 
    ON public.player_matches (champion_name);
CREATE INDEX IF NOT EXISTS idx_player_matches_queue 
    ON public.player_matches (queue_id);

-- Row Level Security
ALTER TABLE public.player_matches ENABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS "Allow public read for player matches" ON public.player_matches;
CREATE POLICY "Allow public read for player matches"
    ON public.player_matches FOR SELECT
    USING (true);

DROP POLICY IF EXISTS "Allow service role full access on player matches" ON public.player_matches;
CREATE POLICY "Allow service role full access on player matches"
    ON public.player_matches FOR ALL
    TO service_role
    USING (true)
    WITH CHECK (true);

-- 2. Player Telemetry & Radar Cache Table
CREATE TABLE IF NOT EXISTS public.player_telemetry_cache (
    puuid TEXT PRIMARY KEY,
    riot_id TEXT NOT NULL,
    region TEXT NOT NULL,
    radar_scores JSONB NOT NULL DEFAULT '[]'::jsonb,
    phase_ratings JSONB NOT NULL DEFAULT '[]'::jsonb,
    telemetry_benchmarks JSONB NOT NULL DEFAULT '[]'::jsonb,
    recurring_mistakes JSONB NOT NULL DEFAULT '[]'::jsonb,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT timezone('utc'::text, now())
);

ALTER TABLE public.player_telemetry_cache ENABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS "Allow public read for telemetry cache" ON public.player_telemetry_cache;
CREATE POLICY "Allow public read for telemetry cache"
    ON public.player_telemetry_cache FOR SELECT
    USING (true);

DROP POLICY IF EXISTS "Allow service role full access on telemetry cache" ON public.player_telemetry_cache;
CREATE POLICY "Allow service role full access on telemetry cache"
    ON public.player_telemetry_cache FOR ALL
    TO service_role
    USING (true)
    WITH CHECK (true);
