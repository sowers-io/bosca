// deno-lint-ignore-file require-await
export default defineEventHandler(async (e) => {
  const cookies = parseCookies(e)
  if (cookies._bat) return
  const path = e.path.split(' ')[0]
  if (
    path !== '/login' &&
    path !== '/forgotpassword' &&
    path !== '/signup/verify' &&
    path !== '/signup' &&
    path !== '/terms' &&
    path !== '/privacy' &&
    path !== '/fix-tprotocol-service-worker.js' &&
    !path.startsWith('/_') &&
    !path.startsWith('/api/') &&
    !path.startsWith('/graphql')
  ) {
    console.error('Unauthorized, redirecting to login. :: `' + path + '`')
    return sendRedirect(e, '/login')
  }
})
