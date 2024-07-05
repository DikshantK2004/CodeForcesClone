
<script lang="ts">
    import "$lib/auth_styles.css";
	import { error } from "@sveltejs/kit";
	import type { ActionData, PageData } from "./$types";

    export let form : ActionData;


 </script> 
<form class = "input-form" action="?/login" method="POST" >

    {#if form?.error}
        <p class="error">{form.error}</p>
    {/if}

    {#if form?.body}
        <p class="success">{form.body}</p>
    {/if}

    <h1 id="tex"> Login to HELL</h1>
    <input type= "text" name="email" class= "inputprops" placeholder="email"><br>
    <input type="password" name="password" class= "inputprops" placeholder="password"><br>
    <input type="submit"  value="Login" class="buttonprops"><br>
    <input type="button" value="Register Instead" class="buttonprops">
    
</form>
