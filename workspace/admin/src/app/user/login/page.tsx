import { Metadata } from 'next'
import { gql } from '@apollo/client'
import { getClient } from '@/lib/client'
import { redirect } from 'next/navigation'
import { cookies, headers } from 'next/headers'
import { LoginForm } from '@/app/user/login/form'

export const metadata: Metadata = {
  title: 'Login',
  description: 'Bosca',
}

const loginQuery = gql`
  query Login($identifier: String!, $password: String!) {
    security {
      login {
        password(identifier: $identifier, password: $password) {
          principal {
            id
            groups {
              id
              name
            }
          }
          token {
            token
          }
        }
      }
    }
  }
`

export default async function Page() {
  async function login(formData: FormData) {
    'use server'
    const variables = {
      identifier: formData.get('username'),
      password: formData.get('password'),
    }
    const { data } = await getClient().query({ query: loginQuery, variables: variables })
    const host = headers().get('host')?.split(':')[0].replaceAll('admin.', '') || undefined
    cookies().set('_bat', data.security.login.password.token.token, { domain: host })
    redirect('/')
  }
  return (
    <div className="w-screen h-screen flex justify-center items-center">
      <LoginForm login={login} />
    </div>
  )
}