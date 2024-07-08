
<script lang="ts">
    import "$lib/auth_styles.css";
    import {goto} from "$app/navigation";
    import type { ActionData } from "./$types";
    import {enhance} from "$app/forms";
    import ErrorDialog from "$lib/error_dialog.svelte";
    
    export let form: ActionData;

 </script> 
<form class = "input-form" method = "POST" action="?/register">
    <h1 id="tex"> Register to HELL</h1>
    <input type= "text" name="name"  class= "inputprops" placeholder="full name"><br>
    <input type= "email" name="email"  class= "inputprops" placeholder="email"><br>
    <input type= "text" name="username"  class= "inputprops" placeholder="username"><br>
    <input type="password" name="password"  class= "inputprops" placeholder="password"><br>
    <input type="submit"  value="Register" class="buttonprops"><br>
    <input type="button" value="Login Instead" class="buttonprops" on:click={() => {goto("/login");}}>
</form>

<ErrorDialog htmlContent={form?.error} visible={form?.error !== undefined} message="" />

