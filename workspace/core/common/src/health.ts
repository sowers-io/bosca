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

import { Health, HealthCheckResponse, HealthCheckResponse_ServingStatus } from '@bosca/protobufs'
import { type ConnectRouter } from '@connectrpc/connect'
import { logger } from './logger'

export function health(router: ConnectRouter): ConnectRouter {
  return router.service(Health, {
    check: async () => {
      return new HealthCheckResponse({
        status: HealthCheckResponse_ServingStatus.SERVING,
      })
    },
    watch: async function* (request, handler) {
      logger.info('health check watch')
      while (!handler.signal.aborted) {
        yield new HealthCheckResponse({
          status: HealthCheckResponse_ServingStatus.SERVING,
        })
        await new Promise((resolve) => setTimeout(resolve, 3000))
      }
    },
  })
}
