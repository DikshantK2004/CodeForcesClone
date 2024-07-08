<script>

	import Header from "./Header.svelte";
    export let username;
</script>

<Header username={username}/>
<slot/>



<style>
    :global(body) {
        font-family : 'Urbanist';
        background: linear-gradient(to top, #FFF8F3 0%, #ECCEAE 100%);
        margin:0;
        padding:0;
        height:100vh;
    }

    


</style>