<script lang="ts">
  import init, { greet, SortBy, type Colors, type ColorResult } from "rust-wasm"
  import { KInput } from "@ikun-ui/input"
  import { KSwitch } from "@ikun-ui/switch"
  import { KMessage } from "@ikun-ui/message"
  import Collapse from "./lib/Collapse.svelte"
  import { KBacktop } from "@ikun-ui/backtop"
  import { onMount } from "svelte"

  let colors: Colors
  let list: ColorResult["data"] = []
  let value = "粉"
  let sortby = SortBy.Asc
  const page = 0
  let limitGap = 20
  let limit = limitGap
  let total = 0
  let messageInst: any = null
  let listRef: null | HTMLElement = null
  function getList() {
    const { data, total: _total } = colors.get_colors(
      value,
      sortby,
      page,
      limit
    )
    list = data
    total = _total
    if (list.length === 0) {
      if (messageInst !== null) {
        KMessage.clear(messageInst)
      }
      messageInst = KMessage({
        content: "什么也没有哦",
        type: "info",
      })
    } else {
      KMessage.clearAll()
    }
  }

  function switchChange(e: CustomEvent<any>) {
    sortby = e.detail.newVal
    limit = limitGap
    getList()
  }

  function debounce(fn: (...arg: unknown[]) => void, delay = 90) {
    let timer: number | null = null
    return function () {
      if (timer) {
        clearTimeout(timer)
      }
      timer = setTimeout(() => {
        fn(arguments)
      }, delay)
    }
  }

  async function getJson() {
    const json = await fetch(
      import.meta.env.DEV ? "/color.json" : "/chinese-palette/color.json"
    )
    const str = await json.text()
    await init()
    colors = greet(str)
  }
  onMount(async () => {
    await getJson()
    const { top } = listRef!.getBoundingClientRect()
    const { clientHeight } = document.documentElement

    limitGap = Math.ceil((clientHeight - top) / 40)
    limit = limitGap

    getList()
    window.addEventListener(
      "scroll",
      debounce(() => {
        if (limit > total) {
          return
        }
        if (
          document.documentElement.scrollTop + window.innerHeight >=
          document.documentElement.scrollHeight - 20
        ) {
          limit += limitGap
          getList()
        }
      })
    )
  })
</script>

<main>
  <div>
    <KInput
      bind:value
      placeholder="输入颜色名称或者hex值，例如：黑/灰/白/#83cbac"
      useCompositionInput
      on:compositionInput={() => {
        limit = limitGap
        getList()
      }}
    />
  </div>

  <div class="mt-6 flex items-center justify-end">
    <span class="text-14px mr-2">色系由小到大</span>
    <KSwitch
      bind:value={sortby}
      checkedColor="bg-sky-500"
      unCheckedColor="bg-green-500"
      unCheckedValue={SortBy.Asc}
      checkedValue={SortBy.Desc}
      on:change={switchChange}
    />
    <span class="text-14px ml-2">色系由大到小</span>
  </div>

  <div bind:this={listRef} class="mt-5">
    {#each list as item}
      {#key item.hex}
        <Collapse {item} />
      {/key}
    {/each}
  </div>
</main>
<KBacktop showHeight={100} />

<style>
  main {
    width: 40%;
    margin: 0 auto;
    padding-top: 100px;
  }

  @media (max-width: 760px) {
    main {
      width: 100%;
      padding-top: 30px;
    }
  }
</style>
