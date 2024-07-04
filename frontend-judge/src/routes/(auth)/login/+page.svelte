
<script lang="ts">
    import "$lib/auth_styles.css";

    let email: string;
    let password: string;


 </script> 
<form class = "input-form" method = "POST">
    <h1 id="tex"> Login to HELL</h1>
    <input type="text" name="email" bind:value={email} class= "inputprops" placeholder="email"><br>
    <input type="password" name="password" bind:value={password} class= "inputprops" placeholder="password"><br>
    <input type="submit"  value="Login" class="buttonprops"><br>
    <input type="button" value="Register Instead" class="buttonprops">
</form>

