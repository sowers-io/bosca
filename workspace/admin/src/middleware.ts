import { NextResponse } from 'next/server'
import type { NextRequest } from 'next/server'
import { cookies } from 'next/headers'

export function middleware(request: NextRequest) {
  if (!cookies().has('_bat')) {
    const url = new URL(request.url)
    if (url.pathname !== '/user/login') {
      return NextResponse.redirect(new URL('/user/login', request.url))
    }
  }
  return NextResponse.next()
}

export const config = {
  matcher: [
    '/((?!api|_next/static|_next/image|user/login|favicon.ico).*)',
  ],
}