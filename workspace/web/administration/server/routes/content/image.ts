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
  if (id) {
    const metadata = await client.metadata.get(id)
    return fetch(metadata.content.urls.download.url)
  } else {
    const slug = url.search.split('?slug=')[1]
    const item = await client.get(slug)
    if (item?.__typename != 'Metadata') throw new Error('Not found')
    return fetch(item.content.urls.download.url)
  }
})
