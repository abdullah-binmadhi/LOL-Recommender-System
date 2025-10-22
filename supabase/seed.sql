-- Create users table
create table if not exists users (
  id uuid default uuid_generate_v4() primary key,
  full_name text,
  email text,
  phone text,
  age integer,
  gender text,
  experience text,
  registration_date timestamp with time zone default now(),
  session_id text unique
);

-- Create questionnaire_results table
create table if not exists questionnaire_results (
  id uuid default uuid_generate_v4() primary key,
  session_id text references users(session_id),
  recommended_champion text,
  winning_algorithm text,
  confidence_score numeric,
  random_forest_champion text,
  random_forest_confidence numeric,
  decision_tree_champion text,
  decision_tree_confidence numeric,
  knn_champion text,
  knn_confidence numeric,
  consensus_level integer,
  user_answers jsonb,
  completion_date timestamp with time zone default now(),
  pressure_response text,
  aesthetic_preference text,
  team_contribution text,
  character_identity text,
  problem_solving text
);

-- Add indexes for better performance
create index if not exists idx_users_session_id on users(session_id);
create index if not exists idx_questionnaire_results_session_id on questionnaire_results(session_id);
create index if not exists idx_questionnaire_results_completion_date on questionnaire_results(completion_date);