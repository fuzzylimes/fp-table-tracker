<script>
	import { onMount } from 'svelte';
	import LatestChart from '../components/LatestChart.svelte';
	import Loading from '../components/Loading.svelte';
	import TodayChart from '../components/TodayChart.svelte'

	let tableCount = '';
	let latest;
	let today;

	onMount(async (promise) => {

		fetch('/api/getLatest')
		.then(response => response.json())
		.then(data => {
			tableCount = data.tableCount;
			latest = data;
		})
		.catch(error => console.error(error));

		const {tsStart, tsStop} = getToday();
		fetch('/api/getByDateRange?' + new URLSearchParams({start: tsStart, stop: tsStop}))
		.then(response => response.json())
		.then(data => {
			today = data;
		})
		.catch(error => console.error(error));
	});

	const getNextUpdateTime = () => {
		const d = new Date();
		return 30 - d.getMinutes() % 30;
	}

	function getToday() {
		let now = new Date();
		let startOfDay = new Date(now.getFullYear(), now.getMonth(), now.getDate());
		let tsStart = startOfDay/1000;
		let startOfTomorrow = new Date(startOfDay);
		startOfTomorrow.setDate(startOfTomorrow.getDate() + 1);
		let tsStop = startOfTomorrow/1000;
		return {tsStart, tsStop};
	}
</script>

<main>
	<div class="container">
		{#if latest}
		<div class="section has-text-centered mt-6">
			<div class="columns is-centered is-vcentered">
				<div class="column is-half">
					<h1 class="title is-1">{tableCount} Open Tables</h1>
					<h3 class="subtitle">
						<p>Last Updated: {new Date(latest.ts * 1000)}</p>
						<p>Next Update: {getNextUpdateTime()} mins</p>
					</h3>
				</div>
				<div class="column is-half chart">
					<LatestChart {latest}/>
				</div>
			</div>
		</div>
		<div class="section has-text-centered">
			<h1 class="title is-1">Today's Activity</h1>
			<div class="columns is-centered">
				<div class="column is-half-tablet chart">
					{#await today then value}
					<TodayChart {today}/>
					{/await}
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