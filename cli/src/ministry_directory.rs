use std::path::Path;
use anyhow::Result;
use yaml_rust2::YamlLoader;

use slugify::slugify;

#[derive(Debug)]
pub struct DeckMetadata{
    pub title: String,
    pub slug: String,
    pub author: String,
    pub author_slug: String,
    pub description: String,
    pub image_url: String,
    pub locale: String,
}

pub struct Card{
    id: String,
}

pub struct MinistryDirectory{
    directory_root: String,
}

impl MinistryDirectory{
    pub fn new(directory_root: String) -> MinistryDirectory{
        MinistryDirectory{
            directory_root: directory_root,
        }
    }

    pub fn init(&self, force: bool) -> Result<()>{
        if self.exists()? {
            println!("This directory already contains a deck!");
            if force {
                println!("Forcing re-initialization...");
                return self.create();
            }
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
        println!("✅ {}", content_path);
        std::fs::write(content_path, content_yml)?;

        // create the .ministry file
        let ministry_path = format!("{}/.ministry", self.directory_root);
        println!("✅ {}", ministry_path);
        std::fs::write(ministry_path, "")?;

        // create the assets directory
        let assets_path = format!("{}/assets", self.directory_root);
        println!("✅ {}", assets_path);
        if Path::new(&assets_path).exists(){
        }
        else{
            std::fs::create_dir(assets_path)?;
        }

        // the default bee.jpg file
        let content_bee = include_bytes!("bee.jpg");
        // write content_yml to the directory root
        let bee_path = format!("{}/assets/bee.jpg", self.directory_root);
        println!("✅ {}", bee_path);
        std::fs::write(bee_path, content_bee)?;

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

    pub fn _get_content(&self) -> Result<String>{
        let content_path = format!("{}/content.yml", self.directory_root);
        let content = std::fs::read_to_string(content_path)?;
        Ok(content)
    }

    pub fn get_metadata(&self) -> Result<DeckMetadata>{
        // what's a DeckMetadata?
        let content_string = self._get_content()?;
        let yaml = YamlLoader::load_from_str(&content_string)?;
        let doc = &yaml[0];

        let name = doc["name"].as_str().unwrap_or_else(|| "");
        let title = doc["title"].as_str().unwrap_or_else(|| "");
        let name_or_title = if name == "" { title } else { name };
        let author = doc["author"].as_str().unwrap_or_else(|| "");

        let description = doc["description"].as_str().unwrap_or_else(|| "");
        let image_url = doc["image"].as_str().unwrap_or_else(|| "");
        let locale = doc["locale"].as_str().unwrap_or_else(|| "");

        // test for the existence of image_url as a file
        let image_path = format!("{}/assets/{}", self.directory_root, image_url);
        if Path::new(&image_path).exists() {
            println!("Image exists: {}", image_path);
        }
        else{
            println!("Image does not exist: {}", image_path);
        }

        let dm = DeckMetadata{
            title: name_or_title.to_string(),
            slug: slugify!(name_or_title),
            author: author.to_string(),
            author_slug: slugify!(author),
            description: description.to_string(),
            image_url: image_url.to_string(),
            locale: locale.to_string(),
        };
        // Pretty print
        println!("{:#?}", dm);
        Ok(dm)
    }

    /*
    pub fn get_index(&self) -> Result<Index>{
        // what's an Index?
    }
    */

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