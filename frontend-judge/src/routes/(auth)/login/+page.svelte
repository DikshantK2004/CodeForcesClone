<script lang="ts">
	import { goto } from '$app/navigation';
	import type { ActionData } from './$types';
	import '$lib/auth_styles.css';
	import ErrorDialog from '$lib/error_dialog.svelte';
	import { enhance } from '$app/forms';
	export let form: ActionData;
</script>

<form class="input-form" method="POST" use:enhance>
	<h1 id="tex">Login to HELL</h1>
	<input type="email" name="email" class="inputprops" placeholder="email" /> <br />
	<input type="password" name="password" class="inputprops" placeholder="password" /><br />
	<input type="submit" value="Login" class="buttonprops" /><br />
	<button
		type="button"
		class="buttonprops"
		on:click={() => {
			goto('/register');
		}}>Register Instead</button
	>
</form>

<ErrorDialog htmlContent={form?.error} visible={form?.error !== undefined} message="" />
