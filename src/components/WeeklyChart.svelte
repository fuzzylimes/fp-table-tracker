<script>
    import { onMount } from 'svelte';
    export let week;
    let weeklyChartCanvas;
    let labels = [];
    let tables = [];

    let config;
    let myChart;

    $: week && buildChart(week);

    const colors = ["#1b4079","#4d7c8a","#7f9c96","#8fad88","#9D8DF1","#FF4000","#faa381"]


    const buildChart = (week) => {
        const grouped = groupday(week);
        const labels = buildLabels();
        const datasets = Object.keys(grouped).map((day, i) => {
            return {
                label: day,
                backgroundColor: colors[i],
                borderColor: colors[i],
                data: grouped[day].map(record => record.tableCount),
            }
        });
        console.log(datasets);
        const data = {
            labels,
            datasets
        };
        config = {
            type: 'line',
            data,
            options: {
                responsive: true,
                maintainAspectRatio: false,
                plugins: {
                    legend: {
                        position: 'bottom',
                        title:{
                            display: true,
                            text: 'Days'
                        }
                    }
                }
            }
        };
        if(myChart) {
            myChart.destroy();
            let ctx = weeklyChartCanvas.getContext('2d');
            myChart = new Chart(
                ctx,
                config
            );
        }
    };

    onMount(() => {
        let ctx = weeklyChartCanvas.getContext('2d');
        myChart = new Chart(
            ctx,
            config
        );
    })

    function groupday(data){
        const r = data.reduce((acc, v) => {
            const time = new Date(v.ts * 1000)
            const key = `${time.getMonth()}/${time.getDate()}/${time.getFullYear()}`;
            if (acc.hasOwnProperty(key)) {
                acc[key].push(v);
                return acc;
            } else {
                acc[key] = [];
                acc[key].push(v);
                return acc;
            }
        }, {})
        return r
    }

    function buildLabels() {
        var x = 30; //minutes interval
        var times = []; // time array
        var tt = 0; // start time
        //loop to increment the time and push results in array
        for (var i=0;tt<24*60; i++) {
            var hh = Math.floor(tt/60); // getting hours of day in 0-24 format
            var mm = (tt%60); // getting minutes of the hour in 0-55 format
            times[i] = ("0" + hh).slice(-2) + ':' + ("0" + mm).slice(-2); // pushing data in array in [00:00 - 24:00 format]
            tt = tt + x;
        }
        return times;
    }
</script>

<main>
    <canvas bind:this={weeklyChartCanvas} id="weeklyChart" width="400" height="400"></canvas>
</main>

<style></style>