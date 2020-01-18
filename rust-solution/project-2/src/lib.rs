#![deny(missing_docs)]

//! The kvs crate contains an in-memory Key-Value store (`kvs::KvStore`) and
//! a command line tool `kvs`.

use std::collections::HashMap;

use failure::Error;
use std::fs;


/// An in-memory Key-Value store where the Keys and Values are both Strings.
pub struct KvStore {
    kv: HashMap<String, String>,
    f: std::fs::File,
    recs: u64,
}

/// A result.
pub type Result<T> = std::result::Result<T, Error>;

/// A command.
#[derive(serde::Serialize, serde::Deserialize)]
enum LogCommand {
    Set(String, String),
    Rm(String)
}

#[derive(failure::Fail, Debug)]
#[fail(display = "Removed key not found")]
/// An error: key not found on remove.
pub struct KeyNotFoundOnRemove;

impl KvStore {

    /// Gets a value from the store if one exists, otherwise returns None.
    /// ```
    /// let temp_dir = tempfile::TempDir::new().unwrap();
    /// let mut store = kvs::KvStore::open(temp_dir.path()).unwrap();
    /// assert_eq!(store.get("key".to_owned()).unwrap(), None);
    /// store.set("key".to_owned(), "value".to_owned());
    /// assert_eq!(store.get("key".to_owned()).unwrap(), Some("value".to_owned()));
    /// ```
    pub fn get(&self, k: String) -> Result<Option<String>> {
        Ok(self.kv.get(&k).map(|s| s.to_owned()))
    }

    fn set_(&mut self, k: String, v: String) {
        self.kv.insert(k, v);
    }

    /// Stores a value in the store, overwriting existing values, if any.
    pub fn set(&mut self, k: String, v: String) -> Result<()> {
        self.set_(k.to_owned(), v.to_owned());
        self.append_(LogCommand::Set(k, v))?;
        Ok(())
    }

    fn remove_(&mut self, k: &String) -> Option<String> {
        self.kv.remove(k)
    }

    fn maybe_compact_(&mut self) -> Result<()> {
        use std::io::{Write, Seek};
        use std::convert::TryInto;
        if self.recs < (self.kv.len() * 3).try_into().unwrap() {
            return Ok(());
        }

        // Unsafe rewrite
        self.f.set_len(0)?;
        self.recs = 0;
        self.f.seek(std::io::SeekFrom::Start(0))?;
        for (key, val) in self.kv.iter() {
            let lc = LogCommand::Set(key.to_owned(), val.to_owned());
            let vec = serde_cbor::to_vec(&lc)?;
            let l: u64 = vec.len().try_into().unwrap();
            self.f.write_all(&l.to_be_bytes())?;
            self.f.write_all(&vec)?;
            self.recs += 1;
        }

        return Ok(());
    }

    /// Removes a value from the store.
    /// ```
    /// let temp_dir = tempfile::TempDir::new().unwrap();
    /// let mut store = kvs::KvStore::open(temp_dir.path()).unwrap();
    /// store.set("key".to_owned(), "value".to_owned());
    /// assert_eq!(store.get("key".to_owned()).unwrap(), Some("value".to_owned()));
    /// store.remove("key".to_owned());
    /// assert_eq!(store.get("key".to_owned()).unwrap(), None);
    /// ```
    pub fn remove(&mut self, k: String) -> Result<Option<String>> {
        let prev = self.remove_(&k);
        if prev == None {
            return Err(KeyNotFoundOnRemove{}.into());
        }
        self.append_(LogCommand::Rm(k))?;
        Ok(prev)
    }

    fn append_(&mut self, lc: LogCommand) -> Result<()> {
        use std::io::{Write, Seek};
        use std::convert::TryInto;
        self.f.seek(std::io::SeekFrom::End(0))?;
        let vec = serde_cbor::to_vec(&lc)?;
        let l: u64 = vec.len().try_into().unwrap();
        self.f.write_all(&l.to_be_bytes())?;
        self.f.write_all(&vec)?;
        self.recs += 1;
        self.maybe_compact_()?;
        Ok(())
    }

    ///
    pub fn open(filename: &std::path::Path) -> Result<KvStore> {
        let mut pb = filename.to_path_buf();
        pb.push("db");
        let mut f = fs::OpenOptions::new().read(true).write(true).create(true).open(pb)?;
        let mut kv = HashMap::new();
        let mut recs = 0;
        loop {
            use std::io::Read;
            let mut len_buffer = [0; 8];
            let res = f.read_exact(&mut len_buffer);
            match res {
                Err(s) => {
                    if s.kind() == std::io::ErrorKind::UnexpectedEof {
                        break;
                    }
                    return Err(s.into());
                },
                Ok(_) => {
                    let lim = f.by_ref().take(u64::from_be_bytes(len_buffer));
                    let lc = serde_cbor::from_reader(lim)?;
                    match lc {
                        LogCommand::Set(k, v) => {kv.insert(k, v);},
                        LogCommand::Rm(k) => {kv.remove(&k);},
                    }
                    recs += 1;
                }
            };
        }
        Ok(KvStore { 
            kv: kv,
            f: f,
            recs: recs,
        })
    }
}
