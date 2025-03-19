import { NetworkClient } from './networkclient.ts'
import { env } from 'node:process';

console.log('Installing persisted queries...')

// @ts-ignore
const documents = JSON.parse(Deno.readTextFileSync(import.meta.dirname + '/persisted-documents.json'))

// @ts-ignore
const queries = []

for (const sha256 in documents) {
  queries.push({
    sha256,
    query: documents[sha256],
  })
}

console.log('Found ' + queries.length + ' queries.')

const client = new NetworkClient()
const graphqlUrl = env.GRAPHQL_URL || 'http://localhost:8000/graphql'
const username = env.GRAPHQL_USERNAME || 'admin'
const password = env.GRAPHQL_PASSWORD || 'password'

// First login to get a token
client.execute({
  identifier: username,
  password: password,
}, {
  url: graphqlUrl,
  query: 'mutation Login($identifier: String!, $password: String!) { security { login { password(identifier: $identifier, password: $password) { token { token } } } } }',
})
.then(async (response) => {
  // Extract token from response
  // @ts-ignore
  const token = response.security.login.password.token.token
  console.log('Successfully logged in')

  // Use token to add persisted queries
  // @ts-ignore
  return await client.execute({
    application: 'bosca-administration',
    // @ts-ignore
    queries: queries,
  }, {
    url: graphqlUrl,
    query: 'mutation AddPersistedQueries($application: String!, $queries: [PersistedQueryInput!]!) { persistedQueries { addAll(application: $application, queries: $queries) } }',
    token: token,
  })
})
.catch((e) => {
  console.error(e)
  // @ts-ignore
  Deno.exit(1);
})
.finally(() => {
  console.log('Finished installing.')
  // @ts-ignore
  Deno.exit();
})
console.log('Waiting for install to finish...')
