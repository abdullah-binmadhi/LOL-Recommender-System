# AI Prompt: Supabase Backend Setup for LoL Champion Recommender

## Project Context

This is a League of Legends Champion Recommender application that currently runs entirely client-side using localStorage. We need to add a Supabase backend to:

1. Store user registration data and recommendation history
2. Create an admin dashboard at `/admin` route with authentication
3. Enable cross-device synchronization of user preferences
4. Track analytics and usage patterns

**Current Tech Stack:** Vanilla HTML/CSS/JavaScript (no frameworks)  
**Target Backend:** Supabase (PostgreSQL + Auth + Storage)  
**Admin Route:** `/admin` with authentication required

---

## Task Overview

Create a complete Supabase backend integration with the following components:

### 1. Supabase Project Setup
- Initialize new Supabase project
- Configure authentication providers (Email/Password)
- Set up Row Level Security (RLS) policies
- Create database schema for users and recommendations

### 2. Database Schema
Design and implement tables with proper relationships and RLS policies.

### 3. Admin Dashboard (`/admin` route)
- Protected route requiring admin authentication
- View all registered users
- View user recommendation history
- Export user data to CSV
- Analytics dashboard showing usage statistics

### 4. Frontend Integration
- Migrate from localStorage to Supabase
- Implement user registration/login
- Sync recommendation history to database
- Keep existing UI/UX intact

---

## Detailed Requirements

## Part 1: Supabase Database Schema

Create the following tables with proper constraints and RLS policies:

### Table 1: `users`
```sql
CREATE TABLE users (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  email TEXT UNIQUE NOT NULL,
  created_at TIMESTAMPTZ DEFAULT NOW(),
  updated_at TIMESTAMPTZ DEFAULT NOW(),
  is_admin BOOLEAN DEFAULT FALSE,
  last_login TIMESTAMPTZ,
  metadata JSONB DEFAULT '{}'::jsonb
);

-- Enable RLS
ALTER TABLE users ENABLE ROW LEVEL SECURITY;

-- Users can only read their own data
CREATE POLICY "Users can view own data"
  ON users FOR SELECT
  USING (auth.uid() = id);

-- Admins can view all users
CREATE POLICY "Admins can view all users"
  ON users FOR SELECT
  USING (
    EXISTS (
      SELECT 1 FROM users
      WHERE id = auth.uid() AND is_admin = true
    )
  );
```

### Table 2: `user_answers`
Store questionnaire responses for each user session.

```sql
CREATE TABLE user_answers (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID REFERENCES users(id) ON DELETE CASCADE,
  session_id UUID NOT NULL,
  question_id TEXT NOT NULL,
  question_text TEXT NOT NULL,
  answer_value INTEGER NOT NULL,
  answer_text TEXT,
  created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Enable RLS
ALTER TABLE user_answers ENABLE ROW LEVEL SECURITY;

-- Users can view/insert their own answers
CREATE POLICY "Users can manage own answers"
  ON user_answers FOR ALL
  USING (auth.uid() = user_id);

-- Admins can view all answers
CREATE POLICY "Admins can view all answers"
  ON user_answers FOR SELECT
  USING (
    EXISTS (
      SELECT 1 FROM users
      WHERE id = auth.uid() AND is_admin = true
    )
  );

-- Indexes for performance
CREATE INDEX idx_user_answers_user_id ON user_answers(user_id);
CREATE INDEX idx_user_answers_session_id ON user_answers(session_id);
CREATE INDEX idx_user_answers_created_at ON user_answers(created_at DESC);
```

### Table 3: `recommendations`
Store champion recommendations for each user session.

```sql
CREATE TABLE recommendations (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID REFERENCES users(id) ON DELETE CASCADE,
  session_id UUID NOT NULL,
  champion_name TEXT NOT NULL,
  champion_role TEXT NOT NULL,
  rank INTEGER NOT NULL CHECK (rank >= 1 AND rank <= 5),
  
  -- ML Algorithm Scores
  rf_score DECIMAL(5,2) NOT NULL,
  dt_score DECIMAL(5,2) NOT NULL,
  knn_score DECIMAL(5,2) NOT NULL,
  average_score DECIMAL(5,2) NOT NULL,
  weighted_score DECIMAL(5,2) NOT NULL,
  
  -- Champion attributes
  champion_data JSONB NOT NULL,
  
  -- Timestamps
  created_at TIMESTAMPTZ DEFAULT NOW(),
  
  UNIQUE(session_id, rank)
);

-- Enable RLS
ALTER TABLE recommendations ENABLE ROW LEVEL SECURITY;

-- Users can manage their own recommendations
CREATE POLICY "Users can manage own recommendations"
  ON recommendations FOR ALL
  USING (auth.uid() = user_id);

-- Admins can view all recommendations
CREATE POLICY "Admins can view all recommendations"
  ON recommendations FOR SELECT
  USING (
    EXISTS (
      SELECT 1 FROM users
      WHERE id = auth.uid() AND is_admin = true
    )
  );

-- Indexes
CREATE INDEX idx_recommendations_user_id ON recommendations(user_id);
CREATE INDEX idx_recommendations_session_id ON recommendations(session_id);
CREATE INDEX idx_recommendations_created_at ON recommendations(created_at DESC);
CREATE INDEX idx_recommendations_champion_name ON recommendations(champion_name);
```

### Table 4: `sessions`
Track user sessions and completion status.

```sql
CREATE TABLE sessions (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID REFERENCES users(id) ON DELETE CASCADE,
  started_at TIMESTAMPTZ DEFAULT NOW(),
  completed_at TIMESTAMPTZ,
  is_completed BOOLEAN DEFAULT FALSE,
  user_agent TEXT,
  ip_address INET,
  
  -- Summary statistics
  total_questions_answered INTEGER DEFAULT 0,
  total_recommendations INTEGER DEFAULT 0,
  
  CONSTRAINT valid_completion CHECK (
    (is_completed = false AND completed_at IS NULL) OR
    (is_completed = true AND completed_at IS NOT NULL)
  )
);

-- Enable RLS
ALTER TABLE sessions ENABLE ROW LEVEL SECURITY;

-- Users can manage their own sessions
CREATE POLICY "Users can manage own sessions"
  ON sessions FOR ALL
  USING (auth.uid() = user_id);

-- Admins can view all sessions
CREATE POLICY "Admins can view all sessions"
  ON sessions FOR SELECT
  USING (
    EXISTS (
      SELECT 1 FROM users
      WHERE id = auth.uid() AND is_admin = true
    )
  );

-- Indexes
CREATE INDEX idx_sessions_user_id ON sessions(user_id);
CREATE INDEX idx_sessions_created_at ON sessions(started_at DESC);
CREATE INDEX idx_sessions_completed ON sessions(is_completed);
```

### Table 5: `analytics_events`
Track user interactions and events.

```sql
CREATE TABLE analytics_events (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID REFERENCES users(id) ON DELETE SET NULL,
  session_id UUID,
  event_type TEXT NOT NULL,
  event_data JSONB DEFAULT '{}'::jsonb,
  created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Enable RLS
ALTER TABLE analytics_events ENABLE ROW LEVEL SECURITY;

-- Users can insert their own events
CREATE POLICY "Users can insert own events"
  ON analytics_events FOR INSERT
  WITH CHECK (auth.uid() = user_id);

-- Admins can view all events
CREATE POLICY "Admins can view all events"
  ON analytics_events FOR SELECT
  USING (
    EXISTS (
      SELECT 1 FROM users
      WHERE id = auth.uid() AND is_admin = true
    )
  );

-- Indexes
CREATE INDEX idx_analytics_events_user_id ON analytics_events(user_id);
CREATE INDEX idx_analytics_events_type ON analytics_events(event_type);
CREATE INDEX idx_analytics_events_created_at ON analytics_events(created_at DESC);
```

---

## Part 2: Database Functions & Triggers

### Function: Update `updated_at` timestamp
```sql
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Apply to users table
CREATE TRIGGER update_users_updated_at
  BEFORE UPDATE ON users
  FOR EACH ROW
  EXECUTE FUNCTION update_updated_at_column();
```

### Function: Get User Statistics (for admin dashboard)
```sql
CREATE OR REPLACE FUNCTION get_user_statistics()
RETURNS TABLE (
  total_users BIGINT,
  total_sessions BIGINT,
  total_recommendations BIGINT,
  active_users_7d BIGINT,
  active_users_30d BIGINT
) AS $$
BEGIN
  RETURN QUERY
  SELECT
    (SELECT COUNT(*) FROM users)::BIGINT,
    (SELECT COUNT(*) FROM sessions)::BIGINT,
    (SELECT COUNT(*) FROM recommendations)::BIGINT,
    (SELECT COUNT(DISTINCT user_id) FROM sessions 
     WHERE started_at >= NOW() - INTERVAL '7 days')::BIGINT,
    (SELECT COUNT(DISTINCT user_id) FROM sessions 
     WHERE started_at >= NOW() - INTERVAL '30 days')::BIGINT;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;
```

### Function: Get Top Recommended Champions
```sql
CREATE OR REPLACE FUNCTION get_top_champions(limit_count INT DEFAULT 10)
RETURNS TABLE (
  champion_name TEXT,
  recommendation_count BIGINT,
  avg_weighted_score DECIMAL
) AS $$
BEGIN
  RETURN QUERY
  SELECT
    r.champion_name,
    COUNT(*)::BIGINT AS recommendation_count,
    AVG(r.weighted_score)::DECIMAL(5,2) AS avg_weighted_score
  FROM recommendations r
  GROUP BY r.champion_name
  ORDER BY recommendation_count DESC, avg_weighted_score DESC
  LIMIT limit_count;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;
```

---

## Part 3: Admin Dashboard HTML (`/admin/index.html`)

Create a new admin dashboard page with the following features:

### File: `/admin/index.html`

**Requirements:**
1. **Authentication Check**: Verify user is logged in and has `is_admin = true`
2. **Dashboard Sections**:
   - User Management Table (with search/filter)
   - Recommendation History View
   - Analytics Charts (users over time, popular champions)
   - Export to CSV functionality
3. **Styling**: Match existing app design (dark theme, modern UI)
4. **Responsive**: Works on desktop and tablet

**Key Features to Include:**

```html
<!-- Admin Dashboard Structure -->
<div id="admin-dashboard">
  <!-- Header with logout button -->
  <header>
    <h1>Admin Dashboard</h1>
    <button id="logout-btn">Logout</button>
  </header>

  <!-- Statistics Cards -->
  <section id="stats-cards">
    <div class="stat-card">
      <h3>Total Users</h3>
      <p id="total-users">-</p>
    </div>
    <div class="stat-card">
      <h3>Total Sessions</h3>
      <p id="total-sessions">-</p>
    </div>
    <div class="stat-card">
      <h3>Active Users (7d)</h3>
      <p id="active-users-7d">-</p>
    </div>
    <div class="stat-card">
      <h3>Total Recommendations</h3>
      <p id="total-recommendations">-</p>
    </div>
  </section>

  <!-- Users Table -->
  <section id="users-section">
    <h2>Registered Users</h2>
    <div class="table-controls">
      <input type="text" id="search-users" placeholder="Search by email...">
      <button id="export-users-btn">Export to CSV</button>
    </div>
    <table id="users-table">
      <thead>
        <tr>
          <th>Email</th>
          <th>Created At</th>
          <th>Last Login</th>
          <th>Sessions</th>
          <th>Actions</th>
        </tr>
      </thead>
      <tbody id="users-table-body">
        <!-- Populated dynamically -->
      </tbody>
    </table>
  </section>

  <!-- Recommendations Table -->
  <section id="recommendations-section">
    <h2>Recent Recommendations</h2>
    <div class="table-controls">
      <select id="filter-champion">
        <option value="">All Champions</option>
        <!-- Populated dynamically -->
      </select>
      <button id="export-recommendations-btn">Export to CSV</button>
    </div>
    <table id="recommendations-table">
      <thead>
        <tr>
          <th>User</th>
          <th>Champion</th>
          <th>Rank</th>
          <th>RF Score</th>
          <th>DT Score</th>
          <th>KNN Score</th>
          <th>Weighted Score</th>
          <th>Created At</th>
        </tr>
      </thead>
      <tbody id="recommendations-table-body">
        <!-- Populated dynamically -->
      </tbody>
    </table>
  </section>

  <!-- Analytics Charts -->
  <section id="analytics-section">
    <h2>Analytics</h2>
    <div id="charts-container">
      <div class="chart-card">
        <h3>Top 10 Recommended Champions</h3>
        <canvas id="top-champions-chart"></canvas>
      </div>
      <div class="chart-card">
        <h3>Users Over Time</h3>
        <canvas id="users-timeline-chart"></canvas>
      </div>
    </div>
  </section>
</div>
```

---

## Part 4: Supabase Client Integration

### File: `/admin/supabase-client.js`

**Initialize Supabase client:**

```javascript
// Supabase configuration
const SUPABASE_URL = 'YOUR_SUPABASE_URL'; // Replace with your Supabase URL
const SUPABASE_ANON_KEY = 'YOUR_SUPABASE_ANON_KEY'; // Replace with your anon key

// Initialize Supabase client
const supabaseClient = supabase.createClient(SUPABASE_URL, SUPABASE_ANON_KEY);

// Auth state management
let currentUser = null;

// Check if user is admin
async function checkAdminAccess() {
  const { data: { user }, error } = await supabaseClient.auth.getUser();
  
  if (error || !user) {
    window.location.href = '/admin/login.html';
    return false;
  }

  // Check if user is admin
  const { data: userData, error: userError } = await supabaseClient
    .from('users')
    .select('is_admin')
    .eq('id', user.id)
    .single();

  if (userError || !userData?.is_admin) {
    alert('Access Denied: Admin privileges required');
    window.location.href = '/';
    return false;
  }

  currentUser = user;
  return true;
}

// Fetch dashboard statistics
async function fetchDashboardStats() {
  const { data, error } = await supabaseClient.rpc('get_user_statistics');
  
  if (error) {
    console.error('Error fetching stats:', error);
    return null;
  }
  
  return data[0];
}

// Fetch all users with pagination
async function fetchUsers(page = 1, limit = 50, searchTerm = '') {
  let query = supabaseClient
    .from('users')
    .select('id, email, created_at, last_login, is_admin', { count: 'exact' })
    .order('created_at', { ascending: false })
    .range((page - 1) * limit, page * limit - 1);

  if (searchTerm) {
    query = query.ilike('email', `%${searchTerm}%`);
  }

  const { data, error, count } = await query;

  if (error) {
    console.error('Error fetching users:', error);
    return { users: [], total: 0 };
  }

  return { users: data, total: count };
}

// Fetch user sessions count
async function fetchUserSessionsCount(userId) {
  const { count, error } = await supabaseClient
    .from('sessions')
    .select('id', { count: 'exact', head: true })
    .eq('user_id', userId);

  if (error) {
    console.error('Error fetching sessions count:', error);
    return 0;
  }

  return count;
}

// Fetch recommendations with filters
async function fetchRecommendations(filters = {}) {
  let query = supabaseClient
    .from('recommendations')
    .select(`
      id,
      user_id,
      champion_name,
      champion_role,
      rank,
      rf_score,
      dt_score,
      knn_score,
      weighted_score,
      created_at,
      users(email)
    `)
    .order('created_at', { ascending: false })
    .limit(100);

  if (filters.championName) {
    query = query.eq('champion_name', filters.championName);
  }

  const { data, error } = await query;

  if (error) {
    console.error('Error fetching recommendations:', error);
    return [];
  }

  return data;
}

// Export users to CSV
async function exportUsersToCSV() {
  const { users } = await fetchUsers(1, 10000); // Fetch all users
  
  const csvContent = [
    ['Email', 'Created At', 'Last Login', 'Is Admin'].join(','),
    ...users.map(user => [
      user.email,
      new Date(user.created_at).toISOString(),
      user.last_login ? new Date(user.last_login).toISOString() : 'Never',
      user.is_admin ? 'Yes' : 'No'
    ].join(','))
  ].join('\n');

  downloadCSV(csvContent, 'users_export.csv');
}

// Export recommendations to CSV
async function exportRecommendationsToCSV() {
  const recommendations = await fetchRecommendations();
  
  const csvContent = [
    ['User Email', 'Champion', 'Role', 'Rank', 'RF Score', 'DT Score', 'KNN Score', 'Weighted Score', 'Created At'].join(','),
    ...recommendations.map(rec => [
      rec.users.email,
      rec.champion_name,
      rec.champion_role,
      rec.rank,
      rec.rf_score,
      rec.dt_score,
      rec.knn_score,
      rec.weighted_score,
      new Date(rec.created_at).toISOString()
    ].join(','))
  ].join('\n');

  downloadCSV(csvContent, 'recommendations_export.csv');
}

// Helper: Download CSV
function downloadCSV(content, filename) {
  const blob = new Blob([content], { type: 'text/csv;charset=utf-8;' });
  const link = document.createElement('a');
  const url = URL.createObjectURL(blob);
  link.setAttribute('href', url);
  link.setAttribute('download', filename);
  link.style.visibility = 'hidden';
  document.body.appendChild(link);
  link.click();
  document.body.removeChild(link);
}

// Logout function
async function logout() {
  await supabaseClient.auth.signOut();
  window.location.href = '/admin/login.html';
}
```

---

## Part 5: Admin Login Page

### File: `/admin/login.html`

**Simple login form with Supabase authentication:**

```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Admin Login - LoL Champion Recommender</title>
  <style>
    * {
      margin: 0;
      padding: 0;
      box-sizing: border-box;
    }

    body {
      font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
      background: linear-gradient(135deg, #1a1a2e 0%, #16213e 100%);
      display: flex;
      justify-content: center;
      align-items: center;
      min-height: 100vh;
      color: #fff;
    }

    .login-container {
      background: rgba(255, 255, 255, 0.1);
      padding: 40px;
      border-radius: 15px;
      box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
      backdrop-filter: blur(10px);
      max-width: 400px;
      width: 100%;
    }

    h1 {
      text-align: center;
      margin-bottom: 30px;
      color: #ffd700;
    }

    .form-group {
      margin-bottom: 20px;
    }

    label {
      display: block;
      margin-bottom: 8px;
      font-weight: 500;
    }

    input {
      width: 100%;
      padding: 12px;
      border: 2px solid rgba(255, 255, 255, 0.2);
      border-radius: 8px;
      background: rgba(255, 255, 255, 0.1);
      color: #fff;
      font-size: 16px;
      transition: border-color 0.3s;
    }

    input:focus {
      outline: none;
      border-color: #ffd700;
    }

    button {
      width: 100%;
      padding: 12px;
      background: linear-gradient(135deg, #ffd700 0%, #ffed4e 100%);
      color: #1a1a2e;
      border: none;
      border-radius: 8px;
      font-size: 16px;
      font-weight: bold;
      cursor: pointer;
      transition: transform 0.2s;
    }

    button:hover {
      transform: translateY(-2px);
    }

    button:disabled {
      opacity: 0.5;
      cursor: not-allowed;
    }

    .error-message {
      background: rgba(255, 0, 0, 0.2);
      border: 1px solid #ff4444;
      padding: 10px;
      border-radius: 8px;
      margin-bottom: 20px;
      display: none;
    }

    .back-link {
      text-align: center;
      margin-top: 20px;
    }

    .back-link a {
      color: #ffd700;
      text-decoration: none;
    }
  </style>
</head>
<body>
  <div class="login-container">
    <h1>🔐 Admin Login</h1>
    
    <div class="error-message" id="error-message"></div>

    <form id="login-form">
      <div class="form-group">
        <label for="email">Email</label>
        <input type="email" id="email" required autocomplete="email">
      </div>

      <div class="form-group">
        <label for="password">Password</label>
        <input type="password" id="password" required autocomplete="current-password">
      </div>

      <button type="submit" id="login-btn">Login</button>
    </form>

    <div class="back-link">
      <a href="/">← Back to Home</a>
    </div>
  </div>

  <script src="https://cdn.jsdelivr.net/npm/@supabase/supabase-js@2"></script>
  <script>
    const SUPABASE_URL = 'YOUR_SUPABASE_URL';
    const SUPABASE_ANON_KEY = 'YOUR_SUPABASE_ANON_KEY';
    const supabaseClient = supabase.createClient(SUPABASE_URL, SUPABASE_ANON_KEY);

    const loginForm = document.getElementById('login-form');
    const loginBtn = document.getElementById('login-btn');
    const errorMessage = document.getElementById('error-message');

    loginForm.addEventListener('submit', async (e) => {
      e.preventDefault();
      
      const email = document.getElementById('email').value;
      const password = document.getElementById('password').value;

      loginBtn.disabled = true;
      loginBtn.textContent = 'Logging in...';
      errorMessage.style.display = 'none';

      try {
        // Sign in with Supabase
        const { data, error } = await supabaseClient.auth.signInWithPassword({
          email,
          password
        });

        if (error) throw error;

        // Check if user is admin
        const { data: userData, error: userError } = await supabaseClient
          .from('users')
          .select('is_admin')
          .eq('id', data.user.id)
          .single();

        if (userError) throw userError;

        if (!userData.is_admin) {
          throw new Error('Access denied: Admin privileges required');
        }

        // Update last login
        await supabaseClient
          .from('users')
          .update({ last_login: new Date().toISOString() })
          .eq('id', data.user.id);

        // Redirect to admin dashboard
        window.location.href = '/admin/index.html';

      } catch (error) {
        console.error('Login error:', error);
        errorMessage.textContent = error.message;
        errorMessage.style.display = 'block';
      } finally {
        loginBtn.disabled = false;
        loginBtn.textContent = 'Login';
      }
    });
  </script>
</body>
</html>
```

---

## Part 6: Main App Integration

### Update `src/index.html` to integrate Supabase

**Add Supabase CDN** (in `<head>` section):
```html
<script src="https://cdn.jsdelivr.net/npm/@supabase/supabase-js@2"></script>
```

**Add initialization code** (after existing ML code):
```javascript
// Supabase configuration
const SUPABASE_URL = 'YOUR_SUPABASE_URL';
const SUPABASE_ANON_KEY = 'YOUR_SUPABASE_ANON_KEY';
const supabase = supabase.createClient(SUPABASE_URL, SUPABASE_ANON_KEY);

// Current session ID
let currentSessionId = null;
let currentUserId = null;

// Check authentication status
async function checkAuth() {
  const { data: { user } } = await supabase.auth.getUser();
  
  if (user) {
    currentUserId = user.id;
    // Update or insert user record
    await supabase
      .from('users')
      .upsert({ 
        id: user.id, 
        email: user.email,
        last_login: new Date().toISOString()
      });
  }
  
  return user;
}

// Save questionnaire session to Supabase
async function saveSessionToSupabase() {
  if (!currentUserId) return;

  currentSessionId = crypto.randomUUID();

  // Insert session
  await supabase.from('sessions').insert({
    id: currentSessionId,
    user_id: currentUserId,
    user_agent: navigator.userAgent,
    total_questions_answered: Object.keys(userAnswers).length
  });

  // Save all answers
  const answersToInsert = Object.entries(userAnswers).map(([questionId, answer]) => ({
    user_id: currentUserId,
    session_id: currentSessionId,
    question_id: questionId,
    question_text: answer.questionText || '',
    answer_value: answer.value,
    answer_text: answer.text || ''
  }));

  await supabase.from('user_answers').insert(answersToInsert);
}

// Save recommendations to Supabase
async function saveRecommendationsToSupabase(top5Champions) {
  if (!currentUserId || !currentSessionId) return;

  const recommendationsToInsert = top5Champions.map((champ, index) => ({
    user_id: currentUserId,
    session_id: currentSessionId,
    champion_name: champ.championName,
    champion_role: champ.role,
    rank: index + 1,
    rf_score: champ.rfScore,
    dt_score: champ.dtScore,
    knn_score: champ.knnScore,
    average_score: champ.averageScore,
    weighted_score: champ.weightedScore,
    champion_data: champ
  }));

  await supabase.from('recommendations').insert(recommendationsToInsert);

  // Update session as completed
  await supabase
    .from('sessions')
    .update({
      completed_at: new Date().toISOString(),
      is_completed: true,
      total_recommendations: top5Champions.length
    })
    .eq('id', currentSessionId);
}

// Track analytics events
async function trackEvent(eventType, eventData = {}) {
  if (!currentUserId) return;

  await supabase.from('analytics_events').insert({
    user_id: currentUserId,
    session_id: currentSessionId,
    event_type: eventType,
    event_data: eventData
  });
}
```

**Modify existing functions to call Supabase:**

1. When questionnaire starts: Call `saveSessionToSupabase()`
2. When recommendations are generated: Call `saveRecommendationsToSupabase(mlResults.top5)`
3. On user interactions: Call `trackEvent('button_click', { button: 'export' })`

---

## Part 7: Environment Setup Instructions

### Step 1: Create Supabase Project
1. Go to https://supabase.com
2. Create new project
3. Note down:
   - Project URL
   - Anon/Public Key
   - Database Password

### Step 2: Run SQL Migrations
1. Go to SQL Editor in Supabase dashboard
2. Copy and paste all table creation SQL from Part 1
3. Run each migration sequentially
4. Verify tables are created

### Step 3: Create Admin User
```sql
-- Insert admin user (run in SQL Editor)
INSERT INTO auth.users (
  email,
  encrypted_password,
  email_confirmed_at,
  created_at,
  updated_at
) VALUES (
  'admin@example.com',
  crypt('your_admin_password', gen_salt('bf')),
  NOW(),
  NOW(),
  NOW()
);

-- Get the user ID from auth.users
-- Then insert into public.users with admin flag
INSERT INTO public.users (id, email, is_admin, created_at)
SELECT id, email, true, created_at
FROM auth.users
WHERE email = 'admin@example.com';
```

### Step 4: Update Configuration
Replace in all files:
- `YOUR_SUPABASE_URL` → Your actual Supabase URL
- `YOUR_SUPABASE_ANON_KEY` → Your actual anon key

### Step 5: Deploy
1. Add `/admin` folder to repository
2. Push to GitHub
3. Verify routes work on GitHub Pages

---

## Part 8: Security Considerations

1. **Row Level Security (RLS)**: All tables have RLS enabled
2. **Admin Check**: Always verify `is_admin` flag server-side
3. **API Key**: Never commit actual Supabase keys to repository (use environment variables in production)
4. **Password Policy**: Enforce strong passwords in Supabase Auth settings
5. **Rate Limiting**: Enable in Supabase dashboard to prevent abuse

---

## Part 9: Testing Checklist

- [ ] Can create new user account
- [ ] Can login as regular user
- [ ] Regular user cannot access `/admin`
- [ ] Admin can login to `/admin`
- [ ] Admin dashboard displays all users
- [ ] Admin can view recommendation history
- [ ] CSV export works correctly
- [ ] Analytics charts display data
- [ ] RLS policies prevent unauthorized access
- [ ] Session tracking works end-to-end

---

## Implementation Priority

1. **Phase 1** (Critical): Database schema + RLS policies
2. **Phase 2** (Critical): Admin login page + authentication
3. **Phase 3** (Critical): Admin dashboard basic UI + users table
4. **Phase 4** (High): Recommendations table + analytics
5. **Phase 5** (Medium): Main app Supabase integration
6. **Phase 6** (Low): Analytics charts + advanced features

---

## Expected File Structure After Implementation

```
/admin/
  ├── index.html          # Admin dashboard
  ├── login.html          # Admin login page
  ├── supabase-client.js  # Supabase helper functions
  └── styles.css          # Admin-specific styles (optional)

/src/
  └── index.html          # Updated with Supabase integration

/.env.example             # Environment variables template
SUPABASE_SETUP_PROMPT.md  # This file
```

---

## Additional Notes

- Keep existing localStorage functionality as fallback
- Gradually migrate users from localStorage to Supabase
- Add loading states and error handling
- Implement proper error logging
- Consider adding email notifications for admin events
- Plan for data export/backup strategies

---

**Once implemented, test thoroughly before deploying to production!**
