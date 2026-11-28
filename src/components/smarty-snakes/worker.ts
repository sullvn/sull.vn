import { Game, Input } from '#rust/smarty-snakes'
import type { FromWorker, ToWorker } from './messages.ts'

/**
 * Milliseconds between moves, for eight moves a second
 */
const moveInterval = 125

let game: Game | undefined
let pending = Input.None
let timer: ReturnType<typeof setInterval> | undefined

addEventListener('message', ({ data }: MessageEvent<ToWorker>) => {
  switch (data.type) {
    case 'start':
      game = new Game(data.width, data.height)
      sendFrame(game)
      break
    case 'press':
      pending = Input[data.input]

      // Waking up shouldn't wait for the next move
      if (game?.parked) tick()
      break
  }
})

send({ type: 'ready' })

/**
 * Plays a move with the latest input, keeping the clock running only while
 * the player moves
 */
function tick(): void {
  if (!game) return

  game.tick(pending)
  pending = Input.None
  sendFrame(game)

  if (game.parked) {
    clearInterval(timer)
    timer = undefined
  } else {
    timer ??= setInterval(tick, moveInterval)
  }
}

function sendFrame(game: Game): void {
  send({ type: 'frame', cells: game.cells() })
}

function send(message: FromWorker): void {
  postMessage(message)
}
