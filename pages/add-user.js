import { useState } from 'react'
import { supabase } from '../lib/supabaseClient'
import { useRouter } from 'next/router'
import Layout from '../components/Layout'

export default function AddUser() {
  const router = useRouter()
  const [formData, setFormData] = useState({
    full_name: '',
    email: '',
    phone: '',
    age: '',
    gender: '',
    experience: '',
    session_id: ''
  })
  const [loading, setLoading] = useState(false)
  const [message, setMessage] = useState('')

  const handleChange = (e) => {
    setFormData({
      ...formData,
      [e.target.name]: e.target.value
    })
  }

  const handleSubmit = async (e) => {
    e.preventDefault()
    setLoading(true)
    setMessage('')

    try {
      // Generate a session ID if not provided
      const sessionId = formData.session_id || `session_${Date.now()}_${Math.floor(Math.random() * 1000)}`

      const userData = {
        full_name: formData.full_name,
        email: formData.email,
        phone: formData.phone || '',
        age: parseInt(formData.age) || null,
        gender: formData.gender || '',
        experience: formData.experience || '',
        registration_date: new Date().toISOString(),
        session_id: sessionId
      }

      const response = await fetch('/api/users', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(userData),
      })

      const result = await response.json()

      if (result.success) {
        setMessage('User added successfully!')
        // Reset form
        setFormData({
          full_name: '',
          email: '',
          phone: '',
          age: '',
          gender: '',
          experience: '',
          session_id: ''
        })
        // Redirect to home after 2 seconds
        setTimeout(() => {
          router.push('/')
        }, 2000)
      } else {
        setMessage(`Error: ${result.error}`)
      }
    } catch (error) {
      console.error('Error adding user:', error)
      setMessage(`Error: ${error.message}`)
    } finally {
      setLoading(false)
    }
  }

  return (
    <Layout>
      <div style={{ padding: '20px', maxWidth: '600px', margin: '0 auto' }}>
        <h1>Add New User</h1>
      
      {message && (
        <div style={{ 
          padding: '10px', 
          marginBottom: '20px', 
          borderRadius: '5px',
          backgroundColor: message.includes('Error') ? '#ffebee' : '#e8f5e9',
          color: message.includes('Error') ? '#c62828' : '#2e7d32'
        }}>
          {message}
        </div>
      )}

      <form onSubmit={handleSubmit}>
        <div style={{ marginBottom: '15px' }}>
          <label htmlFor="full_name">Full Name *</label>
          <input
            type="text"
            id="full_name"
            name="full_name"
            value={formData.full_name}
            onChange={handleChange}
            required
            style={{ width: '100%', padding: '8px', marginTop: '5px' }}
          />
        </div>

        <div style={{ marginBottom: '15px' }}>
          <label htmlFor="email">Email *</label>
          <input
            type="email"
            id="email"
            name="email"
            value={formData.email}
            onChange={handleChange}
            required
            style={{ width: '100%', padding: '8px', marginTop: '5px' }}
          />
        </div>

        <div style={{ marginBottom: '15px' }}>
          <label htmlFor="phone">Phone</label>
          <input
            type="tel"
            id="phone"
            name="phone"
            value={formData.phone}
            onChange={handleChange}
            style={{ width: '100%', padding: '8px', marginTop: '5px' }}
          />
        </div>

        <div style={{ marginBottom: '15px' }}>
          <label htmlFor="age">Age</label>
          <input
            type="number"
            id="age"
            name="age"
            value={formData.age}
            onChange={handleChange}
            min="13"
            max="100"
            style={{ width: '100%', padding: '8px', marginTop: '5px' }}
          />
        </div>

        <div style={{ marginBottom: '15px' }}>
          <label htmlFor="gender">Gender</label>
          <select
            id="gender"
            name="gender"
            value={formData.gender}
            onChange={handleChange}
            style={{ width: '100%', padding: '8px', marginTop: '5px' }}
          >
            <option value="">Select Gender</option>
            <option value="male">Male</option>
            <option value="female">Female</option>
            <option value="non-binary">Non-binary</option>
            <option value="prefer-not-to-say">Prefer not to say</option>
          </select>
        </div>

        <div style={{ marginBottom: '15px' }}>
          <label htmlFor="experience">League of Legends Experience</label>
          <select
            id="experience"
            name="experience"
            value={formData.experience}
            onChange={handleChange}
            style={{ width: '100%', padding: '8px', marginTop: '5px' }}
          >
            <option value="">Select Experience Level</option>
            <option value="beginner">Beginner (0-1 years)</option>
            <option value="intermediate">Intermediate (1-3 years)</option>
            <option value="advanced">Advanced (3+ years)</option>
            <option value="veteran">Veteran (5+ years)</option>
          </select>
        </div>

        <div style={{ marginBottom: '15px' }}>
          <label htmlFor="session_id">Session ID (optional)</label>
          <input
            type="text"
            id="session_id"
            name="session_id"
            value={formData.session_id}
            onChange={handleChange}
            style={{ width: '100%', padding: '8px', marginTop: '5px' }}
          />
        </div>

        <button 
          type="submit" 
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
          {loading ? 'Adding User...' : 'Add User'}
        </button>
        
        <button 
          type="button" 
          onClick={() => router.push('/')}
          style={{ 
            padding: '10px 20px', 
            backgroundColor: '#666', 
            color: 'white', 
            border: 'none', 
            borderRadius: '5px',
            cursor: 'pointer',
            marginLeft: '10px'
          }}
        >
          Back to Dashboard
        </button>
      </form>
    </div>
  </Layout>
)}