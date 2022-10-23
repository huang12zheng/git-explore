use crate::*;
pub trait ToPath {}

impl ToPath for String {}

pub trait PathEx {
    fn to_config_path(&self) -> PathBuf;
    fn to_path(&self) -> PathBuf;
    fn get_content(&self) -> String;
}

pub trait ToStringVec {
    fn to_string_vec(&self) -> Vec<String>;
}
fn to_string_vec(path_vec: &[PathBuf]) -> Vec<String> {
    path_vec
        .iter()
        .map(|e| e.to_str().unwrap().to_owned())
        .collect::<_>()
}

impl ToStringVec for Vec<PathBuf> {
    fn to_string_vec(&self) -> Vec<String> {
        to_string_vec(self)
    }
}

impl PathEx for &str {
    fn to_config_path(&self) -> PathBuf {
        Path::new(self).join(KEY_CONFIG_PATH)
    }
    fn to_path(&self) -> PathBuf {
        Path::new(self)
            .canonicalize()
            .with_context(|| format!("{:?}", self))
            .unwrap()
    }
    fn get_content(&self) -> String {
        self.to_path().get_content()
    }
}
impl PathEx for String {
    fn to_config_path(&self) -> PathBuf {
        self.as_str().to_config_path()
    }
    fn to_path(&self) -> PathBuf {
        self.as_str().to_path()
    }
    fn get_content(&self) -> String {
        self.to_path().get_content()
    }
}
impl PathEx for PathBuf {
    fn to_config_path(&self) -> PathBuf {
        self.to_str().unwrap().to_config_path()
    }
    fn to_path(&self) -> PathBuf {
        self.to_str().unwrap().to_path()
    }
    fn get_content(&self) -> String {
        String::from_utf8(
            std::fs::read(self)
                .with_context(|| format!("{:?}", self.to_str()))
                .unwrap(),
        )
        .unwrap()
    }
}
