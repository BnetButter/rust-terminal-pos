use std::io;
use termios::{ self };
use termios::os::target::{ TCSANOW };

struct Termios {
    termios: termios::Termios,
}

impl Termios {
    fn new() -> Termios
    {
        let cur_term = Termios { termios: termios::Termios::from_fd(0).unwrap() };
        let new_term = termios::Termios::from_fd(0).unwrap();
        termios::tcsetattr(0, TCSANOW, &new_term).unwrap();

        println!("Set new termios");
        return cur_term
    }
}

impl Drop for Termios {
    fn drop(&mut self)
    {
        println!("Restoring termios");
        termios::tcsetattr(0, TCSANOW, &self.termios).unwrap();
    }
}



fn main() {
    let _ = Termios::new();   
}

