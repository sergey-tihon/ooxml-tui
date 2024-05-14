use std::io;

pub struct App {
    pub file_path: String,
    pub entries: Vec<String>,
}

impl App {
    pub fn from_file(path: String) -> io::Result<Self> {
        let file = std::fs::File::open(path.clone())?;
        let archive = zip::ZipArchive::new(file)?;

        Ok(Self {
            file_path: path,
            entries: archive.file_names().map(|name| name.to_string()).collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::App;
    use std::io;

    #[test]
    fn load_pptx() -> io::Result<()> {
        let app = App::from_file("data/sample.pptx".to_string())?;
        app.entries.iter().for_each(|name| {
            println!("{}", name);
        });

        Ok(())
    }
}
