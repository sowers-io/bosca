import { test } from 'vitest'
import { BoscaSink } from './bosca'
import { addSink, logImpression } from './sink'

test('impression', async () => {
  console.log('adding impressions...')
  const sink = new BoscaSink('http://127.0.0.1:8009', 'a', 'b', 'c')
  addSink(sink)
  const impressions = []
  for (let i = 0; i < 50; i++) {
    impressions.push(logImpression({
      id: 'asdf',
      type: 'asdf',
      content: [
        {
          id: 'a',
          type: 'b',
          index: 1,
          percent: 0.9,
        },
      ],
      extras: {},
    }))
  }
  await Promise.all(impressions)
  await new Promise((resolve) => {
    setTimeout(resolve, 3000)
  })
  console.log('...impressions')

  await sink.flush()
}, 600_000)