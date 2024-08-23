use super::wallets;

#[test]
fn test_generate_keypair() {
    let keypair = wallets::KeyPair::generate(42);
    assert_eq!(keypair.public_key.serialize().len(), 33);
    assert_eq!(keypair.secret_key[..].len(), 32);

    println!("secret key: {}", &keypair.secret_key.to_string());
    println!("public key: {}", &keypair.public_key.to_string());
}