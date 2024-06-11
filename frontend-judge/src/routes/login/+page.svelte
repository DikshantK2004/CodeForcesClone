<script>
    import { goto } from '$app/navigation';

    let email = '';
    let password = '';
    let errorMessage = '';

    async function login() {
        const res = await fetch('/login', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ email, password })
        });

        if (res.ok) {
            goto('/dashboard');  // Redirect to dashboard on successful login
        } else {
            const errorData = await res.json();
            errorMessage = errorData.message || 'Invalid login credentials';
        }
    }
</script>

<div class="login-container">
    <form on:submit|preventDefault={login} class="login-form">
        <div class="input-group">
            <label for="email">Email</label>
            <input id="email" type="email" bind:value={email} required placeholder="Enter your email">
        </div>
        <div class="input-group">
            <label for="password">Password</label>
            <input id="password" type="password" bind:value={password} required placeholder="Enter your password">
        </div>
        <button type="submit" class="login-button">Login</button>
        {#if errorMessage}
            <p class="error">{errorMessage}</p>
        {/if}
    </form>
</div>

<style>
    .login-container {
        display: flex;
        align-items: 30%;
        padding-top: 20%;
        justify-content: center;
        
    }

    .login-form {
        padding: 2em;
        max-width: var(--column-width);
        width: auto;
        background-color: #F1E5D150;
        box-shadow: 0 4px 8px var(--color-shadow);
        
        border-radius: 10px;
        display: flex;
        flex-direction: column;
        gap: 1.5rem;
    }

    .input-group label {
        font-size: 1rem;
        color: var(--color-text);
    }

    .input-group input {
        padding: 0.8rem;
        border: 2px solid var(--color-border);
        border-radius: 5px;
        font-size: 0.9rem;
        background: white;
        color: var(--color-text);
    }

    .input-group input:focus {
        border-color: var(--color-theme-1);
        outline: none;
    }

    .login-button {
        padding: 1rem;
        background-color: var(--color-theme-1);
        color: white;
        border: none;
        border-radius: 5px;
        cursor: pointer;
        font-weight: bold;
        transition: background-color 0.3s;
    }

    .login-button:hover {
        background-color: var(--color-theme-2);
    }

    .error {
        color: var(--color-theme-1);
        text-align: center;
    }
</style>
