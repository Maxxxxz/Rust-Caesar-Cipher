use std::io;

pub struct Cipher
{
    pub msg: String,
    pub shift: i32
}

const Ualpha: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const Lalpha: &str = "abcdefghijklmnopqrstuvwxyz";

impl Cipher
{
    pub fn new(shift: i32) -> Cipher
    {
        let _shift = shift % (Ualpha.len() + Lalpha.len()) as i32;
        return Cipher {msg: String::new(), shift: _shift};
    }

    pub fn encrypt(&mut self)
    {
        let mut e: String = String::new();
        for c in self.msg.chars()
        {
            if c.is_uppercase()
            {
                match Ualpha.chars().position(|_c| c == _c)
                {
                    Some(offset) => 
                    {
                        let t = (offset as i32 + self.shift) % Ualpha.len() as i32;
                        e.push(Ualpha.chars().nth(t as usize).unwrap());
                    },
                    None => panic!("Character {} not in alphabet!", c),
                }
            }
            else if c.is_lowercase()
            {
                match Lalpha.chars().position(|_c| c == _c)
                {
                    Some(offset) => 
                    {
                        let t = (offset as i32 + self.shift) % Ualpha.len() as i32;
                        e.push(Lalpha.chars().nth(t as usize).unwrap());
                    },
                    None => panic!("what"),
                }
            }
            else    // c must be special char
            {
                e.push(c);
            }
        }
        self.msg = e;
    }
    
    pub fn decrypt(&mut self)
    {
        self.shift = -self.shift;   // flip shift
        self.encrypt();
        self.shift = -self.shift;   // flip shift back to original
    }

    pub fn inputMsg(&mut self)
    {
        let mut inp = String::new();
        io::stdin().read_line(&mut inp).expect("Failed to get input!");
        self.msg = inp[..inp.len()-1].to_string();
        if self.msg.ends_with('\r') // Catch windows \r\n
        {
            self.msg = self.msg[..self.msg.len()-1].to_string();
        }
    }

    pub fn getMsg(&mut self) -> String
    {
        return self.msg.clone();
    }
}

#[test]
fn test_encrypt()
{
    let mut enc = Cipher::new(3);
    enc.msg = "Hello, World!".to_string();
    enc.encrypt();
    assert_eq!("Khoor, Zruog!".to_string(), enc.getMsg());
}

#[test]
fn test_decrypt()
{
    let mut enc = Cipher::new(3);
    enc.msg = "Khoor, Zruog!".to_string();
    enc.decrypt();
    assert_eq!("Hello, World!".to_string(), enc.getMsg());
}