use std::path::Path;
use anyhow::{Result, anyhow};
use serde::Serialize;
use yaml_rust2::YamlLoader;
use image::{ImageReader, imageops};
use webp::Encoder;

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
    pub card_type: String,
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

    fn parse_card(doc: &yaml_rust2::Yaml, default_id: String) -> Card{
        let id = doc["id"].as_str().unwrap_or_else(|| &default_id).to_string();
        let mut card_type = doc["type"].as_str().unwrap_or_else(|| "").to_string();

        if card_type == "" {
            if doc["content"].as_str().is_some(){
                card_type = "markdown".to_string();
            }
            else if doc["image"].as_str().is_some(){
                card_type = "image".to_string();
            }
            else{
                // this is our defaultiest default
                card_type = "title".to_string();
            }
        }

        Card{
            id: id,
            card_type: card_type,
            content: doc["content"].as_str().map(|s| s.to_string()),
            image_url: doc["image"].as_str().map(|s| s.to_string()),
            title: doc["title"].as_str().map(|s| s.to_string()),
        }
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
            deck.push(
                MinistryDirectory::parse_card(&item, counter_string)
            );
            counter += 1;
        }

        Ok(deck)
    }

    pub fn get_asset_path(&self, asset_path: std::path::PathBuf) -> String{
        let asset_path = asset_path.to_str().unwrap_or_else(|| "");
        format!("{}/assets/{}", self.directory_root, asset_path)
    }

    pub async fn get_named_file(&self, asset_path: std::path::PathBuf, config: &crate::Config, file_directives: &crate::FileDirectives) -> Result<rocket::fs::NamedFile>{
        let asset_path: &str = &self.get_asset_path(asset_path);
        let filename = asset_path.split("/").last().unwrap_or_else(|| "");
        if filename.ends_with(".jpg") || filename.ends_with(".png") || filename.ends_with(".gif"){
            // so, we want to replace the file with a .webp
            // but what if two different projects both have `bee.jpg`?
            // let's do the conversion
            let metadata = self.get_metadata()?;
            let webp_filename = filename.replace(".jpg", ".webp")
                                     .replace(".png", ".webp")
                                     .replace(".gif", ".webp");
            let webp_filename = format!("{}_{}_{}_{}", metadata.author_slug, metadata.slug, file_directives.to_string(), webp_filename);
            let temp_directory = config.temporary_asset_directory.clone();
            let webp_path = format!("{}/{}", temp_directory, webp_filename);

            let lossless = filename.ends_with(".png");

            // if the file already exists, return it
            if !Path::new(&webp_path).exists(){
                println!("Converting {} to {}", asset_path, webp_path);
                let mut img = ImageReader::open(asset_path)?.decode()?;
                // create the temp directory if it doesn't exist

                let mut max_width = config.max_width;
                let w = img.width();
                if file_directives.wide.unwrap_or(false) {
                    max_width = w;
                }
                if file_directives.width.unwrap_or(0) > 0 {
                    max_width = file_directives.width.unwrap_or(0);
                }
                let h = img.height();
                let mut max_height = config.max_height;
                if file_directives.tall.unwrap_or(false) {
                    max_height = h;
                }
                if file_directives.height.unwrap_or(0) > 0 {
                    max_height = file_directives.height.unwrap_or(0);
                }

                if w > max_width && h > max_height {
                    if w > h {
                        img = image::DynamicImage::ImageRgba8(imageops::resize(
                            &img,
                            max_width,
                            (max_width as f64 * h as f64 / w as f64) as u32,
                            imageops::FilterType::Lanczos3,
                        ));
                    }
                    else{
                        img = image::DynamicImage::ImageRgba8(imageops::resize(
                            &img,
                            (max_height as f64 * w as f64 / h as f64) as u32,
                            max_height,
                            imageops::FilterType::Lanczos3,
                        ));
                    }
                }
                else if w > max_width {
                    img = image::DynamicImage::ImageRgba8(imageops::resize(
                        &img,
                        max_width,
                        (max_width as f64 * h as f64 / w as f64) as u32,
                        imageops::FilterType::Lanczos3,
                    ));
                }
                else if h > max_height {
                    img = image::DynamicImage::ImageRgba8(imageops::resize(
                        &img,
                        (max_height as f64 * w as f64 / h as f64) as u32,
                        max_height,
                        imageops::FilterType::Lanczos3,
                    ));
                }

                if file_directives.grayscale.unwrap_or(false) {
                    img = image::DynamicImage::ImageLumaA8(imageops::grayscale_alpha(&img));
                    img = image::DynamicImage::ImageRgba8(img.to_rgba8());
                }

                if !Path::new(&temp_directory).exists(){
                    std::fs::create_dir(temp_directory)?;
                }
                let enc = Encoder::from_image(&img).unwrap();
                if lossless{
                    std::fs::write(&webp_path, &*enc.encode_lossless())?;
                }
                else{
                    std::fs::write(&webp_path, &*enc.encode(config.webp_quality))?;
                }
            }
            else{
                println!("Using existing {}", webp_path);
            }
            let opened_file = rocket::fs::NamedFile::open(webp_path).await?;
            return Ok(opened_file);
        }
        else if asset_path.ends_with(".webp") ||
                asset_path.ends_with(".mp3") ||
                asset_path.ends_with(".mp4") ||
                asset_path.ends_with(".webm") ||
                asset_path.ends_with(".ogg"){
            let opened_file = rocket::fs::NamedFile::open(asset_path).await?;
            // if the file is a .jpg, a .png, or a .gif, replace it with a .webp
            //  also: there are some automatic effects that can be applied to the image
            //  like, for example, a blur effect

            Ok(opened_file)
        }
        else{
            // the file is not one of the approved types
            Err(anyhow!("File type not supported"))
        }
    }
}