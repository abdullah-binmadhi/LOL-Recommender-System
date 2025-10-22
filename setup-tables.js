// Script to set up Supabase tables
const { createClient } = require('@supabase/supabase-js')

// Get credentials from environment
const supabaseUrl = process.env.NEXT_PUBLIC_SUPABASE_URL
const supabaseAnonKey = process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY

if (!supabaseUrl || !supabaseAnonKey) {
  console.error('Missing Supabase credentials. Please check your environment variables.')
  process.exit(1)
}

const supabase = createClient(supabaseUrl, supabaseAnonKey)

async function setupTables() {
  console.log('Setting up Supabase tables...')
  
  // Create users table
  const { error: usersError } = await supabase.rpc('create_users_table')
  
  if (usersError) {
    console.log('Users table creation result:', usersError.message)
  } else {
    console.log('Users table created successfully')
  }
  
  // Create questionnaire_results table
  const { error: resultsError } = await supabase.rpc('create_questionnaire_results_table')
  
  if (resultsError) {
    console.log('Questionnaire results table creation result:', resultsError.message)
  } else {
    console.log('Questionnaire results table created successfully')
  }
}

setupTables()