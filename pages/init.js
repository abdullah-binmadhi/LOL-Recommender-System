import { useState } from 'react'
import Layout from '../components/Layout'
import { useRouter } from 'next/router'

export default function InitDB() {
  const router = useRouter()
  const [status, setStatus] = useState('')
  const [loading, setLoading] = useState(false)

  const handleInit = async () => {
    setLoading(true)
    setStatus('Initializing database...')

    try {
      const response = await fetch('/api/init-db', {
        method: 'POST',
      })

      const result = await response.json()
      
      if (result.success) {
        setStatus(`Success: ${result.message}`)
        // Redirect to home after 3 seconds
        setTimeout(() => {
          router.push('/')
        }, 3000)
      } else {
        setStatus(`Action required: ${result.message}. ${result.action}`)
      }
    } catch (error) {
      console.error('Error:', error)
      setStatus(`Error: ${error.message}`)
    } finally {
      setLoading(false)
    }
  }

  return (
    <Layout>
      <div style={{ padding: '20px', fontFamily: 'sans-serif', maxWidth: '600px', margin: '0 auto' }}>
        <h1>Initialize Database</h1>
        
        <p>
          This page will check if the required tables exist in your Supabase database.
        </p>
        
        <div style={{ 
          padding: '15px', 
          backgroundColor: '#f0f0f0', 
          borderRadius: '5px',
          marginBottom: '20px'
        }}>
          <h3>Required Tables:</h3>
          <ul>
            <li><strong>users</strong> - Stores user registration information</li>
            <li><strong>questionnaire_results</strong> - Stores questionnaire responses and recommendations</li>
          </ul>
        </div>
        
        {status && (
          <div style={{ 
            padding: '10px', 
            marginBottom: '20px', 
            borderRadius: '5px',
            backgroundColor: status.includes('Error') || status.includes('Action required') ? '#ffebee' : '#e8f5e9',
            color: status.includes('Error') || status.includes('Action required') ? '#c62828' : '#2e7d32'
          }}>
            {status}
          </div>
        )}
        
        <button 
          onClick={handleInit}
          disabled={loading}
          style={{ 
            padding: '10px 20px', 
            backgroundColor: '#0070f3', 
            color: 'white', 
            border: 'none', 
            borderRadius: '5px',
            cursor: loading ? 'not-allowed' : 'pointer'
          }}
        >
          {loading ? 'Initializing...' : 'Check Database'}
        </button>
        
        <div style={{ marginTop: '20px' }}>
          <h3>If tables need to be created:</h3>
          <ol>
            <li>Go to your Supabase dashboard at <a href="https://app.supabase.com" target="_blank" rel="noopener noreferrer">app.supabase.com</a></li>
            <li>Navigate to the SQL Editor</li>
            <li>Run the SQL commands from <code>supabase/seed.sql</code>:</li>
          </ol>
          <pre style={{ 
            backgroundColor: '#f5f5f5', 
            padding: '15px', 
            borderRadius: '5px',
            overflowX: 'auto'
          }}>
{`-- Create users table
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
);`}
          </pre>
        </div>
      </div>
    </Layout>
  )
}