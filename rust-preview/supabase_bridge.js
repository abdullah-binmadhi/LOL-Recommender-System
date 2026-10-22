// Supabase Bridge for League of Legends Champion Recommender
// Handles Google OAuth, session state, cloud recommendations, and mock auth testing.

(function() {
  const SUPABASE_URL = "https://erfybpoymihcepxnaosj.supabase.co";
  const SUPABASE_ANON_KEY = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImVyZnlicG95bWloY2VweG5hb3NqIiwicm9sZSI6ImFub24iLCJpYXQiOjE3NjEwOTYxNzgsImV4cCI6MjA3NjY3MjE3OH0.3DhF2rqIsltczyZdxbkpQI3Kp7HhV_vE6Ym2fDyMJUA";

  let client = null;
  let currentUser = null;

  // Retrieve saved local Riot profile if any
  let savedRiotId = null;
  let savedRiotRegion = null;
  try {
    savedRiotId = window.localStorage.getItem("lol_riot_id");
    savedRiotRegion = window.localStorage.getItem("lol_riot_region");
  } catch (_) {}

  // Check for mock test mode (?mock_auth=1)
  const urlParams = new URLSearchParams(window.location.search);
  if (urlParams.get("mock_auth") === "1") {
    try { window.localStorage.setItem("mock_auth", "1"); } catch (_) {}
  }
  const isMockAuth = urlParams.get("mock_auth") === "1" || window.localStorage.getItem("mock_auth") === "1";

  if (isMockAuth || savedRiotId) {
    currentUser = {
      id: "mock-summoner-uuid-001",
      email: "summoner@example.com",
      full_name: savedRiotId ? savedRiotId.split("#")[0] : "SHENTRICK",
      avatar_url: "https://ddragon.leagueoflegends.com/cdn/14.20.1/img/profileicon/588.png",
      riot_id: savedRiotId || "SHENTRICK#SHEN",
      riot_region: savedRiotRegion || "ME"
    };
  }

  // Check for OAuth error in URL hash
  if (typeof window !== "undefined" && window.location && window.location.hash) {
    if (window.location.hash.includes("error_description")) {
      try {
        const hashParams = new URLSearchParams(window.location.hash.substring(1));
        const errorDesc = hashParams.get("error_description") || hashParams.get("error");
        if (errorDesc) {
          console.warn("Supabase Auth error from OAuth callback:", errorDesc);
          alert("Authentication notice: " + decodeURIComponent(errorDesc.replace(/\+/g, " ")));
        }
      } catch (_) {}
    }
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

      // Clean access_token from hash after client initialization
      if (typeof window !== "undefined" && window.location && window.location.hash && window.location.hash.includes("access_token")) {
        setTimeout(() => {
          if (window.history && window.history.replaceState) {
            const cleanUrl = window.location.origin + window.location.pathname + window.location.search;
            window.history.replaceState(null, "", cleanUrl);
          }
        }, 500);
      }

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
          redirectTo: redirectTo
        }
      });

      if (error) {
        console.error("Google sign-in error:", error);
        alert("Google sign-in failed: " + error.message);
        return false;
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
      try {
        window.localStorage.setItem("lol_riot_id", riotId);
        window.localStorage.setItem("lol_riot_region", riotRegion);
      } catch (_) {}

      if (currentUser) {
        currentUser.riot_id = riotId;
        currentUser.riot_region = riotRegion;
        currentUser.full_name = riotId.split("#")[0] || currentUser.full_name;
      } else {
        currentUser = {
          id: "local-summoner-" + Date.now(),
          email: "",
          full_name: riotId.split("#")[0] || "Summoner",
          avatar_url: "https://ddragon.leagueoflegends.com/cdn/14.20.1/img/profileicon/588.png",
          riot_id: riotId,
          riot_region: riotRegion
        };
      }

      if (client && currentUser && !isMockAuth && currentUser.id && !currentUser.id.startsWith("local-")) {
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
    },

    fetchRiotTelemetry: async function(riotId, region, apiKey) {
      const targetRiotId = riotId || (currentUser && currentUser.riot_id) || "SHENTRICK#SHEN";
      const targetRegion = region || (currentUser && currentUser.riot_region) || "ME";

      // Extract recommended champions from cached recommendation runs to calculate actual calibration accuracy
      let recommendedChamps = [];
      try {
        const lastRec = window.localStorage.getItem("lastRecommendationResult");
        if (lastRec) {
          const parsed = JSON.parse(lastRec);
          const topList = parsed.top10 || parsed.top_champions || [];
          recommendedChamps = topList.map(c => (typeof c === "string" ? c : c.champion_name || c.championName)).filter(Boolean);
        }
      } catch (_) {}

      try {
        const edgeFunctionUrl = `${SUPABASE_URL}/functions/v1/riot-proxy`;
        const res = await fetch(edgeFunctionUrl, {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
            "apikey": SUPABASE_ANON_KEY,
            "Authorization": `Bearer ${SUPABASE_ANON_KEY}`
          },
          body: JSON.stringify({
            riot_id: targetRiotId,
            region: targetRegion.toLowerCase(),
            api_key: apiKey || undefined,
            recommended_champions: recommendedChamps
          })
        });

        if (!res.ok) {
          const errText = await res.text();
          console.warn("Riot proxy returned non-200:", res.status, errText);
        } else {
          const telemetryData = await res.json();
          try {
            window.localStorage.setItem("lol_riot_telemetry", JSON.stringify(telemetryData));
          } catch (_) {}

          window.dispatchEvent(new CustomEvent("riot-telemetry-synced", { detail: telemetryData }));
          return telemetryData;
        }
      } catch (err) {
        console.warn("fetchRiotTelemetry exception:", err);
      }

      return null;
    }
  };
})();
