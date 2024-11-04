import { test } from 'vitest'
import { BoscaSink } from './bosca'
import { addSink, logImpression } from './sink'

test('impression', async () => {
  const sink = new BoscaSink('http://127.0.0.1:8009/events', 'a', 'b', 'c')
  addSink(sink)
  const impressions = []
  for (let i = 0; i < 1_000_000; i++) {
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
    if (i % 500 === 0) {
      await sink.flush()
    }
  }
  await Promise.all(impressions)
  await sink.flush()
}, 600_000)