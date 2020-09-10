use std::io::{self, Read};
use std::process::{Child, ChildStdout};



pub(crate) struct CatFileReader {
    pub(crate) child:  Child,
    pub(crate) stdout: ChildStdout,
}

impl Read for CatFileReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let read = self.stdout.read(buf)?;
        if read != 0 { return Ok(read); }

        let exit = self.child.wait()?;
        match exit.code() {
            Some(0) => Ok(0),
            Some(_) => Err(io::Error::new(io::ErrorKind::Other, "git cat-file exited non-zero")),
            None    => Err(io::Error::new(io::ErrorKind::Other, "git cat-file died by signal")),
        }
    }
}
