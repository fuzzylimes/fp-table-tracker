<script>
	import { onMount } from 'svelte';
	import Loading from '../components/Loading.svelte';
	import WeeklyChart from '../components/WeeklyChart.svelte';

	let tableCount = '';
	let week;

	

	onMount(async (promise) => {

		const {tsStart, tsStop} = getWeek();
		console.log(tsStart, tsStop);
		fetch('/api/getByDateRange?' + new URLSearchParams({start: tsStart, stop: tsStop}))
		.then(response => response.json())
		.then(data => {
			week = data;
		})
		.catch(error => console.error(error));
	});

	function getWeek() {
		let now = new Date();
		let startOfDay = new Date(now.getFullYear(), now.getMonth(), now.getDate());
		let tsStop = startOfDay/1000;
		let startOfLastWeek = new Date(startOfDay);
		startOfLastWeek.setDate(startOfLastWeek.getDate() - 7);
		let tsStart = startOfLastWeek/1000;
		return {tsStart, tsStop};
	}
</script>

<main>
	<div class="container">
		{#if week}
		<div class="section has-text-centered mt-6">
			<div class="columns is-centered is-vcentered">
				<div class="column is-three-quarters chart">
					<div class="section">
						<h1 class="title">Last 7 Days</h1>
					</div>
					<WeeklyChart {week}/>
				</div>
			</div>
		</div>
		{:else}
		<Loading />
		{/if}
	</div>
</main>


<style>
.chart {
	max-height: 500px;
}
</style>