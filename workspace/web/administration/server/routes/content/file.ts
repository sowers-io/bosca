import { BoscaClient } from '~/lib/bosca/client'
import { NetworkClient } from '~/lib/bosca/networkclient'

export default defineEventHandler(async (e) => {
  const cookies = parseCookies(e)
  const url = getRequestURL(e)
  const network = new NetworkClient()
  network.token = cookies._bat
  const client = new BoscaClient(network)
  const id = url.search.split('?id=')[1].split('&')[0].split('.')[0]
  const metadata = await client.metadata.get(id)
  const response = await fetch(metadata.content.urls.download.url)
  return response
})
