<script lang="ts">
  //let count = 0; would also work
  //The $state makes it clear which variables should update the UI
  let count = $state(0);
</script>

<button onclick={() => count++}>
  Count: {count}
</button>
