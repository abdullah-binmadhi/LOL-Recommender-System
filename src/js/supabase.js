// Supabase client configuration
import { createClient } from '@supabase/supabase-js';

// Get Supabase credentials from environment variables
const supabaseUrl = process.env.SUPABASE_URL || 'https://brshmbbohqcdseyirqsn.supabase.co';
const supabaseAnonKey = process.env.SUPABASE_ANON_KEY || 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImJyc2htYmJvaHFjZHNleWlycXNuIiwicm9sZSI6ImFub24iLCJpYXQiOjE3NjEwOTYyNzAsImV4cCI6MjA3NjY3MjI3MH0.Yi-aWEuvArux0qRrEgnnzIljrsh5NnVBOxHlpBqbn2E';

// Create Supabase client
export const supabase = createClient(supabaseUrl, supabaseAnonKey);

// Function to insert user data into Supabase
export async function insertUserData(userData) {
  try {
    const { data, error } = await supabase
      .from('champion_recommender_users')
      .insert([
        {
          full_name: userData.full_name,
          email: userData.email,
          phone: userData.phone || '',
          age: userData.age,
          gender: userData.gender || '',
          lol_experience: userData.lol_experience || '',
          session_id: userData.session_id,
          recommended_champion: userData.recommended_champion || '',
          winning_algorithm: userData.winning_algorithm || '',
          confidence_score: userData.confidence_score || null,
          random_forest_champion: userData.random_forest_champion || '',
          random_forest_confidence: userData.random_forest_confidence || null,
          decision_tree_champion: userData.decision_tree_champion || '',
          decision_tree_confidence: userData.decision_tree_confidence || null,
          knn_champion: userData.knn_champion || '',
          knn_confidence: userData.knn_confidence || null,
          consensus_level: userData.consensus_level || null,
          user_answers: userData.user_answers || {},
          completion_timestamp: userData.completion_timestamp || null,
          pressure_response: userData.pressure_response || '',
          aesthetic_preference: userData.aesthetic_preference || '',
          team_contribution: userData.team_contribution || '',
          character_identity: userData.character_identity || '',
          problem_solving: userData.problem_solving || ''
        }
      ]);

    if (error) {
      console.error('Error inserting user data:', error);
      return { success: false, error };
    }

    console.log('User data inserted successfully:', data);
    return { success: true, data };
  } catch (error) {
    console.error('Exception occurred while inserting user data:', error);
    return { success: false, error };
  }
}

// Function to get user count from Supabase
export async function getUserCount() {
  try {
    const { count, error } = await supabase
      .from('champion_recommender_users')
      .select('*', { count: 'exact' });

    if (error) {
      console.error('Error getting user count:', error);
      return { success: false, error };
    }

    return { success: true, count };
  } catch (error) {
    console.error('Exception occurred while getting user count:', error);
    return { success: false, error };
  }
}

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