<script>
    import { onMount } from 'svelte';
    export let today;
    let todayChartCanvas;
    let labels = [];
    let tables = [];

    let config;
    let myChart;

    $: today && buildChart(today);

    const buildChart = (today) => {
        today.forEach(e => {
            let d = new Date(e.ts * 1000);
            labels.push(`${d.getHours().toString().padStart(2, "0")}:${d.getMinutes().toString().padStart(2, "0")}`);
            tables.push(e.tableCount);
        });
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
            type: 'line',
            data,
            options: {
                responsive: true,
                maintainAspectRatio: false,
                plugins: {
                    title: {
                        display: true,
                        text: 'Active Tables',
                    },
                    legend: {
                        display: false,
                    }
                }
            }
        };
        if(myChart) {
            myChart.destroy();
            let ctx = todayChartCanvas.getContext('2d');
            myChart = new Chart(
                ctx,
                config
            );
        }
    };

    onMount(() => {
        let ctx = todayChartCanvas.getContext('2d');
        myChart = new Chart(
            ctx,
            config
        );
    })
</script>

<main>
    <canvas bind:this={todayChartCanvas} id="todayChart" width="400" height="400"></canvas>
</main>

<style></style>