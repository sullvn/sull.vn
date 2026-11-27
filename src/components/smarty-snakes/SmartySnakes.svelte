<script lang="ts">
  import { onMount } from 'svelte'
  import { type BoardSize, boardSize } from './board-size.ts'
  import { cellNames, type FromWorker, type Press, type ToWorker } from './messages.ts'

  /**
   * Share of the viewport's height the board may fill
   */
  const maxViewportHeight = 0.7

  let board: HTMLButtonElement
  let size = $state<BoardSize>()
  let cells = $state<Uint8Array>()
  let worker: Worker | undefined

  onMount(() => {
    const measured = boardSize(board.clientWidth, innerHeight * maxViewportHeight)
    size = measured

    worker = new Worker(new URL('./worker.ts', import.meta.url), { type: 'module' })
    worker.addEventListener('message', ({ data }: MessageEvent<FromWorker>) => {
      switch (data.type) {
        case 'ready':
          send({ type: 'start', ...measured })
          break
        case 'frame':
          cells = data.cells
          break
      }
    })

    return () => worker?.terminate()
  })

  function onkeydown(event: KeyboardEvent & { currentTarget: HTMLButtonElement }): void {
    if (event.key === 'Escape') {
      event.currentTarget.blur()
      return
    }

    const input = arrows[event.key]
    if (!input) return

    // Arrows steer instead of scrolling while the board has focus
    event.preventDefault()
    press(input)
  }

  /**
   * Clicks focus the board, for the arrow keys
   *
   * Safari doesn't focus buttons on click by itself.
   *
   */
  function onclick(event: MouseEvent & { currentTarget: HTMLButtonElement }): void {
    event.currentTarget.focus()
  }

  /**
   * Taps turn toward their side of the board
   *
   * A touch that scrolls the page ends without a `pointerup`, so swipes don't
   * steer.
   *
   */
  function onpointerup(event: PointerEvent & { currentTarget: HTMLButtonElement }): void {
    if (event.pointerType !== 'touch') return

    const { left, width } = event.currentTarget.getBoundingClientRect()
    press(event.clientX - left < width / 2 ? 'TapLeft' : 'TapRight')
  }

  function press(input: Press['input']): void {
    send({ type: 'press', input })
  }

  function send(message: ToWorker): void {
    worker?.postMessage(message)
  }

  const arrows: Partial<Record<string, Press['input']>> = {
    ArrowUp: 'Up',
    ArrowRight: 'Right',
    ArrowDown: 'Down',
    ArrowLeft: 'Left',
  }
</script>

<button type="button" bind:this={board} aria-label="Smarty Snakes" {onkeydown} {onclick} {onpointerup}>
  {#if size && cells}
    <svg viewBox="0 0 {size.width} {size.height}" aria-hidden="true">
      {#each cells as cell, index (index)}
        {@const name = cellNames[cell]}
        {@const x = index % size.width}
        {@const y = Math.floor(index / size.width)}
        {#if name === 'food'}
          <circle class="food" cx={x + 0.5} cy={y + 0.5} r="0.3" />
        {:else if name && name !== 'empty'}
          <rect class={name} {x} {y} width="1" height="1" />
        {/if}
      {/each}
    </svg>
  {/if}
</button>

<style>
  button {
    display: block;
    inline-size: 100%;
    padding: 0;
    border: none;
    background: none;
    touch-action: manipulation;
  }

  button:focus-visible {
    outline: 0.125rem solid var(--fg);
  }

  svg {
    inline-size: 100%;

    /* Shows the walls without competing with the snakes */
    outline: 0.0625rem solid color-mix(in oklab, var(--fg) 25%, transparent);
  }

  /* Bodies are paler than heads, so a parked snake shows which way it faces */
  .you {
    fill: color-mix(in oklab, var(--green) 60%, var(--bg));
  }

  .yourHead {
    fill: var(--green);
  }

  .clone {
    fill: color-mix(in oklab, var(--purple) 60%, var(--bg));
  }

  .cloneHead {
    fill: var(--purple);
  }

  .food {
    fill: var(--red);
  }
</style>
