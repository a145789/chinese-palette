<script lang="ts">
  import init, { greet, SortBy, type Colors } from "rust-wasm"
  import { KInput } from "@ikun-ui/input"
  import { KSwitch } from "@ikun-ui/switch"
  import { KMessage } from "@ikun-ui/message"
  import { KCollapse } from "@ikun-ui/collapse"

  let colors: Colors
  let list: string | any[] = []
  function getList() {
    list = colors.get_colors(value, sortby)
    console.log(list)
    if (list.length === 0) {
      KMessage({
        content: "什么也没有哦",
        type: "info",
      })
    }
  }

  let value = "粉"
  const onInput = (e: CustomEvent) => {
    value = e.detail
    getList()
  }

  let sortby = SortBy.Asc

  async function getJson() {
    const json = await fetch("/color.json")
    const str = await json.text()
    await init()
    colors = greet(str)
    getList()
  }
  getJson()
</script>

<main>
  <div>
    <KInput
      placeholder="输入一种颜色，例如：黑/灰/白"
      on:input={onInput}
      {value}
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
      on:change={getList}
    />
    <span class="text-14px ml-2">色系由大到小</span>
  </div>

  <div class="mt-5">
    {#each list as { hex, name }}
      <KCollapse
        content="Sadness makes people more acute."
        title={`${name} ${hex}`}
        cls="my-4"
      />
    {/each}
  </div>
</main>

<style>
  main {
    width: 40%;
    margin: 0 auto;
    padding-top: 100px;
  }

  @media (max-width: 480px) {
    main {
      width: 100%;
      padding-top: 30px;
    }
  }
</style>
