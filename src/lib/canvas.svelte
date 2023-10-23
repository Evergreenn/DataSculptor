<script>
	import Chart from 'chart.js/auto';
	import { onMount } from 'svelte';

	let canvasElement;
	export let dataConfig;
	$: dataConfig;

	onMount(() => {
		new Chart(canvasElement, dataConfig);
	});
</script>

<div>
	<canvas bind:this={canvasElement} />
</div>
