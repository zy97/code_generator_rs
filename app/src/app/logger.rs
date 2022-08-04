use std::{io, sync::mpsc::Sender};

pub struct Logger {
    pub sender: Sender<u8>,
}
impl io::Write for Logger {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        for chr in buf {
            let op = self.sender.send(*chr);
            match op {
                Ok(_) => {
                    println!("send: {}", *chr);
                }
                Err(err) => {
                    println!("err11:{:?}", err);
                    return Ok(0);
                }
            }
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
