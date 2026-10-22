-- Supabase Schema for League of Legends Champion Recommender
-- Supports cross-device user sync, recommendation telemetry, and community ML feedback

-- 1. User Profiles
CREATE TABLE IF NOT EXISTS public.user_profiles (
    id UUID PRIMARY KEY REFERENCES auth.users(id) ON DELETE CASCADE,
    email TEXT,
    full_name TEXT,
    avatar_url TEXT,
    riot_id TEXT,
    riot_region TEXT,
    preferred_role TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT timezone('utc'::text, now()),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT timezone('utc'::text, now())
);

-- Idempotently ensure all profile columns exist even if table was created in an earlier migration
ALTER TABLE public.user_profiles
    ADD COLUMN IF NOT EXISTS email TEXT,
    ADD COLUMN IF NOT EXISTS full_name TEXT,
    ADD COLUMN IF NOT EXISTS avatar_url TEXT,
    ADD COLUMN IF NOT EXISTS riot_id TEXT,
    ADD COLUMN IF NOT EXISTS riot_region TEXT,
    ADD COLUMN IF NOT EXISTS preferred_role TEXT;

ALTER TABLE public.user_profiles ENABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS "Users can view own profile" ON public.user_profiles;
CREATE POLICY "Users can view own profile"
    ON public.user_profiles FOR SELECT
    USING (auth.uid() = id);

DROP POLICY IF EXISTS "Users can insert own profile" ON public.user_profiles;
CREATE POLICY "Users can insert own profile"
    ON public.user_profiles FOR INSERT
    WITH CHECK (auth.uid() = id);

DROP POLICY IF EXISTS "Users can update own profile" ON public.user_profiles;
CREATE POLICY "Users can update own profile"
    ON public.user_profiles FOR UPDATE
    USING (auth.uid() = id);

-- Automatic Profile Sync from Google OAuth (auth.users)
CREATE OR REPLACE FUNCTION public.handle_new_user()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO public.user_profiles (id, email, full_name, avatar_url, updated_at)
    VALUES (
        NEW.id,
        NEW.email,
        COALESCE(NEW.raw_user_meta_data->>'full_name', NEW.raw_user_meta_data->>'name', ''),
        COALESCE(NEW.raw_user_meta_data->>'avatar_url', NEW.raw_user_meta_data->>'picture', ''),
        timezone('utc'::text, now())
    )
    ON CONFLICT (id) DO UPDATE SET
        email = EXCLUDED.email,
        full_name = CASE WHEN EXCLUDED.full_name <> '' THEN EXCLUDED.full_name ELSE public.user_profiles.full_name END,
        avatar_url = CASE WHEN EXCLUDED.avatar_url <> '' THEN EXCLUDED.avatar_url ELSE public.user_profiles.avatar_url END,
        updated_at = timezone('utc'::text, now());
    RETURN NEW;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

CREATE OR REPLACE TRIGGER on_auth_user_created
    AFTER INSERT OR UPDATE ON auth.users
    FOR EACH ROW
    EXECUTE FUNCTION public.handle_new_user();

-- 2. Recommendation History & Feedback
CREATE TABLE IF NOT EXISTS public.recommendation_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES auth.users(id) ON DELETE SET NULL,
    session_id TEXT,
    answers JSONB NOT NULL,
    top_champion TEXT NOT NULL,
    top10_result JSONB NOT NULL,
    feedback_rating INT CHECK (feedback_rating >= 1 AND feedback_rating <= 5),
    feedback_text TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT timezone('utc'::text, now())
);

ALTER TABLE public.recommendation_history ENABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS "Allow public insert for recommendation runs" ON public.recommendation_history;
CREATE POLICY "Allow public insert for recommendation runs"
    ON public.recommendation_history FOR INSERT
    WITH CHECK (true);

DROP POLICY IF EXISTS "Users can view own recommendations" ON public.recommendation_history;
CREATE POLICY "Users can view own recommendations"
    ON public.recommendation_history FOR SELECT
    USING (auth.uid() = user_id OR auth.uid() IS NULL);

-- 3. Community Champion Telemetry
CREATE TABLE IF NOT EXISTS public.community_champion_stats (
    champion_name TEXT PRIMARY KEY,
    total_recommendations BIGINT NOT NULL DEFAULT 0,
    positive_ratings BIGINT NOT NULL DEFAULT 0,
    total_ratings BIGINT NOT NULL DEFAULT 0,
    avg_satisfaction NUMERIC(3, 2) NOT NULL DEFAULT 0.0,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT timezone('utc'::text, now())
);

ALTER TABLE public.community_champion_stats ENABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS "Public read for community champion stats" ON public.community_champion_stats;
CREATE POLICY "Public read for community champion stats"
    ON public.community_champion_stats FOR SELECT
    USING (true);

-- 4. Automatic Aggregation Trigger for Community Telemetry
CREATE OR REPLACE FUNCTION public.handle_new_recommendation()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO public.community_champion_stats (champion_name, total_recommendations, updated_at)
    VALUES (NEW.top_champion, 1, timezone('utc'::text, now()))
    ON CONFLICT (champion_name) DO UPDATE
    SET total_recommendations = community_champion_stats.total_recommendations + 1,
        updated_at = timezone('utc'::text, now());
        
    IF NEW.feedback_rating IS NOT NULL THEN
        UPDATE public.community_champion_stats
        SET total_ratings = total_ratings + 1,
            positive_ratings = CASE WHEN NEW.feedback_rating >= 4 THEN positive_ratings + 1 ELSE positive_ratings END,
            avg_satisfaction = ROUND(((avg_satisfaction * total_ratings) + NEW.feedback_rating) / (total_ratings + 1), 2),
            updated_at = timezone('utc'::text, now())
        WHERE champion_name = NEW.top_champion;
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

CREATE OR REPLACE TRIGGER on_recommendation_created
    AFTER INSERT ON public.recommendation_history
    FOR EACH ROW
    EXECUTE FUNCTION public.handle_new_recommendation();
