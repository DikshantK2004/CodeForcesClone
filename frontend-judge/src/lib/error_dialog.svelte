<script>
    export let visible = false;
    export let message = "An unexpected error has occurred.";
    export let htmlContent = ""; // Optional: To render raw HTML safely
    export let closeDialog = () => {
        visible = false;
    };

</script>

{#if visible}
<!-- <button class="backdrop" on:click={closeDialog} type="button" aria-label="Close dialog"></button> -->
    <dialog class="error-dialog" open>
        <button class="close-button" on:click={closeDialog}></button>
        <div class="content">
            {@html message || ''}
            {#if htmlContent}
                <div class="custom-html" bind:innerHTML={htmlContent} contenteditable="false"></div>
            {/if}
        </div>
    </dialog>
{/if}

<style>
    dialog {
        border-radius: 8px;
        padding: 10px;
        background: #f3afaf;
        text-align: center;
        font-family: 'Courier New', Courier, monospace;
        width: auto;
        max-width: 300px;
        color: #e82a2a;
        z-index: 1000;
        box-shadow: 0 2px 10px rgba(0,0,0,0.1);
    }
    .close-button {
        position: absolute;
        top: 10px;
        right: 10px;
        margin:auto;
        width: 20px;
        border: none;
        background: #d33939;
        cursor: pointer;
        height: 8px;
    }
    .custom-html {
        margin-top: 15px;
        background: white;
        padding: 10px;
        border-radius: 4px;
    }
</style>
