import { supabase } from '../../lib/supabaseClient'

export default async function handler(req, res) {
  if (req.method === 'GET') {
    const { session_id } = req.query
    
    try {
      let query = supabase.from('questionnaire_results').select('*')
      
      if (session_id) {
        query = query.eq('session_id', session_id)
      }
      
      const { data, error } = await query.order('completion_date', { ascending: false })

      if (error) throw error

      res.status(200).json({ success: true, data })
    } catch (error) {
      console.error('Error fetching questionnaire results:', error)
      res.status(500).json({ success: false, error: error.message })
    }
  } else if (req.method === 'POST') {
    // Create new questionnaire results
    try {
      const { data, error } = await supabase
        .from('questionnaire_results')
        .insert([req.body])
        .select()

      if (error) throw error

      res.status(200).json({ success: true, data })
    } catch (error) {
      console.error('Error creating questionnaire results:', error)
      res.status(500).json({ success: false, error: error.message })
    }
  } else {
    res.setHeader('Allow', ['GET', 'POST'])
    res.status(405).end(`Method ${req.method} Not Allowed`)
  }
}