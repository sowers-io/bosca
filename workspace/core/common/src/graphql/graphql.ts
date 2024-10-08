/*
 * Copyright 2024 Sowers, LLC
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

import { createSchema, GraphQLSchemaWithContext, YogaInitialContext } from 'graphql-yoga'
import { loadFiles } from '@graphql-tools/load-files'

import { fastify, FastifyRequest, FastifyReply } from 'fastify'
import { createYoga } from 'graphql-yoga'
import { logger } from '../logger'

export interface GraphQLRequestContext extends YogaInitialContext {
  request: FastifyRequest & Request
  reply: FastifyReply
}

export async function createSchemaWithContext<TContext extends GraphQLRequestContext>(): Promise<
  GraphQLSchemaWithContext<TContext>
> {
  const production = process.env.NODE_ENV === 'production'
  return createSchema<TContext>({
    typeDefs: await loadFiles('src/schema/**/*.graphql'),
    resolvers: await loadFiles(
      production ? ['dist/resolvers/*.js', 'dist/resolvers/**/*.js'] : ['src/resolvers/*.ts', 'src/resolvers/**/*.ts'],
    ),
  })
}

export async function createAndRunServer<TContext extends GraphQLRequestContext>(port: number, path: string = '/graphql') {
  const server = fastify()
  const schema = createSchemaWithContext<TContext>()
  const yoga = createYoga<TContext>({
    schema: schema,
    graphqlEndpoint: path,
    logging: {
      debug: (...args) => args.forEach((arg) => server.log.debug(arg)),
      info: (...args) => args.forEach((arg) => server.log.info(arg)),
      warn: (...args) => args.forEach((arg) => server.log.warn(arg)),
      error: (...args) => args.forEach((arg) => server.log.error(arg)),
    },
  })
  server.route({
    url: '/health',
    method: ['GET', 'POST', 'OPTIONS'],
    handler: async (request, reply) => {
      reply
        .code(200)
        .header('Content-Type', 'application/json; charset=utf-8')
        .send({ success: true })
    },
  })
  server.route({
    url: '/alive',
    method: ['GET', 'POST', 'OPTIONS'],
    handler: async (request, reply) => {
      reply
        .code(200)
        .header('Content-Type', 'application/json; charset=utf-8')
        .send({ success: true })
    },
  })
  server.route({
    url: yoga.graphqlEndpoint,
    method: ['GET', 'POST', 'OPTIONS'],
    handler: async (request, reply) => {
      // @ts-ignore
      const response = await yoga.handleNodeRequestAndResponse(request, reply, { request: request, reply: reply })
      response.headers.forEach((value, key) => {
        reply.header(key, value)
      })
      reply.status(response.status)
      reply.send(response.body)
      return reply
    },
  })
  await server.listen({ host: '0.0.0.0', port: port })
  logger.info(`Server running on 0.0.0.0:${port}`)
}