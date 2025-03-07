import { BoscaClient } from '~/lib/bosca/client'
import { NetworkClient } from '~/lib/bosca/networkclient'

export default defineEventHandler(async (e) => {
  const cookies = parseCookies(e)
  if (!cookies._bat) {
    return sendRedirect(e, '/login')
  }
  const url = getRequestURL(e)
  const network = new NetworkClient()
  network.token = cookies._bat
  const client = new BoscaClient(network)
  const id = url.search.split('?id=')[1]
  const metadata = await client.metadata.get(id)
  return fetch(metadata.content.urls.download.url)
})
