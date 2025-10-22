import { supabase } from '../../lib/supabaseClient'

export default async function handler(req, res) {
  if (req.method === 'GET') {
    // Get all users
    try {
      const { data, error } = await supabase
        .from('users')
        .select('*')
        .order('registration_date', { ascending: false })

      if (error) throw error

      res.status(200).json({ success: true, data })
    } catch (error) {
      console.error('Error fetching users:', error)
      res.status(500).json({ success: false, error: error.message })
    }
  } else if (req.method === 'POST') {
    // Create a new user
    try {
      const { data, error } = await supabase
        .from('users')
        .insert([req.body])
        .select()

      if (error) throw error

      res.status(200).json({ success: true, data })
    } catch (error) {
      console.error('Error creating user:', error)
      res.status(500).json({ success: false, error: error.message })
    }
  } else {
    res.setHeader('Allow', ['GET', 'POST'])
    res.status(405).end(`Method ${req.method} Not Allowed`)
  }
}