// deno-lint-ignore-file require-await
import { NetworkClient } from '~/lib/bosca/networkclient'
import { ValidatePrincipalDocument } from '~/lib/graphql/graphql'

export default defineEventHandler(async (e) => {
  const cookies = parseCookies(e)
  if (cookies._bat) {
    try {
      const client = new NetworkClient()
      const result = await client.execute(ValidatePrincipalDocument, {}, {
        token: cookies._bat,
      })
      if (
        result?.security?.principal?.id &&
        result?.security?.principal?.id !==
          '00000000-0000-0000-0000-000000000000'
      ) {
        // TODO: maybe only let certain groups access this?
        return
      }
    } catch (e) {
      console.error(e)
    }
  }
  const path = e.path.split(' ')[0].split('?')[0]
  if (
    path !== '/login' &&
    path !== '/forgotpassword' &&
    path !== '/signup/verify' &&
    path !== '/signup' &&
    path !== '/terms' &&
    path !== '/privacy' &&
    path !== '/content/image' &&
    path !== '/health' &&
    path !== '/fix-tprotocol-service-worker.js' &&
    !path.startsWith('/_') &&
    !path.startsWith('/api/') &&
    !path.startsWith('/graphql')
  ) {
    console.error('Unauthorized, redirecting to login. :: `' + path + '`')
    return sendRedirect(e, '/login')
  }
})
