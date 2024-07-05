
<script lang="ts">
    import "$lib/auth_styles.css";

    let email: string;
    let password: string;


 </script> 
<form class = "input-form" method = "POST">
    <h1 id="tex"> Register to HELL</h1>
    <input type= "text" name="full-name" bind:value={email} class= "inputprops" placeholder="full name"><br>
    <input type= "text" name="email" bind:value={email} class= "inputprops" placeholder="email"><br>
    <input type= "text" name="username" bind:value={email} class= "inputprops" placeholder="username"><br>
    <input type="password" name="password" bind:value={password} class= "inputprops" placeholder="password"><br>
    <input type="submit"  value="Register" class="buttonprops"><br>
    <input type="button" value="Login Instead" class="buttonprops">
</form>

