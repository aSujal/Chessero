<script lang="ts">
  import Counter from "$lib/components/Counter.svelte";

  let team = $state(["Alice", "Bob", "Charlie"]);
</script>

<main class="container">
  <h3>Team Members</h3>
  <ul>
    {#each team as member}
      <li>{member}</li>
    {/each}
  </ul>
</main>
