use std::path::Path;
use anyhow::{Result, anyhow};
use serde::Serialize;
use yaml_rust2::YamlLoader;

use slugify::slugify;

#[derive(Debug, Serialize)]
pub struct DeckMetadata{
    // title & author are non-optional
    pub title: String,
    pub slug: String,
    pub author: String,
    pub author_slug: String,
    pub author_link: Option<String>,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub locale: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct Card{
    pub id: String,
    pub content: Option<String>,
    pub image_url: Option<String>,
    pub title: Option<String>,
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
        if name_or_title == "" {
            return Err(anyhow!("No name or title found: this is a mandatory field"));
        }
        let author = doc["author"].as_str().unwrap_or_else(|| "");
        if author == "" {
            return Err(anyhow!("No author found: this is a mandatory field"));
        }

        let author_link = match doc["author_link"].as_str(){
            Some(author_link) => Some(author_link.to_string()),
            None => None,
        };
        let description = match doc["description"].as_str(){
            Some(description) => Some(description.to_string()),
            None => None,
        };
        let image_url = match doc["image"].as_str(){
            Some(image_url) => {
                // test for the existence of image_url as a file
                let image_path = format!("{}/assets/{}", self.directory_root, image_url);
                if Path::new(&image_path).exists() {
                    println!("Image exists: {}", image_path);
                    Some(image_url.to_string())
                }
                else{
                    println!("Image does not exist: {}", image_path);
                    None
                }
            }
            None => None,
        };
        let locale = match doc["locale"].as_str(){
            Some(locale) => Some(locale.to_string()),
            None => None,
        };

        let dm = DeckMetadata{
            title: name_or_title.to_string(),
            slug: slugify!(name_or_title),
            author: author.to_string(),
            author_slug: slugify!(author),
            author_link: author_link,
            description: description,
            image_url: image_url,
            locale: locale,
        };
        // Pretty print
        println!("{:#?}", dm);
        Ok(dm)
    }

    pub fn compile_assets(&self) -> Result<()>{
        // target directory is:
        // /build/assets/{author_slug}/{slug}

        let deck_metadata = self.get_metadata()?;
        let target_directory = format!("./build/assets/{}/{}/", deck_metadata.author_slug, deck_metadata.slug);
        let source_directory = format!("{}/assets", self.directory_root);
        println!("Copying assets from {} to {}", source_directory, target_directory);

        Ok(())
    }

    pub fn get_deck(&self) -> Result<Vec<Card>>{
        // what's an Index?
        let content_string = self._get_content()?;
        let yaml = YamlLoader::load_from_str(&content_string)?;
        let doc = &yaml[0];
        let list = match doc["pages"].as_vec() {
            Some(list) => list,
            None => return Err(anyhow!("No content found")),
        };

        let mut deck = Vec::new();
        let mut counter = 0;
        for item in list {
            let counter_string = counter.to_string();
            deck.push(Card{
                id: item["id"].as_str().unwrap_or_else(|| counter_string.as_str()).to_string(),
                content: item["content"].as_str().map(|s| s.to_string()),
                image_url: item["image"].as_str().map(|s| s.to_string()),
                title: item["title"].as_str().map(|s| s.to_string()),
            });
            counter += 1;
        }

        Ok(deck)
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