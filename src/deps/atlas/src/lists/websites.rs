
pub static IMAGE_EXTENSIONS: &'static [&str] = &[".jpg", "jpeg", ".gif", ".png", ".bmp", ".ocr", ".tiff"];
pub static DOWNLOADABLE_FILE_EXTENSIONS: &'static [&str] = &[
    ".zip", ".gz", ".tar", ".rar", ".7z", ".bz", ".raw", ".bin", ".deb",
    ".pdf", ".doc", ".docx", ".odt", ".xls", ".xlsx", ".ppt", ".pptx", ".ods", ".odp",
    ".avi", ".mp3", ".wav", 
    ".mp4", ".avi", ".mov", ".mpg", ".mpeg", ".webm", ".mkv", ".wma"
];


pub static WEBSITE_EXCLUDE_PATHS: &'static [&str] = &[
    "login",
    "register",
    "signup",
    "join",
    "contact",
    "forgot_password",
    "forgot",
    "lookup",
    "terms",
    "privacy",
    "docs/",
    "blog/",
    "404",
    "500",
    "error",
    "not_found"
];


