import { useState } from 'react'
import Layout from '../components/Layout'

export default function TestSupabase() {
  const [testResult, setTestResult] = useState(null)
  const [loading, setLoading] = useState(false)

  const testConnection = async () => {
    setLoading(true)
    setTestResult(null)

    try {
      const response = await fetch('/api/test-connection')
      const result = await response.json()
      setTestResult({ type: 'connection', data: result })
    } catch (error) {
      setTestResult({ type: 'error', message: error.message })
    } finally {
      setLoading(false)
    }
  }

  const testInsert = async () => {
    setLoading(true)
    setTestResult(null)

    try {
      const response = await fetch('/api/test-insert', {
        method: 'POST',
      })
      const result = await response.json()
      setTestResult({ type: 'insert', data: result })
    } catch (error) {
      setTestResult({ type: 'error', message: error.message })
    } finally {
      setLoading(false)
    }
  }

  return (
    <Layout>
      <div style={{ padding: '20px', fontFamily: 'sans-serif', maxWidth: '800px', margin: '0 auto' }}>
        <h1>Supabase Connection Test</h1>
        
        <div style={{ marginBottom: '20px' }}>
          <button 
            onClick={testConnection}
            disabled={loading}
            style={{ 
              padding: '10px 20px', 
              backgroundColor: '#0070f3', 
              color: 'white', 
              border: 'none', 
              borderRadius: '5px',
              cursor: loading ? 'not-allowed' : 'pointer',
              marginRight: '10px'
            }}
          >
            {loading ? 'Testing...' : 'Test Connection'}
          </button>
          
          <button 
            onClick={testInsert}
            disabled={loading}
            style={{ 
              padding: '10px 20px', 
              backgroundColor: '#4caf50', 
              color: 'white', 
              border: 'none', 
              borderRadius: '5px',
              cursor: loading ? 'not-allowed' : 'pointer'
            }}
          >
            {loading ? 'Inserting...' : 'Test Insert'}
          </button>
        </div>

        {testResult && (
          <div style={{ 
            padding: '15px', 
            borderRadius: '5px',
            backgroundColor: testResult.type === 'error' ? '#ffebee' : '#e8f5e9',
            color: testResult.type === 'error' ? '#c62828' : '#2e7d32'
          }}>
            <h3>Test Result:</h3>
            <pre style={{ 
              backgroundColor: 'rgba(0,0,0,0.05)', 
              padding: '10px', 
              borderRadius: '3px',
              overflowX: 'auto'
            }}>
              {JSON.stringify(testResult, null, 2)}
            </pre>
          </div>
        )}

        <div style={{ 
          marginTop: '30px',
          padding: '15px', 
          backgroundColor: '#f0f0f0', 
          borderRadius: '5px'
        }}>
          <h3>Next Steps:</h3>
          <ol>
            <li>Click "Test Connection" to verify your Supabase credentials</li>
            <li>If connection is successful, click "Test Insert" to add a test user</li>
            <li>Check the dashboard at <a href="/">the home page</a> to see the test user</li>
            <li>If you get table errors, go to <a href="/init">the initialization page</a> for setup instructions</li>
          </ol>
        </div>
      </div>
    </Layout>
  )
}