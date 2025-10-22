import Navigation from './Navigation'

export default function Layout({ children }) {
  return (
    <div>
      <Navigation />
      <main>{children}</main>
    </div>
  )
}