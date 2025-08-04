use crate::CLI;
use crate::str;
use super::EnvironmentVariable;
use super::encode::EncodeAction;
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::path::PathBuf;
use fs2::FileExt;
use std::fs::OpenOptions;
use std::fs;
use std::io;
use std::io::{BufReader, BufWriter, ErrorKind, Read, Write};
use std::slice::Iter;
use std::sync::{OnceLock, RwLock, RwLockReadGuard, RwLockWriteGuard};
use regex::Regex;

impl EnvironmentVariable {
    pub(crate) fn delete(&self) -> Result<(), WriterError> {
        if let Some(position) = self.position()? {
            WriteManager::get_mut()?.delete(position);
        }
        Ok(())
    }

    pub(crate) fn append(&self) -> Result<(), WriterError> {
        let encode_action = if CLI.args.as_ref().unwrap().unset {
            EncodeAction::Unset
        } else if CLI.args.as_ref().unwrap().unexport {
            EncodeAction::Unexport
        } else {
            EncodeAction::Default
        };
        let line = self.encode(encode_action);
        
        WriteManager::get_mut()?.append(line);
        
        Ok(())
    }
    
    pub(crate) fn get_line(&self) -> Result<Option<String>, WriterError> {
        Ok(
            if let Some(position) = self.position()? {
                Some(WriteManager::get()?.get_line(position))
            } else {
                None
            }
        )
    }

    fn position(&self) -> Result<Option<usize>, WriterError> {
        let var_line = Regex::new(
            &format!("{}{}(?:[ =]|$)", Self::LINE_PATTERN, &self.name)
        ).expect("regex pattern exceeded max size or is invalid");

        let write_manager = WriteManager::get()?;
        Ok(write_manager
            .iter()
            .position(|line| {
                var_line.is_match(line)
            })
        )
    }
}

pub(crate) struct WriteManager {
    location: PathBuf,
    lines: Vec<String>,
    append: Vec<String>,
    rewrite: bool,
}

impl WriteManager {
    fn new() -> Result<Self, WriterError> {
        let location = CLI.config_home.join("gexport");
        fs::create_dir_all(&location)?;
        
        let location = location.join("gexports");
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .append(true)
            .open(&location)?;
        
        let lines;
        FileExt::lock_shared(&file)?;
        {
            let mut reader = BufReader::new(&file);
            let mut data = Vec::new();
            reader.read_to_end(&mut data)?;
            lines = String::from_utf8_lossy(&data)
                .lines()
                .map(str::to_string)
                .collect::<Vec<_>>();
        }
        FileExt::unlock(&file)?;
        
        Ok(Self {
            location,
            lines,
            append: Vec::new(),
            rewrite: false,
        })
    }
    
    fn get_mut() -> Result<RwLockWriteGuard<'static, Self>, WriterError> {
        Ok(Self::get_lock()?.write().unwrap())
    }
    
    pub(crate) fn get() -> Result<RwLockReadGuard<'static, Self>, WriterError> {
        Ok(Self::get_lock()?.read().unwrap())
    }
    
    fn get_lock() -> Result<&'static RwLock<Self>, WriterError> {
        static WRITE_MANAGER: OnceLock<RwLock<WriteManager>> = OnceLock::new();
        let write_manager = WRITE_MANAGER.get();
        
        let manager;
        if let Some(write_manager) = write_manager {
            manager = write_manager;
        } else {
            let write_manager = Self::new()?;
            manager = WRITE_MANAGER.get_or_init(|| RwLock::new(write_manager));
        }
        
        Ok(manager)
    }
    
    fn get_line(&self, index: usize) -> String {
        self.lines[index].clone()
    }
    
    fn delete(&mut self, index: usize) {
        self.lines.remove(index);
        self.rewrite = true;
    }
    
    fn append(&mut self, item: String) {
        self.append.push(item);
    }
    
    pub(crate) fn finalize() -> Result<(), WriterError> {
        let this = Self::get()?;
        
        let file = if !this.rewrite {
            OpenOptions::new()
                .append(true)
                .open(&this.location)?
        } else {
            OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(&this.location)?
        };
        
        FileExt::lock_exclusive(&file)?;
        {
            let mut writer = BufWriter::new(&file);

            if this.rewrite {
                for line in &this.lines {
                    writeln!(writer, "{}", line)?;
                }
            }

            for line in &this.append {
                writeln!(writer, "{}", line)?;
            }
        }
        FileExt::unlock(&file)?;
        
        Ok(())
    }
    
    pub(crate) fn import(import: Vec<u8>) -> Result<(), WriterError> {
        let lines = String::from_utf8_lossy(&import)
            .lines()
            .map(str::to_string)
            .collect::<Vec<_>>();

        EnvironmentVariable::from_lines(&lines)
            .iter()
            .try_for_each(EnvironmentVariable::delete)?;

        let mut this = Self::get_mut()?;

        lines.into_iter()
            .for_each(|line| this.append(line));

        Ok(())
    }

    pub(crate) fn clear() -> Result<(), WriterError> {
        let mut this = Self::get_mut()?;
        this.lines.clear();
        this.rewrite = true;
        Ok(())
    }
    
    pub(crate) fn iter(&self) -> Iter<String> {
        self.lines.iter()
    }
}

#[derive(Debug)]
pub(crate) struct WriterError {
    cause: ErrorKind,
}

impl From<io::Error> for WriterError {
    fn from(error: io::Error) -> Self {
        Self {
            cause: error.kind()
        }
    }
}

impl Display for WriterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error = match self.cause {
            ErrorKind::PermissionDenied => str!("permission denied"),
            ErrorKind::ReadOnlyFilesystem => str!("read-only filesystem"),
            ErrorKind::ResourceBusy => str!("resource busy"),
            error => format!("{error:?}")
        };
        write!(f, "cannot write updated state, {error}")
    }
}

impl Error for WriterError {}