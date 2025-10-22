import { useEffect, useState } from 'react'
import { supabase } from '../lib/supabaseClient'
import Layout from '../components/Layout'

export default function Home() {
  const [users, setUsers] = useState([])
  const [questionnaireResults, setQuestionnaireResults] = useState([])
  const [loading, setLoading] = useState({ users: true, results: true })
  const [connectionStatus, setConnectionStatus] = useState('Checking...')

  useEffect(() => {
    checkConnection()
    fetchUsers()
    fetchQuestionnaireResults()
  }, [])

  async function checkConnection() {
    try {
      const response = await fetch('/api/test-connection')
      const result = await response.json()
      
      if (result.connected) {
        setConnectionStatus(`Connected - ${result.userCount} users`)
      } else {
        setConnectionStatus('Connection failed')
      }
    } catch (error) {
      console.error('Connection check failed:', error)
      setConnectionStatus('Connection failed')
    }
  }

  async function fetchUsers() {
    try {
      setLoading(prev => ({ ...prev, users: true }))
      const response = await fetch('/api/users')
      const result = await response.json()
      
      if (result.success) {
        setUsers(result.data || [])
      } else {
        console.error('Error fetching users:', result.error)
      }
    } catch (error) {
      console.error('Error fetching users:', error)
    } finally {
      setLoading(prev => ({ ...prev, users: false }))
    }
  }

  async function fetchQuestionnaireResults() {
    try {
      setLoading(prev => ({ ...prev, results: true }))
      const response = await fetch('/api/questionnaire-results')
      const result = await response.json()
      
      if (result.success) {
        setQuestionnaireResults(result.data || [])
      } else {
        console.error('Error fetching questionnaire results:', result.error)
      }
    } catch (error) {
      console.error('Error fetching questionnaire results:', error)
    } finally {
      setLoading(prev => ({ ...prev, results: false }))
    }
  }

  return (
    <Layout>
      <div style={{ padding: '20px', maxWidth: '1200px', margin: '0 auto' }}>
        <h1>League of Legends Champion Recommender - Supabase Dashboard</h1>
      
      <div style={{ marginBottom: '20px', padding: '10px', backgroundColor: '#f0f0f0', borderRadius: '5px' }}>
        <strong>Connection Status:</strong> {connectionStatus}
      </div>

      <div style={{ display: 'flex', gap: '20px', flexWrap: 'wrap' }}>
        <div style={{ flex: 1, minWidth: '300px' }}>
          <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
            <h2>Users ({users.length})</h2>
            <button onClick={fetchUsers} disabled={loading.users}>
              Refresh
            </button>
          </div>
          
          {loading.users ? (
            <p>Loading users...</p>
          ) : (
            <div>
              {users.length > 0 ? (
                <ul style={{ maxHeight: '300px', overflowY: 'auto' }}>
                  {users.map((user) => (
                    <li key={user.id} style={{ marginBottom: '10px', padding: '10px', backgroundColor: '#f9f9f9', borderRadius: '5px' }}>
                      <strong>{user.full_name}</strong> ({user.email})<br />
                      <small>Session: {user.session_id} | Registered: {new Date(user.registration_date).toLocaleString()}</small>
                    </li>
                  ))}
                </ul>
              ) : (
                <p>No users found in the database.</p>
              )}
            </div>
          )}
        </div>

        <div style={{ flex: 1, minWidth: '300px' }}>
          <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
            <h2>Questionnaire Results ({questionnaireResults.length})</h2>
            <button onClick={fetchQuestionnaireResults} disabled={loading.results}>
              Refresh
            </button>
          </div>
          
          {loading.results ? (
            <p>Loading results...</p>
          ) : (
            <div>
              {questionnaireResults.length > 0 ? (
                <ul style={{ maxHeight: '300px', overflowY: 'auto' }}>
                  {questionnaireResults.map((result) => (
                    <li key={result.id} style={{ marginBottom: '10px', padding: '10px', backgroundColor: '#f9f9f9', borderRadius: '5px' }}>
                      <strong>{result.recommended_champion}</strong> (Confidence: {result.confidence_score})<br />
                      <small>Session: {result.session_id} | Completed: {new Date(result.completion_date).toLocaleString()}</small>
                    </li>
                  ))}
                </ul>
              ) : (
                <p>No questionnaire results found.</p>
              )}
            </div>
          )}
        </div>
      </div>
    </div>
  </Layout>
)}