mod cipher;
use cipher::Cipher;

fn main()
{
    println!("Enter string to encrypt: ");
    let mut enc = Cipher::new(3);
    enc.inputMsg();
    enc.encrypt();
    println!("Encrypted: {}", enc.getMsg());
    enc.decrypt();
    println!("Decrypted: {}", enc.getMsg());
}