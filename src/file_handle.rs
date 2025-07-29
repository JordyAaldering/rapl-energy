use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::str::FromStr;
use std::sync::{Mutex, MutexGuard};

#[derive(Debug)]
pub struct FileHandle {
    handle: Mutex<File>,
}

impl FileHandle {
    pub fn new(path: &str, write: bool) -> io::Result<Self> {
        let file = OpenOptions::new().read(true).write(write).open(path)?;
        Ok(Self { handle: Mutex::new(file) })
    }

    pub fn read<T: FromStr>(&self) -> T where T::Err: std::fmt::Debug {
        // Open file
        let mut file = self.open(0);
        // Read content
        let mut buf = String::new();
        file.read_to_string(&mut buf).unwrap();
        // Parse buffer
        let buf = buf.trim();
        buf.parse::<T>().expect(&format!("Could not parse {}", buf))
    }

    pub fn write(&self, value: u64) {
        let mut file = self.open(0);
        write!(file, "{}", value).unwrap();
    }

    fn open(&self, offset: u64) -> MutexGuard<'_, File> {
        let mut file = self.handle.lock().unwrap();
        file.seek(SeekFrom::Start(offset)).unwrap();
        file
    }
}
