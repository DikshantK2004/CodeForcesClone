
    <script lang="ts">
        import type { PageData } from "./$types";
        import { onMount, onDestroy } from "svelte";

        export let data: PageData;

        // onDestroy(() => {
        //     clearInterval(interval);
        // });
    </script>

    <div class="container">
        <table>
            <thead>
                <tr>
                    <th>Contest Name</th>
                    <th>Start Time(IST)</th>
                    <th> Duration</th>
                    <th> Status</th>
                </tr>
            </thead>
            <tbody>
                {#each data.contests as contest}
                    <tr>
                        <td class="name">{contest.name}</td>
                        <td class="time">
                            <div>{contest.start_date}</div>
                            <div>{contest.start_time}</div>
                        </td>
                        <td>{contest.duration}</td>
                        <td class="status">
                            {#if contest.start_msecs - Date.now() > 0}
                                Not Started
                            {:else }
                                <a href ={`/contest/{contest.id}`}>Enter</a>
                            {/if}
                        </td>
                    </tr>
                {/each}
            </tbody>
        </table>
    </div>

    
<style>
.container{
    margin:auto;
    width:auto;
    max-width: max-content;
    font-size: 18px;
}

table {
    border-collapse: collapse;
    margin: 20px;
}
th, td {
    padding: 15px;
    text-align: center;
    border-bottom: 1px solid #ddd;
}

th {
    background-color: #f2f2f2;
}

tr:nth-child(odd){
    background-color:#DEF9C4;

}

tr:nth-child(even) {
    background-color:  #9CDBA690;
}

tr:hover {
    background-color: #ddd;
}

.name{
    max-width: 250px;
}
</style>