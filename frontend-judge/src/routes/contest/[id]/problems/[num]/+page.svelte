<script lang="ts">
    import type {PageData} from "./$types";
    export let data: PageData;
    import '$lib/contest_styles.css';
    let problem = data.problem;
    console.log(problem)
</script>
<div class="flex-in left main">

<h3>
    {problem.info.name} 
    <span style="color: gray; font-size:14px;font-weight:normal;">
        Time Limit:{problem.info.time_limit} ms
    </span>
</h3>
{@html data.problem.statement}
<h3> Sample Test Cases</h3>
{#if problem.samples?.length > 0}
<table>
    <thead>
        <th style="width:70%;"> Input</th>
        <th> Output</th>
    </thead>
    <tbody>
        {#each problem.samples as test_case}
            <tr>
                <td><pre>{test_case.input}</pre></td>
                <td><pre>{test_case.output}</pre></td>
            </tr>
        {/each}
    </tbody>
</table>
{/if}
</div>

<style>
    .main{
        text-align: left;
        font-family: 'PT Sans Narrow';
        padding: 20px;
        width:100%;
    }

    table {
    width: 100%;
    border-collapse: collapse; /* Ensure borders are collapsed into a single border */
    box-shadow: 0 4px 8px rgba(0,0,0,0.1); /* Adds subtle shadow for depth */
    margin-top: 20px; /* Adds space above the table */
    font-family: Arial, sans-serif; /* Ensures a clean, readable font is used */
}

thead {
    text-align: left; /* Aligns text to the left for headings */
    background-color: lightgray; /* A light grey background for the header */
}

th, td {
    border: 1px solid #ddd; /* Lighter border for a less intense separation */
    padding: 8px 16px; /* Padding inside cells for better text alignment */
}

th {
    font-weight: bold; /* Makes header text bold */
    color: #333; /* Dark grey color for text for better readability */
}

tbody tr:nth-child(odd) {
    background-color: #fafafa; /* Zebra-striping for rows */
}

tbody tr:hover {
    background-color: #f0f0f0; /* Slight highlight on hover */
}

td {
    vertical-align: top; /* Aligns text to the top of the cell for better readability with multiline content */
}
pre{
    margin:0;
    padding:0;
}

</style>