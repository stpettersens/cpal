<!DOCTYPE html>
<?php
    echo "<!-- Using PHP version " . phpversion() . " -->\n";
?>
<html>
    <head>
        <title>CPAL Test Page</title>
    </head>
    <body>
        <h4>CPAL Test Page</h4>
        <hr>
        <form action="submitted.php" method="post">
            <label for="bs_username">Username:</label>
            <input id="bs_username" name="bs_username" type="text">
            <br><br>
            <label for="ps_password">Password:</label>
            <input id="ps_password" name="bs_password" type="password">
            <br><br>
            <input type="submit" value="Log in">
        </form>
    </body>
</html>
