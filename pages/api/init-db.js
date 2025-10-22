import { supabase } from '../../lib/supabaseClient'

export default async function handler(req, res) {
  if (req.method !== 'POST') {
    res.setHeader('Allow', ['POST'])
    return res.status(405).end(`Method ${req.method} Not Allowed`)
  }

  try {
    // Check if tables exist by querying them
    const { error: usersError } = await supabase.from('users').select('id').limit(1)
    const { error: resultsError } = await supabase.from('questionnaire_results').select('id').limit(1)

    // If tables don't exist, we'll get an error
    if (usersError && usersError.message.includes('does not exist')) {
      return res.status(200).json({ 
        success: false, 
        message: 'Tables do not exist. Please create them in Supabase dashboard.',
        action: 'Create tables using the SQL commands in supabase/seed.sql'
      })
    }

    res.status(200).json({ 
      success: true, 
      message: 'Database is ready',
      usersTableExists: !usersError || !usersError.message.includes('does not exist'),
      resultsTableExists: !resultsError || !resultsError.message.includes('does not exist')
    })
  } catch (error) {
    console.error('Error initializing database:', error)
    res.status(500).json({ success: false, error: error.message })
  }
}