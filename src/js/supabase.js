// Supabase client configuration
import { createClient } from '@supabase/supabase-js'

// Initialize Supabase client
const supabaseUrl = process.env.SUPABASE_URL || 'YOUR_SUPABASE_URL'
const supabaseKey = process.env.SUPABASE_ANON_KEY || 'YOUR_SUPABASE_ANON_KEY'

export const supabase = createClient(supabaseUrl, supabaseKey)

// Function to save user registration data
export async function saveUserToSupabase(userData) {
  try {
    const { data, error } = await supabase
      .from('users')
      .insert([
        {
          full_name: userData.fullName,
          email: userData.email,
          phone: userData.phone,
          age: userData.age,
          gender: userData.gender,
          experience: userData.experience,
          registration_date: userData.registrationDate,
          session_id: userData.sessionId
        }
      ])
      .select()

    if (error) {
      console.error('Error saving user to Supabase:', error)
      return { success: false, error }
    }

    console.log('User saved to Supabase:', data)
    return { success: true, data }
  } catch (error) {
    console.error('Error saving user to Supabase:', error)
    return { success: false, error }
  }
}

// Function to save questionnaire results
export async function saveQuestionnaireResultsToSupabase(resultData) {
  try {
    // Parse user answers if they exist
    let parsedAnswers = {}
    if (resultData.userAnswers) {
      try {
        parsedAnswers = JSON.parse(resultData.userAnswers)
      } catch (e) {
        console.error('Error parsing user answers:', e)
      }
    }

    const { data, error } = await supabase
      .from('questionnaire_results')
      .insert([
        {
          session_id: resultData.sessionId,
          recommended_champion: resultData.recommendedChampion,
          winning_algorithm: resultData.winningAlgorithm,
          confidence_score: resultData.confidence,
          random_forest_champion: resultData.randomForestChampion,
          random_forest_confidence: resultData.randomForestConfidence,
          decision_tree_champion: resultData.decisionTreeChampion,
          decision_tree_confidence: resultData.decisionTreeConfidence,
          knn_champion: resultData.knnChampion,
          knn_confidence: resultData.knnConfidence,
          consensus_level: resultData.consensusLevel,
          user_answers: resultData.userAnswers,
          completion_date: resultData.completionDate,
          // Psychological questions mapping
          pressure_response: parsedAnswers[6] || null,
          aesthetic_preference: parsedAnswers[7] || null,
          team_contribution: parsedAnswers[8] || null,
          character_identity: parsedAnswers[9] || null,
          problem_solving: parsedAnswers[10] || null
        }
      ])
      .select()

    if (error) {
      console.error('Error saving questionnaire results to Supabase:', error)
      return { success: false, error }
    }

    console.log('Questionnaire results saved to Supabase:', data)
    return { success: true, data }
  } catch (error) {
    console.error('Error saving questionnaire results to Supabase:', error)
    return { success: false, error }
  }
}

// Function to get all users (for admin panel)
export async function getAllUsers() {
  try {
    const { data, error } = await supabase
      .from('users')
      .select('*')
      .order('registration_date', { ascending: false })

    if (error) {
      console.error('Error fetching users from Supabase:', error)
      return { success: false, error }
    }

    return { success: true, data }
  } catch (error) {
    console.error('Error fetching users from Supabase:', error)
    return { success: false, error }
  }
}

// Function to get questionnaire results for a session
export async function getQuestionnaireResults(sessionId) {
  try {
    const { data, error } = await supabase
      .from('questionnaire_results')
      .select('*')
      .eq('session_id', sessionId)

    if (error) {
      console.error('Error fetching questionnaire results from Supabase:', error)
      return { success: false, error }
    }

    return { success: true, data }
  } catch (error) {
    console.error('Error fetching questionnaire results from Supabase:', error)
    return { success: false, error }
  }
}