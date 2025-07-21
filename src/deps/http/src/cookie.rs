#[derive(Debug, Clone)]
pub struct Cookie {
    pub host: String,
    pub path: String,
    pub http_only: bool,
    pub secure: bool,
    pub expires: u64,
    pub name: String,
    pub value: String,
}

impl Cookie {
    /// Instantiate new cookie, used chained methods to specify other cookie variables.
    pub fn new(name: &str, value: &str) -> Self {
        Self {
            host: String::new(),
            path: "/".to_string(),
            http_only: false,
            secure: true,
            expires: 0_u64,
            name: name.to_string(),
            value: value.to_string(),
        }
    }

    /// Instantiate cookie from line of Netscape formatted cookies.txt file
    pub fn from_line(line: &str) -> Option<Self> {
        let parts: Vec<String> = line.trim().split('\t').map(|e| e.to_string()).collect();

        if parts.len() < 7 {
            return None;
        }

        // Set cookie
        Some(Self {
            host: parts[0].to_string(),
            path: parts[2].to_string(),
            http_only: parts[1].to_lowercase().as_str() != "false",
            secure: parts[3].to_lowercase().as_str() != "false",
            expires: parts[4].parse::<u64>().unwrap(),
            name: parts[5].to_string(),
            value: parts[6].to_string(),
        })
    }

    /// Format cookie as line to be saved within Netscape formatted cookies.txt file
    pub fn to_line(&self) -> String {
        // Get line
        let parts: Vec<String> = vec![
            self.host.clone(),
            if self.http_only {
                "TRUE".to_string()
            } else {
                "FALSE".to_string()
            },
            self.path.clone(),
            if self.secure {
                "TRUE".to_string()
            } else {
                "FALSE".to_string()
            },
            format!("{}", self.expires),
            self.name.clone(),
            self.value.clone(),
        ];

        parts.join("\t").to_string()
    }

    /// Set host
    pub fn host(mut self, host: &str) -> Self {
        self.host = host.to_string();
        self
    }

    /// Set path
    pub fn path(mut self, path: &str) -> Self {
        self.path = path.to_string();
        self
    }

    /// Set http-only
    pub fn http_only(mut self, http_only: bool) -> Self {
        self.http_only = http_only;
        self
    }

    /// Set secure
    pub fn secure(mut self, secure: bool) -> Self {
        self.secure = secure;
        self
    }

    /// Set expires
    pub fn expires(mut self, expires: u64) -> Self {
        self.expires = expires;
        self
    }
}
