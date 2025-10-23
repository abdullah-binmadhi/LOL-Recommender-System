import Link from 'next/link'
import { useRouter } from 'next/router'

export default function Navigation() {
  const router = useRouter()
  
  return (
    <nav style={{ 
      padding: '10px 20px', 
      backgroundColor: '#333', 
      marginBottom: '20px' 
    }}>
      <div style={{ 
        maxWidth: '1200px', 
        margin: '0 auto', 
        display: 'flex', 
        justifyContent: 'space-between',
        alignItems: 'center'
      }}>
        <div>
          <Link href="/" style={{ 
            color: 'white', 
            textDecoration: 'none', 
            fontSize: '1.2em',
            fontWeight: 'bold'
          }}>
            LoL Champion Recommender
          </Link>
        </div>
        
        <div>
          <Link href="/" style={{ 
            color: router.pathname === '/' ? '#1e88e5' : 'white', 
            textDecoration: 'none', 
            marginRight: '20px',
            padding: '5px 10px',
            borderRadius: '3px'
          }}>
            Dashboard
          </Link>
          
          <Link href="/analytics" style={{ 
            color: router.pathname === '/analytics' ? '#1e88e5' : 'white', 
            textDecoration: 'none', 
            padding: '5px 10px',
            borderRadius: '3px',
            marginRight: '20px'
          }}>
            Analytics
          </Link>
          
          <Link href="/add-user" style={{ 
            color: router.pathname === '/add-user' ? '#1e88e5' : 'white', 
            textDecoration: 'none', 
            padding: '5px 10px',
            borderRadius: '3px',
            marginRight: '20px'
          }}>
            Add User
          </Link>
          
          <Link href="/init" style={{ 
            color: router.pathname === '/init' ? '#1e88e5' : 'white', 
            textDecoration: 'none', 
            padding: '5px 10px',
            borderRadius: '3px',
            marginRight: '20px'
          }}>
            Initialize DB
          </Link>
          
          <Link href="/test" style={{ 
            color: router.pathname === '/test' ? '#1e88e5' : 'white', 
            textDecoration: 'none', 
            padding: '5px 10px',
            borderRadius: '3px'
          }}>
            Test Supabase
          </Link>
        </div>
      </div>
    </nav>
  )
}