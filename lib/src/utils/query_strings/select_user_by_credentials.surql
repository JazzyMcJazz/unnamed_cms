SELECT * FROM system_user WHERE 
    email = $email AND 
    crypto::argon2::compare(password, $password);