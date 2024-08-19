use std::path::Path;
use anyhow::Result;

pub struct MinistryDirectory{
    directory_root: String,
}

impl MinistryDirectory{
    pub fn new(directory_root: String) -> MinistryDirectory{
        MinistryDirectory{
            directory_root: directory_root,
        }
    }

    pub fn init(&self) -> Result<()>{
        println!("Initializing...");
        if self.exists()? {
            println!("This directory already contains a deck!");
            return Ok(());
        }
        else{
            return self.create();
        }
    }

    fn create(&self) -> Result<()>{
        println!("Creating a new deck...");

        // the default content.yml file
        let content_yml: &str = include_str!("content.yml");
        // write content_yml to the directory root
        let content_path = format!("{}/content.yml", self.directory_root);
        println!("... {}", content_path);
        std::fs::write(content_path, content_yml)?;

        // create the .ministry file
        let ministry_path = format!("{}/.ministry", self.directory_root);
        println!("... {}", ministry_path);
        std::fs::write(ministry_path, "")?;

        // create the assets directory
        let assets_path = format!("{}/assets", self.directory_root);
        println!("... {}", assets_path);
        std::fs::create_dir(assets_path)?;

        Ok(())
    }

    pub fn exists(&self) -> Result<bool>{
        // Check if the directory exists,
        //  if it does, check if it contains content.yml
        //  if it does, check if it contains .ministry
        //  if it does, check if it contains assets

        let path = Path::new(&self.directory_root);
        if !path.exists(){
            return Ok(false)
        }
        let directory_root = path.to_str().unwrap();
        let content_path_location = format!("{}/content.yml", directory_root);
        let content_path = Path::new(&content_path_location);
        if !content_path.exists(){
            return Ok(false)
        }
        let ministry_path_location = format!("{}/.ministry", directory_root);
        let ministry_path = Path::new(&ministry_path_location);
        if !ministry_path.exists(){
            return Ok(false)
        }
        let assets_path_location = format!("{}/assets", directory_root);
        let assets_path = Path::new(&assets_path_location);
        if !assets_path.exists(){
            return Ok(false)
        }

        Ok(true)
    }

    /*
    pub fn get_ministries(&self) -> Vec<Ministry>{
        let mut ministries: Vec<Ministry> = Vec::new();
        let paths = fs::read_dir(self.directory_root).unwrap();
        for path in paths {
            let path = path.unwrap().path();
            let path_str = path.to_str().unwrap();
            let ministry = Ministry::new(path_str);
            ministries.push(ministry);
        }
        ministries
    }
    */
}