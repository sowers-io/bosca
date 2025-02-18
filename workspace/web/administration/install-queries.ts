import type { PersistedQueryInput } from './lib/graphql/graphql.ts'
import { AddPersistedQueriesDocument } from './lib/graphql/graphql.ts'
import { NetworkClient } from './lib/bosca/networkclient.ts'

const documents = JSON.parse(
  Deno.readTextFileSync('./lib/graphql/persisted-documents.json'),
)

const queries = []

for (const sha256 in documents) {
  queries.push({
    sha256,
    query: documents[sha256],
  } as PersistedQueryInput)
}

const client = new NetworkClient()

client.execute(AddPersistedQueriesDocument, {
  application: 'bosca-administration',
  queries: queries,
}, {
  post: true,
  url: 'http://localhost:8000/graphql',
  query:
    'mutation AddPersistedQueries($application: String!, $queries: [PersistedQueryInput!]!) { persistedQueries { addAll(application: $application, queries: $queries) } }',
  username: 'admin',
  password: 'password',
})
