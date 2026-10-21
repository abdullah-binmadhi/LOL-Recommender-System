// Supabase Bridge for League of Legends Champion Recommender
// Handles Google OAuth, session state, cloud recommendations, and mock auth testing.

(function() {
  const SUPABASE_URL = "https://erfybpoymihcepxnaosj.supabase.co";
  const SUPABASE_ANON_KEY = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImVyZnlicG95bWloY2VweG5hb3NqIiwicm9sZSI6ImFub24iLCJpYXQiOjE3NjEwOTYxNzgsImV4cCI6MjA3NjY3MjE3OH0.3DhF2rqIsltczyZdxbkpQI3Kp7HhV_vE6Ym2fDyMJUA";

  let client = null;
  let currentUser = null;

  // Check for mock test mode (?mock_auth=1)
  const urlParams = new URLSearchParams(window.location.search);
  if (urlParams.get("mock_auth") === "1") {
    try { window.localStorage.setItem("mock_auth", "1"); } catch (_) {}
  }
  const isMockAuth = urlParams.get("mock_auth") === "1" || window.localStorage.getItem("mock_auth") === "1";

  if (isMockAuth) {
    currentUser = {
      id: "mock-summoner-uuid-001",
      email: "summoner@example.com",
      full_name: "Mock Summoner",
      avatar_url: "https://ddragon.leagueoflegends.com/cdn/14.20.1/img/profileicon/588.png",
      riot_id: "Faker#T1",
      riot_region: "KR"
    };
  }

  // Initialize official Supabase client if library is available
  if (window.supabase && typeof window.supabase.createClient === "function") {
    try {
      client = window.supabase.createClient(SUPABASE_URL, SUPABASE_ANON_KEY, {
        auth: {
          persistSession: true,
          autoRefreshToken: true,
          detectSessionInUrl: true
        }
      });

      // Listen for auth state changes
      client.auth.onAuthStateChange(async (event, session) => {
        if (session && session.user) {
          const userMeta = session.user.user_metadata || {};
          // Fetch or merge profile
          let profile = null;
          try {
            const { data } = await client
              .from("user_profiles")
              .select("riot_id, riot_region")
              .eq("id", session.user.id)
              .maybeSingle();
            profile = data;
          } catch (e) {
            console.warn("Could not fetch user profile:", e);
          }

          currentUser = {
            id: session.user.id,
            email: session.user.email || "",
            full_name: userMeta.full_name || userMeta.name || "Summoner",
            avatar_url: userMeta.avatar_url || userMeta.picture || "",
            riot_id: (profile && profile.riot_id) || null,
            riot_region: (profile && profile.riot_region) || null
          };
        } else if (!isMockAuth) {
          currentUser = null;
        }

        window.dispatchEvent(new CustomEvent("supabase-auth-changed", { detail: currentUser }));
      });
    } catch (err) {
      console.error("Supabase client initialization error:", err);
    }
  }

  window.supabaseAuth = {
    isMockMode: function() {
      return isMockAuth;
    },

    signInWithGoogle: async function() {
      if (isMockAuth) {
        currentUser = {
          id: "mock-summoner-uuid-001",
          email: "summoner@example.com",
          full_name: "Mock Summoner",
          avatar_url: "https://ddragon.leagueoflegends.com/cdn/14.20.1/img/profileicon/588.png",
          riot_id: "Faker#T1",
          riot_region: "KR"
        };
        window.dispatchEvent(new CustomEvent("supabase-auth-changed", { detail: currentUser }));
        return true;
      }

      if (!client) {
        alert("Authentication client unavailable. Please check your network connection.");
        return false;
      }

      const redirectTo = window.location.origin + window.location.pathname;
      const { data, error } = await client.auth.signInWithOAuth({
        provider: "google",
        options: {
          redirectTo: redirectTo,
          skipBrowserRedirect: true
        }
      });

      if (error) {
        console.error("Google sign-in error:", error);
        alert("Google sign-in failed: " + error.message);
        return false;
      }

      if (data && data.url) {
        // Probe endpoint to detect if provider is not enabled before navigating
        try {
          const probe = await fetch(data.url, { method: "GET" });
          if (probe.status === 400) {
            const probeJson = await probe.json().catch(() => ({}));
            if (probeJson.error_code === "validation_failed" || (probeJson.msg && probeJson.msg.includes("not enabled"))) {
              window.dispatchEvent(new CustomEvent("supabase-oauth-config-required", {
                detail: {
                  provider: "google",
                  error: probeJson.msg || "Unsupported provider: provider is not enabled",
                  dashboardUrl: "https://supabase.com/dashboard/project/erfybpoymihcepxnaosj/auth/providers",
                  callbackUrl: "https://erfybpoymihcepxnaosj.supabase.co/auth/v1/callback"
                }
              }));
              return false;
            }
          }
        } catch (e) {
          // If probe fails or is blocked, continue
          console.warn("OAuth probe warning:", e);
        }

        window.location.href = data.url;
        return true;
      }
      return true;
    },

    enableDemoMode: function() {
      try { window.localStorage.setItem("mock_auth", "1"); } catch (_) {}
      currentUser = {
        id: "mock-summoner-uuid-001",
        email: "summoner@example.com",
        full_name: "Mock Summoner",
        avatar_url: "https://ddragon.leagueoflegends.com/cdn/14.20.1/img/profileicon/588.png",
        riot_id: "Faker#T1",
        riot_region: "KR"
      };
      window.dispatchEvent(new CustomEvent("supabase-auth-changed", { detail: currentUser }));
      return true;
    },

    signOut: async function() {
      if (isMockAuth) {
        window.localStorage.removeItem("mock_auth");
        currentUser = null;
        window.dispatchEvent(new CustomEvent("supabase-auth-changed", { detail: null }));
        return true;
      }

      if (client) {
        await client.auth.signOut();
      }
      currentUser = null;
      window.dispatchEvent(new CustomEvent("supabase-auth-changed", { detail: null }));
      return true;
    },

    getCurrentUserJson: function() {
      return currentUser ? JSON.stringify(currentUser) : "";
    },

    saveRecommendation: async function(recJson) {
      try {
        const payload = typeof recJson === "string" ? JSON.parse(recJson) : recJson;
        if (!client || isMockAuth) {
          console.log("[Supabase Mock] Saved recommendation run for user:", currentUser ? currentUser.id : "anon");
          return true;
        }

        const topChamp = (payload.top10 && payload.top10[0] && payload.top10[0].champion_name) || "Unknown";
        const { error } = await client
          .from("recommendation_history")
          .insert({
            user_id: currentUser ? currentUser.id : null,
            answers: payload.features || {},
            top_champion: topChamp,
            top10_result: payload.top10 || []
          });

        if (error) {
          console.warn("Failed to persist recommendation to Supabase:", error);
          return false;
        }
        return true;
      } catch (err) {
        console.warn("saveRecommendation parsing exception:", err);
        return false;
      }
    },

    updateRiotProfile: async function(riotId, riotRegion) {
      if (currentUser) {
        currentUser.riot_id = riotId;
        currentUser.riot_region = riotRegion;
      }
      if (client && currentUser && !isMockAuth) {
        try {
          await client
            .from("user_profiles")
            .update({ riot_id: riotId, riot_region: riotRegion, updated_at: new Date().toISOString() })
            .eq("id", currentUser.id);
        } catch (e) {
          console.warn("updateRiotProfile error:", e);
        }
      }
      window.dispatchEvent(new CustomEvent("supabase-auth-changed", { detail: currentUser }));
      return true;
    }
  };
})();
