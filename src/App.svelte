<script lang="ts">
	export let name: string;
    import Scene from "./Scene.svelte";
</script>

<main>
    <Scene />
</main>

<style>
	h1 {
		color: #ff3e00;
		text-transform: uppercase;
		font-size: 4em;
		font-weight: 100;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>
