use std::collections::HashMap;
use uuid::Uuid;
use std::net::TcpStream;
use tokio::runtime::Runtime;
use std::sync::{Arc, Mutex};
use std::io::{Read, Write, Error as IoError};
use std::path::{Path, PathBuf};
use std::fs;
use std::time::SystemTime;
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use tokio::time::{interval, Duration};

// Message types for different commands
#[derive(Serialize, Deserialize, Debug)]
enum CfxMessage {
    ListDirectory { path: String },
    DownloadFile { path: String },
    UploadFile { path: String, content: Vec<u8> },
    DeleteFile { path: String },
    RenameFile { old_path: String, new_path: String },
    GetFileInfo { path: String },
}

#[derive(Serialize, Deserialize, Debug)]
struct FileInfo {
    size: u64,
    mime_type: String,
    sha256: String,
    last_modified: SystemTime,
}

#[derive(Debug)]
enum CfxError {
    IoError(IoError),
    InvalidPath,
    FileNotFound,
    PermissionDenied,
}

pub struct CfxServer {
    conn_id: i16,
    pool: Arc<Mutex<HashMap<i16, AuthConnection>>>,
    rt: Runtime,
    base_directory: PathBuf,
}

impl CfxServer {
    pub fn new(base_directory: PathBuf) -> Self {
        Self {
            conn_id: 0,
            pool: Arc::new(Mutex::new(HashMap::new())),
            rt: Runtime::new().unwrap(),
            base_directory,
        }
    }

    /// Check if stream has correct header bytes to be a CFX message
    pub fn is_cfx_stream(&self, stream: &mut TcpStream) -> bool {
        let mut buf = [0u8; 3];
        match stream.peek(&mut buf) {
            Ok(_) => buf[0] == 0x43 && buf[1] == 0x46 && buf[2] == 0x58,
            Err(_) => false,
        }
    }

    /// Add authenticated connection to pool
    pub fn add_connection(&mut self, uuid: &Uuid, conn: Connection) {
        let auth_conn = AuthConnection::new(uuid, conn);
        self.conn_id += 1;
        self.pool.lock().unwrap().insert(self.conn_id, auth_conn);
    }

    /// Start the message processing loop
    pub fn start_processing(&self) {
        let pool = Arc::clone(&self.pool);
        
        self.rt.spawn(async move {
            let mut interval = interval(Duration::from_secs(2));
            
            loop {
                interval.tick().await;
                let mut pool = pool.lock().unwrap();
                
                for (_, conn) in pool.iter_mut() {
                    if let Some(msg) = conn.check_messages() {
                        match msg {
                            CfxMessage::ListDirectory { path } => {
                                let _ = conn.handle_list_directory(&path);
                            },
                            CfxMessage::DownloadFile { path } => {
                                let _ = conn.handle_download_file(&path);
                            },
                            CfxMessage::UploadFile { path, content } => {
                                let _ = conn.handle_upload_file(&path, &content);
                            },
                            CfxMessage::DeleteFile { path } => {
                                let _ = conn.handle_delete_file(&path);
                            },
                            CfxMessage::RenameFile { old_path, new_path } => {
                                let _ = conn.handle_rename_file(&old_path, &new_path);
                            },
                            CfxMessage::GetFileInfo { path } => {
                                let _ = conn.handle_get_file_info(&path);
                            },
                        }
                    }
                }
            }
        });
    }
}

impl AuthConnection {
    fn check_messages(&mut self) -> Option<CfxMessage> {
        // Read message from connection
        let mut header = [0u8; 4]; // Message length prefix
        if let Ok(_) = self.conn.stream.read_exact(&mut header) {
            let length = u32::from_be_bytes(header) as usize;
            let mut buffer = vec![0u8; length];
            
            if let Ok(_) = self.conn.stream.read_exact(&mut buffer) {
                return serde_json::from_slice(&buffer).ok();
            }
        }
        None
    }

    fn handle_list_directory(&mut self, path: &str) -> Result<(), CfxError> {
        let full_path = self.validate_path(path)?;
        let entries = fs::read_dir(full_path)?
            .filter_map(|e| e.ok())
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .collect::<Vec<_>>();
            
        self.send_response(&entries)
    }

    fn handle_download_file(&mut self, path: &str) -> Result<(), CfxError> {
        let full_path = self.validate_path(path)?;
        let content = fs::read(full_path)?;
        self.send_response(&content)
    }

    fn handle_upload_file(&mut self, path: &str, content: &[u8]) -> Result<(), CfxError> {
        let full_path = self.validate_path(path)?;
        fs::write(full_path, content)?;
        self.send_response(&true)
    }

    fn handle_delete_file(&mut self, path: &str) -> Result<(), CfxError> {
        let full_path = self.validate_path(path)?;
        fs::remove_file(full_path)?;
        self.send_response(&true)
    }

    fn handle_rename_file(&mut self, old_path: &str, new_path: &str) -> Result<(), CfxError> {
        let old_full_path = self.validate_path(old_path)?;
        let new_full_path = self.validate_path(new_path)?;
        fs::rename(old_full_path, new_full_path)?;
        self.send_response(&true)
    }

    fn handle_get_file_info(&mut self, path: &str) -> Result<(), CfxError> {
        let full_path = self.validate_path(path)?;
        let metadata = fs::metadata(&full_path)?;
        
        let mut hasher = Sha256::new();
        let content = fs::read(&full_path)?;
        hasher.update(&content);
        
        let file_info = FileInfo {
            size: metadata.len(),
            mime_type: mime_guess::from_path(&full_path)
                .first_or_octet_stream()
                .to_string(),
            sha256: format!("{:x}", hasher.finalize()),
            last_modified: metadata.modified()?,
        };
        
        self.send_response(&file_info)
    }

    fn validate_path(&self, path: &str) -> Result<PathBuf, CfxError> {
        let full_path = self.base_directory.join(path);
        
        if !full_path.starts_with(&self.base_directory) {
            return Err(CfxError::PermissionDenied);
        }
        
        Ok(full_path)
    }

    fn send_response<T: Serialize>(&mut self, response: &T) -> Result<(), CfxError> {
        let serialized = serde_json::to_vec(response)
            .map_err(|_| CfxError::IoError(IoError::new(std::io::ErrorKind::Other, "Serialization failed")))?;
            
        let length = (serialized.len() as u32).to_be_bytes();
        self.conn.stream.write_all(&length)?;
        self.conn.stream.write_all(&serialized)?;
        self.conn.stream.flush()?;
        
        Ok(())
    }
}

impl From<IoError> for CfxError {
    fn from(error: IoError) -> Self {
        CfxError::IoError(error)
    }
}

