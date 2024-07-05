
<script lang="ts">
    import "$lib/auth_styles.css";
    import {goto} from "$app/navigation";
    import {enhance} from "$app/forms";

    let gotoLogin = () =>{
        goto("/login");
    };

 </script> 
<form class = "input-form" method = "POST">
    <h1 id="tex"> Register to HELL</h1>
    <input type= "text" name="full-name"  class= "inputprops" placeholder="full name"><br>
    <input type= "text" name="email"  class= "inputprops" placeholder="email"><br>
    <input type= "text" name="username"  class= "inputprops" placeholder="username"><br>
    <input type="password" name="password"  class= "inputprops" placeholder="password"><br>
    <input type="submit"  value="Register" class="buttonprops"><br>
    <input type="button" value="Login Instead" class="buttonprops" on:click={gotoLogin}>
</form>

