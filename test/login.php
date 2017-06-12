<?php
    $username = $_POST["bs_username"];
    $password = $_POST["bs_password"];
    $success = false;
    if($username == 'root' && $password = 'toor') {
        $success = true;
    }
    $data = array('username' => $username, 'password' => $password, 'success' => $success);
    header('Content-Type: application/json');
    echo json_encode($data, JSON_PRETTY_PRINT);
?>
