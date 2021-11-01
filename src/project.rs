pub enum ProjectType {
    Binary,
    Library,
    DynamicLibrary,
    Unknown,
}

impl ProjectType {
    pub fn from_string(s: &str) -> Self {
        match s {
            "bin" | "binary" => ProjectType::Binary,
            "lib" | "library" => ProjectType::Library,
            "dylib" | "dynamic_library" => ProjectType::DynamicLibrary,
            _ => ProjectType::Unknown,
        }
    }

    pub fn to_str(&self) -> String {
        match self {
            ProjectType::Binary => "bin".to_string(),
            ProjectType::Library => "lib".to_string(),
            ProjectType::DynamicLibrary => "dylib".to_string(),
            ProjectType::Unknown => "unknown".to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            ProjectType::Binary => "binary".to_string(),
            ProjectType::Library => "library".to_string(),
            ProjectType::DynamicLibrary => "dynamic library".to_string(),
            ProjectType::Unknown => "unknown".to_string(),
        }
    }
}

pub struct Project {}

impl Project {
    pub fn new(proj_name: &str, proj_type: &ProjectType) -> Result<(), String> {
        match proj_type {
            ProjectType::Unknown => {
                return Err(
                    "invalid project type specified\n\nvalid project types are: (bin, lib, dylib)"
                        .to_string(),
                )
            }
            _ => {}
        }

        let dir = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
            + "/"
            + proj_name;

        match std::fs::create_dir(&dir) {
            Ok(_) => {}
            Err(_) => return Err(format!(
                "destination `{}` already exists\n\nUse `crane --init` to initialize the directory",
                &dir
            )),
        }

        Ok(())
    }
}
