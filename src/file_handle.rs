use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom};
use std::str::FromStr;
use std::sync::{Mutex, MutexGuard};

pub struct FileHandle {
    handle: Mutex<File>,
}

impl FileHandle {
    pub fn new(path: &str) -> io::Result<Self> {
        let file = OpenOptions::new().read(true).open(path)?;
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

    pub fn from_le_bytes(&self, offset: u64) -> u64 {
        // Open file
        let mut file = self.open(offset);
        // Read content
        let mut buf = [0; std::mem::size_of::<u64>()];
        file.read_exact(&mut buf).unwrap();
        // Parse buffer
        u64::from_le_bytes(buf)
    }

    fn open(&self, offset: u64) -> MutexGuard<'_, File> {
        let mut file = self.handle.lock().unwrap();
        file.seek(SeekFrom::Start(offset)).unwrap();
        file
    }
}
