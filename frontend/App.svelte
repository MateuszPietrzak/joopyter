<script>
  import Cell from './lib/Cell.svelte';
</script>

<section>
  <div class="notebook">
  <Cell />
  <Cell />
  <Cell />
  <Cell />
  </div>
</section>

<style>
  .notebook {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    margin: 3rem 0rem;
    gap: 3rem;
  }
</style>