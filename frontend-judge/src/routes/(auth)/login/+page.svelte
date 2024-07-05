<script lang="ts">
    import { goto } from '$app/navigation';
    import type { ActionData } from './$types';

    export let form: ActionData;

    let dialogVisible = false;

    // Function to show the dialog
    function showErrorDialog() {
        dialogVisible = true;
    }

    // Function to hide the dialog
    function closeDialog() {
        dialogVisible = false;
    }

    // Redirect on successful login
    if (form?.status === 200) {
        goto('/');
    }
</script>

<form class="input-form" action="?/login" method="POST">
    <h1 id="tex">Login to HELL</h1>
    <input type="password" name="password" class="inputprops" placeholder="password"><br>
    <input type="submit" value="Login" class="buttonprops"><br>
    <button type="button" class="buttonprops" on:click={showErrorDialog}>Show Error</button>
    
    {#if dialogVisible}
    <dialog open class="error-dialog">
        <button class="close-button" on:click={closeDialog}>×</button>
        {@html form?.error || ''}
    </dialog>
    {/if}
</form>

<style>
    .error-dialog {
        position: fixed;
        background-color: white;
        color: black;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        width: 40vw;
    }
</style>