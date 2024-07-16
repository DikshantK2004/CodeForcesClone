<script lang="ts">
	import type { PageData } from './$types';
	import Error from './+error.svelte';

	import Header from './Header.svelte';
	export let data: PageData;
</script>

<Header username={data?.username} />
<slot />

<style>
	:global(body) {
		font-family: 'Urbanist';
		background: linear-gradient(to top, #fff8f3 0%, #ecceae 100%);
		margin: 0;
		height: 100vh;
	}
</style>
