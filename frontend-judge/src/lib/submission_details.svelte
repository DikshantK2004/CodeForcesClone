<script lang="ts">
	import  Prism from "prismjs";
    import {convertToLocalIST} from "$lib/utils";
	let code = 'print("Hello, World!")';
	let language = 'python';
    import "$lib/prism.python.min.js";
	export let details_visible = false;
	export let submission_id = 0;
	let dialog: HTMLDialogElement;
	let closeDialog = () => {
		details_visible = false;
	};


    async function request() {
        let res = await fetch(`/api/submissions/${submission_id}`);
        let js = await res.json();
        return js;
    }


	$: if (dialog && details_visible) {
		dialog.showModal();
	}

</script>



<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-noninteractive-element-interactions -->

<dialog
	bind:this={dialog}
	on:close={() => {
		details_visible = false;
	}}
	on:click|self={() => dialog.close()}
>
	<button
		class="close-button"
		on:click|stopPropagation={() => {
			dialog.close();
			submission_id = 0;
		}}>⨯</button
	>
	<div class="content">

            {#await request() }
                <p>Loading...</p>
            {:then data}
                    <div>Submission ID: {data.id} on {convertToLocalIST(data.created_at)}: 
                    <span class:verdict-green={data.verdict === 'Accepted'} class:verdict-red={data.verdict.startsWith('Wrong')} class:verdict-others={!data.verdict.startsWith('Accepted') && !data.verdict.startsWith('Wrong')}>{data.verdict}</span>
                    </div><hr>
                    <pre class = "code">{@html Prism.highlight(data.code, Prism.languages[data.extension], data.extension)}</pre>
                    <h3>Judgement</h3>

                    {#each data.test_results as test}
                    <hr>
                        <div class="particular-test">
                            
                            <h4>Test Case {test.test_num}: <span class="info"> time: {test.time_taken} ms, verdict: {test.verdict}</span> </h4> 
                            <p>Input:</p>
                            <pre class = "code">{test.content}</pre>
                            <p>Output:</p>
                            <pre class = "code">{test.out}</pre>
                        </div>
                        {/each}
                            
            {:catch error}
                <p>Failed to fetch data: {error}</p>
                
            {/await}
	</div>
	
</dialog>

<style>

    .info, h4{
        margin:0;
        padding:0;
        
    }

    p{
        margin:0;
    }
    .info{
        text-decoration: none;
        font-weight: normal;
    }
    h3{
        margin: 10px 0;
        font-weight:600;
    }

    
    	.code {
		white-space: pre-wrap;
	}
	dialog {
		background-color: white;
		border-radius: 8px;
		border: 4px solid #00000080;
		box-shadow: 0px 2px 4px rgba(0, 0, 0, 0.2);
		padding: 16px;
		font-family: Arial, Helvetica, sans-serif;
		font-size: 22px;
	}

	button.close-button {
		position: absolute;
		top: 8px;
		right: 8px;
		background-color: #d2e2d2;
		border: none;
		cursor: pointer;
	}

	

	.content {
		border-radius: 8px;
		width: 55vw;
		text-align: left;
		height: 55vh;
		margin: 16px;
		padding: 16px;
		border: 3px solid #ddd;
        overflow-y: scroll;
        font-size: 18px;

	}

	.verdict-green {
		color: green;
	}
	.verdict-red {
		color: red;
	}

	.verdict-others {
		color: #2e236c;
	}

	a {
		color: #615efc;
	}

    pre{
        background-color: #EEEEEE;
        margin:0;
        padding:0;

    }


</style>
