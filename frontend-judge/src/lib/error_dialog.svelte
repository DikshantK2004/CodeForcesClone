<script>
    export let visible = false;
    export let message = "An unexpected error has occurred.";
    export let htmlContent = ""; // Optional: To render raw HTML safely
    export let closeDialog = () => {
        visible = false;
    };
    // Event dispatcher for closing the dialog
    import { createEventDispatcher } from 'svelte';

</script>

{#if visible}
<!-- <button class="backdrop" on:click={closeDialog} type="button" aria-label="Close dialog"></button> -->
    <dialog class="error-dialog" open>
        <button class="close-button" on:click={closeDialog}></button>
        <div class="content">
            {@html message || ''}
            {#if htmlContent}
                <div class="custom-html" bind:innerHTML={htmlContent} contenteditable></div>
            {/if}
        </div>
    </dialog>
{/if}

<style>
    dialog {
        position: relative;
        border: none;
        border-radius: 8px;
        /* padding: 20px; */
        background: white;
        color: black;
        width: 40vw;
        max-width: 600px;
        z-index: 1000;
        /* box-shadow: 0 2px 10px rgba(0,0,0,0.1); */
    }
    .close-button {
        position: absolute;
        top: 10px;
        right: 10px;
        margin:auto;
        width: 20px;
        border: none;
        background: black;
        cursor: pointer;
        font-size: 24px;
        height: 8px;
    }
    /* .content {
        padding: 20px;
    } */
    .custom-html {
        margin-top: 15px;
        background: #f4f4f4;
        padding: 10px;
        border-radius: 4px;
    }
</style>
