<script>
	import Charts from '$lib/charts.svelte';
	import { invoke } from '@tauri-apps/api/tauri';

	let extracted = invoke('extract_data_from_csv', {
		file: '/home/guillaume/Downloads/2023-10-11-981_segments_export_1697032083.csv'
	});
	let xaxis;
	let yaxis;

	// let labels = [];
	// let chartData = [];

	// for (let i = 0; i < value.data.length; i++) {
	// 	labels.push(value.data[i][xaxis]);
	// 	chartData.push(value.data[i][yaxis]);
	// }
</script>

<h1>Welcome to SvelteKit</h1>

{#await extracted then value}
	<div class="flex flex-row gap-4">
		<label class="label">
			<span> Select a x axis</span>
			<select class="select" bind:value={xaxis}>
				{#each Object.entries(value.header) as [key, header]}
					<option value={key}>{header}</option>
				{/each}
			</select>
		</label>
		<label class="label">
			<span> Select a y axis</span>
			<select class="select" bind:value={yaxis}>
				{#each Object.entries(value.header) as [key, header]}
					<option value={key}>{header}</option>
				{/each}
			</select>
		</label>
	</div>

	{xaxis}
	{yaxis}

	<Charts chartData={value} {xaxis} {yaxis} />
{/await}
