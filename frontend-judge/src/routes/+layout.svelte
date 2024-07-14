<script lang="ts">
	import type { PageData } from "./$types";
	import Error from "./+error.svelte";


	import Header from "./Header.svelte";
    export let data : PageData;
</script>

<Header username={data?.username}/>
<slot/>


<style>
    :global(body) {
        font-family : 'Urbanist';
        background: linear-gradient(to top, #FFF8F3 0%, #ECCEAE 100%);
        margin:0;
        height:100vh;

    }

    


</style>