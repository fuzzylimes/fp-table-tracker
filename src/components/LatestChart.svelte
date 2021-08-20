<script>
    import { onMount } from 'svelte';
    export let latest;
    let latestChartCanvas;
    let labels = [];
    let tables = [];

    let config;
    let myChart;

    $: latest && buildChart(latest);

    const buildChart = (latest) => {
        labels = Object.keys(latest.games);
        tables = labels.map(e => {
            return latest.games[e].reduce((t,v) => {
                return t + v.tableCount;
            }, 0);
        });
        labels = labels.map(v => v.replaceAll('_', '.'));
        const data = {
            labels: labels,
            datasets: [{
                label: 'Tables',
                backgroundColor: '#3c9fc9',
                borderColor: '#3c9fc9',
                data: tables,
            }]
        };
        config = {
            type: 'bar',
            data,
            options: {
                responsive: true,
                maintainAspectRatio: false,
                scale: {
                    ticks: {
                        precision: 0,
                    }
                },
                plugins: {
                    title: {
                        display: true,
                        text: 'Current Games',
                    },
                    legend: {
                        display: false,
                    }
                }
            }
        };
        if(myChart) {
            myChart.destroy();
            let ctx = latestChartCanvas.getContext('2d');
            myChart = new Chart(
                ctx,
                config
            );
        }
    }

    onMount(() => {
        if (latestChartCanvas) {
            let ctx = latestChartCanvas.getContext('2d');
            myChart = new Chart(
                ctx,
                config
            );
        }
    })
</script>

<main>
    <!-- <div class="columns is-centered has-text-centered">
        <div class="column is-half-tablet latest-chart"> -->
            {#if latest.tableCount === 0}
            <h2 class="title">No Open Games</h2>
            {:else}
            <canvas bind:this={latestChartCanvas} id="latestChart" width="400" height="400"></canvas>
            {/if}
        <!-- </div>
    </div> -->
</main>

<style>
</style>