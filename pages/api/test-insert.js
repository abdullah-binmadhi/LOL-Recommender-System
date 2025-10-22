import { supabase } from '../../lib/supabaseClient'

export default async function handler(req, res) {
  if (req.method !== 'POST') {
    res.setHeader('Allow', ['POST'])
    return res.status(405).end(`Method ${req.method} Not Allowed`)
  }

  try {
    // Insert a test user
    const testUser = {
      full_name: 'Test User',
      email: 'test@example.com',
      phone: '+1234567890',
      age: 25,
      gender: 'male',
      experience: 'intermediate',
      session_id: `test_session_${Date.now()}`
    }

    const { data, error } = await supabase
      .from('users')
      .insert([testUser])
      .select()

    if (error) throw error

    res.status(200).json({ 
      success: true, 
      message: 'Test user inserted successfully',
      data: data[0]
    })
  } catch (error) {
    console.error('Error inserting test user:', error)
    res.status(500).json({ 
      success: false, 
      error: error.message,
      message: 'Failed to insert test user'
    })
  }
}