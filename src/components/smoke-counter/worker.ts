import { Counter } from '#rust/smoke'
import type { FromWorker, ToWorker } from './messages.ts'

const counter = new Counter()

addEventListener('message', ({ data }: MessageEvent<ToWorker>) => {
  switch (data.type) {
    case 'increment':
      counter.increment()
      send({ type: 'count', value: counter.value })
      break
  }
})

function send(message: FromWorker): void {
  postMessage(message)
}
