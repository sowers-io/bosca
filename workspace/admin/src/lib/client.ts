import { HttpLink, InMemoryCache, ApolloClient, DefaultOptions } from '@apollo/client'
import { registerApolloClient } from '@apollo/experimental-nextjs-app-support'
import { cookies } from 'next/headers'
import { setContext } from '@apollo/client/link/context'

export const { getClient } = registerApolloClient(() => {
  const defaultOptions: DefaultOptions = {
    watchQuery: {
      fetchPolicy: 'no-cache',
      errorPolicy: 'ignore',
    },
    query: {
      fetchPolicy: 'no-cache',
      errorPolicy: 'all',
    },
  }
  const authLink = setContext((_, { headers }) => {
    const token = cookies().get('_bat')
    return {
      headers: {
        ...headers,
        authorization: token && token.value ? `Bearer ${token.value}` : '',
      },
    }
  })
  return new ApolloClient({
    cache: new InMemoryCache(),
    link: authLink.concat(new HttpLink({
      uri: process.env.BOSCA_SERVER_URL || 'http://127.0.0.1:8000/graphql',
    })),
    defaultOptions: defaultOptions,
  })
})

export const rootCollectionId = '00000000-0000-0000-0000-000000000000'