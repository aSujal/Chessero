<script lang="ts">
  let { children } = $props();
</script>

<svelte:head>
  <link rel="icon" href="/svelte.svg" />
</svelte:head>
<section>
  <h2>Chessero</h2>
  <nav>
    <a href="/">Home</a>
    <a href="/tutorial">Tutorial</a>
    <a href="/counter">Counter</a>
  </nav>

  <!-- older method or something... <slot/> -->
  {@render children?.()}
</section>
