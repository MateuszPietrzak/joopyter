<script lang="ts">
    import Monaco from 'svelte-monaco';

    let value = 'print("Hello, World!")';
</script>

<div class="cell">
<Monaco
options={{ language: 'python', automaticLayout: true }}
	    theme="vs-dark"
		on:ready={(event) => console.log(event.detail)}
		bind:value
/>
</div>
 
<style>
    .cell {
        height: 10rem;
    } 
</style>