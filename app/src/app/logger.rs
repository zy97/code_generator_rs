use std::{io, sync::mpsc::Sender};

pub struct Logger {
    pub sender: Sender<u8>,
}
impl io::Write for Logger {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        for chr in buf {
            let op = self.sender.send(*chr);
            match op {
                Ok(_) => {}
                Err(err) => {
                    println!("err:{:?}", err);
                    return Ok(0);
                }
            }
        }
        // for c in "\r\n".bytes() {
        //     self.sender.send(c);
        // }
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
