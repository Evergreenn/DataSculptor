<script lang="ts">
	import Canvas from './canvas.svelte';

	let canvasElement: HTMLCanvasElement;
	export let chartData;
	export let xaxis = 0;
	export let yaxis = 0;

	let labels = [];
	let data = [];
	let info = '';

	const dataConfig = {
		type: 'bar',
		data: {
			labels,
			datasets: [
				{
					label: '# of Votes',
					data,
					borderWidth: 1
				}
			]
		},
		options: {
			scales: {
				y: {
					beginAtZero: true
				}
			}
		}
	};

	$: {
		data = [];
		labels = [];
		for (let i = 0; i < chartData.data.length; i++) {
			labels.push(chartData.data[i][xaxis]);
			data.push(chartData.data[i][yaxis]);
		}
		info = chartData.header[yaxis];
		dataConfig.data.labels = labels;
		dataConfig.data.datasets[0].data = data;
		dataConfig.data.datasets[0].label = info;
		console.log(dataConfig);
	}
</script>

<Canvas {dataConfig} />
