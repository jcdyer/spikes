fn main() {

    let password = b"12345";
    let salt = b"himalayan";

    let hash = argon2::hash_encoded(password, salt, &Default::default()).unwrap();
    println!("Hash is: {:?}", hash);
    assert!(argon2::verify_encoded(&hash, password).unwrap_or(false));
    println!("Hello, world!");
}
