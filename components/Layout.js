import Navigation from './Navigation'
import { Analytics } from '@vercel/analytics/next'

export default function Layout({ children }) {
  return (
    <div>
      <Navigation />
      <main>{children}</main>
      <Analytics />
    </div>
  )
}