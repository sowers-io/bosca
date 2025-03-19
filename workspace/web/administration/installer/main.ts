import { NetworkClient } from './networkclient.ts'
import { env } from 'node:process';

console.log('Installing persisted queries...')

// @ts-ignore
const documents = JSON.parse(Deno.readTextFileSync(import.meta.dirname + '/persisted-documents.json'))

const queries = []

for (const sha256 in documents) {
  queries.push({
    sha256,
    query: documents[sha256],
  })
}

console.log('Found ' + queries.length + ' queries.')

const client = new NetworkClient()
client.execute({
  application: 'bosca-administration',
  queries: queries,
}, {
  url: env.GRAPHQL_URL || 'http://localhost:8000/graphql',
  query: 'mutation AddPersistedQueries($application: String!, $queries: [PersistedQueryInput!]!) { persistedQueries { addAll(application: $application, queries: $queries) } }',
  username: env.GRAPHQL_USERNAME || 'admin',
  password: env.GRAPHQL_PASSWORD || 'password',
})
.catch(console.error)
.finally(() => {
  console.log('Finished installing.')
  // @ts-ignore
  Deno.exit();
})
console.log('Waiting for install to finish...')
