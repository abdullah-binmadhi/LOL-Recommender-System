import { supabase } from '../../lib/supabaseClient'

export default async function handler(req, res) {
  try {
    // Test connection by getting user count
    const { count, error } = await supabase
      .from('users')
      .select('*', { count: 'exact', head: true })

    if (error) {
      console.error('Supabase connection error:', error)
      return res.status(500).json({ 
        success: false, 
        error: error.message,
        connected: false
      })
    }

    res.status(200).json({ 
      success: true, 
      connected: true,
      userCount: count || 0
    })
  } catch (error) {
    console.error('Unexpected error:', error)
    res.status(500).json({ 
      success: false, 
      error: error.message,
      connected: false
    })
  }
}