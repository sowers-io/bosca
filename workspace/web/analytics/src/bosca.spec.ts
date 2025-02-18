import { expect, test } from 'vitest'
import { BoscaSink } from './bosca'
import { addSink, logImpression } from './sink'
import { DelayedAction } from './delayed_action'

test('impression', async () => {
  console.log('adding impressions...')
  const sink = new BoscaSink('http://127.0.0.1:8009', 'a', 'b', 'c')
  addSink(sink)
  const impressions = []
  for (let i = 0; i < 5000; i++) {
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

  expect(await sink.pendingSize()).toEqual(0)
  expect(sink.flushed).toEqual(5001) // includes session event
  expect(sink.failures).toEqual(0)
}, 600_000)

test('delayed action', async () => {
  const test: { [key: string]: number } = {
    'flushed': 0,
    'delayed': 0,
  }

  let action = new DelayedAction(async () => {
    console.log('flushing...')
    test['flushed']++
  }, () => {
    console.log('...done')
  })

  await action.promise()

  expect(test['flushed']).toEqual(1)

  action = new DelayedAction(async () => {
    console.log('flushing...')
    test['flushed']++
  }, () => {
    console.log('...done')
  })

  action.promise().then(() => test['delayed']++)
  action.delay()
  expect(test['flushed']).toEqual(1)
  expect(test['delayed']).toEqual(0)
  action.delay()
  expect(test['flushed']).toEqual(1)
  expect(test['delayed']).toEqual(0)
  await action.promise()
  expect(test['flushed']).toEqual(2)
  expect(test['delayed']).toEqual(1)
})