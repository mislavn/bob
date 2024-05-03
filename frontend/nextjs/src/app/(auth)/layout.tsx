import { ReactNode } from 'react'

export default function AuthLayout({ children }: { children: ReactNode }) {
  return (
    <>
      <header className="flex h-20 items-center border-b border-solid border-slate-200 px-4">
        <a href="/">BOB</a>
      </header>
      <div className="flex h-[calc(100vh-theme(space.20))] items-center justify-center">
        {children}
      </div>
    </>
  )
}
