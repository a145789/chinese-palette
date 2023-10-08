<script lang="ts">
  import { KIcon } from "@ikun-ui/icon"
  import { type Color } from "rust-wasm"
  import { generate } from "@ant-design/colors"
  import { copy } from "svelte-copy"
  import { KMessage } from "@ikun-ui/message"
  import { onMount, onDestroy } from "svelte"

  export let item: Color

  enum Luminance {
    black,
    white,
  }

  const [r, g, b] = item.RGB
  const luminance =
    (0.299 * r + 0.587 * g + 0.114 * b) / 255 > 0.5
      ? Luminance.black
      : Luminance.white

  function hexToRgb(hex: string) {
    const r = parseInt(hex.slice(1, 3), 16)
    const g = parseInt(hex.slice(3, 5), 16)
    const b = parseInt(hex.slice(5, 7), 16)

    return `rgb(${r}, ${g}, ${b})`
  }
  const palettes = [
    generate(item.hex).map((hex) => [hex, hexToRgb(hex)]),
    generate(item.hex, {
      theme: "dark",
    }).map((hex) => [hex, hexToRgb(hex)]),
  ]

  let toggle = false
  let fistRender = true
  const color = [item.hex, `rgb(${item.RGB.join()})`]

  const textColor = palettes[0][luminance === Luminance.black ? 9 : 0][0]
  const borderColor = palettes[0][luminance === Luminance.black ? 6 : 4][0]

  let inst: any = null
  function copySuccess(text: string) {
    if (inst) {
      KMessage.clear(inst)
    }
    inst = KMessage({
      content: `${text} 复制成功`,
      type: "success",
    })
  }

  let showCopy = false
  let divRef: null | HTMLElement = null
  function changeShowCopyTrue() {
    showCopy = true
  }
  function changeShowCopyFalse() {
    showCopy = false
  }
  onMount(() => {
    divRef!.addEventListener("mouseenter", changeShowCopyTrue)
    divRef!.addEventListener("mouseleave", changeShowCopyFalse)
  })

  onDestroy(() => {
    divRef!.removeEventListener("mouseenter", changeShowCopyTrue)
    divRef!.removeEventListener("mouseleave", changeShowCopyFalse)
  })
</script>

<div
  bind:this={divRef}
  class="w-full my-2 box-border relative rounded overflow-hidden"
  style="border: 1px solid {borderColor};"
>
  <div
    class="h-10 w-full flex items-center"
    style="background-color: {item.hex};"
  >
    <KIcon
      icon="i-carbon-chevron-right"
      attrs={{
        style: `color: ${textColor};`,
      }}
      cls="mx-2 cursor-pointer transition-all {toggle ? 'rotate-90' : ''}"
      on:click={() => {
        toggle = !toggle
        fistRender = false
      }}
    />
    <div class="h-full flex items-center flex-1" style="color: {textColor};">
      {item.name}
    </div>
  </div>

  <div
    class="w-full overflow-hidden flex justify-around items-center transition-all duration-300 ease-in-out"
    style="height: {toggle
      ? `calc(2rem * ${palettes[0].length} + 0.5rem * (1 + ${palettes[0].length}))`
      : 0};"
  >
    {#if !fistRender}
      {#each palettes as palette}
        <div class="w-49%">
          {#each palette as color, index}
            <div
              class="h-8 w-full my-2 flex"
              style="background-color: {color[0]};"
            >
              {#each color as t}
                <div
                  class="w-50% h-full flex justify-center items-center cursor-pointer text-14px"
                  style="color: {palette[
                    index > 5 ? 0 : palette.length - 1
                  ][0]};"
                  use:copy={t}
                  on:svelte-copy={() => copySuccess(t)}
                >
                  {t}
                </div>
              {/each}
            </div>
          {/each}
        </div>
      {/each}
    {/if}
  </div>

  <!-- copy -->
  <div
    class="h-10 absolute flex transition-all duration-300 ease-in-out md:w-50% w-80% md:right--50% right--80% top-0 {showCopy
      ? 'right-0!'
      : ''}"
  >
    <div class="md:w-full lt-md:flex-1 flex">
      {#each color as t}
        <div
          class="h-full w-50% flex justify-center items-center cursor-pointer text-14px font-bold"
          style="background-color: {textColor};color: {t}"
          use:copy={t}
          on:svelte-copy={() => copySuccess(t)}
        >
          {t}
        </div>
      {/each}
    </div>
  </div>
</div>
