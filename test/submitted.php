<?php
    $username = $_POST["bs_username"];
    $password = $_POST["bs_password"];
    $data = array('username' => $username, 'password' => $password);
    header('Content-Type: application/json');
    echo json_encode($data, JSON_PRETTY_PRINT);
?>
