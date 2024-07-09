<script lang="ts">
    import { onMount } from 'svelte';
    import { browser } from '$app/environment';
    import { goto } from '$app/navigation';

    export let username : string | undefined ;


    function handleLogin() {
        goto('/login');
    }

    function handleRegister() {
        goto('/register');
    }

    function handleLogout() {
        
        goto('/'); // Redirect to home page
    }
</script>

<nav class="header">
    <div class="logo">
        <a href="/">MyApp</a>
    </div>
    <div class="navigation">
        {#if username}
            <span>Welcome, {username}</span>
            <button on:click={handleLogout}>Logout</button>
        {:else}
            <button on:click={handleLogin}>Login</button>
            <button on:click={handleRegister}>Register</button>
        {/if}
    </div>
</nav>

<style>

    .header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 0 40px;
        background: #E0A75E; /* Semi-transparent background */
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
        height: 60px;
    }

    .logo a {
        font-size: 1.5rem;
        color: #333;
        text-decoration: none;
    }

    .navigation button {
        margin-left: 10px;
        padding: 8px 16px;
        background: #f4f4f4;
        border: none;
        border-radius: 4px;
        cursor: pointer;
    }

    .navigation button:hover {
        background-color: #e2e2e2;
    }
</style>
