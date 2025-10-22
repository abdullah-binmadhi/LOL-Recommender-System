# Next.js Supabase Integration

This directory contains a Next.js application that connects to your Supabase project for the League of Legends Champion Recommender.

## Setup

The application is already configured with your Supabase credentials:

- **Supabase URL**: `https://brshmbbohqcdseyirqsn.supabase.co`
- **Supabase ANON KEY**: `eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImJyc2htYmJvaHFjZHNleWlycXNuIiwicm9sZSI6ImFub24iLCJpYXQiOjE3NjEwOTYyNzAsImV4cCI6MjA3NjY3MjI3MH0.Yi-aWEuvArux0qRrEgnnzIljrsh5NnVBOxHlpBqbn2E`

## Running the Application

1. Make sure you're in the project directory:
   ```
   cd /Users/abdullahbinmadhi/Desktop/LOL-Recommender-System
   ```

2. Install dependencies (if not already done):
   ```
   npm install
   ```

3. Start the development server:
   ```
   npm run dev
   ```

4. Open your browser to http://localhost:3000

## Features

- **Dashboard** (`/`): View users and questionnaire results
- **Add User** (`/add-user`): Add new users to the database
- **Initialize Database** (`/init`): Check if required tables exist
- **Test Supabase** (`/test`): Test connection and data insertion

## Required Database Tables

The application expects two tables in your Supabase database:

1. `users` - Stores user registration information
2. `questionnaire_results` - Stores questionnaire responses and recommendations

If these tables don't exist, you can create them using the SQL commands in `supabase/seed.sql`.

## Environment Variables

The Supabase credentials are stored in `.env.local`:

```
NEXT_PUBLIC_SUPABASE_URL=https://brshmbbohqcdseyirqsn.supabase.co
NEXT_PUBLIC_SUPABASE_ANON_KEY=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImJyc2htYmJvaHFjZHNleWlycXNuIiwicm9sZSI6ImFub24iLCJpYXQiOjE3NjEwOTYyNzAsImV4cCI6MjA3NjY3MjI3MH0.Yi-aWEuvArux0qRrEgnnzIljrsh5NnVBOxHlpBqbn2E
```

## API Routes

The application includes several API routes for interacting with Supabase:

- `/api/test-connection` - Test Supabase connection
- `/api/users` - Get all users or add a new user
- `/api/questionnaire-results` - Get questionnaire results or add new results
- `/api/init-db` - Check if required tables exist
- `/api/test-insert` - Insert a test user record

## Next Steps

1. Visit http://localhost:3000 to see the dashboard
2. Use the "Test Supabase" page to verify your connection
3. If needed, create the required tables using the instructions on the "Initialize DB" page
4. Add test data using the "Add User" form