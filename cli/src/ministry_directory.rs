use std::path::{Path, PathBuf};
use anyhow::{Result, anyhow};
use serde::Serialize;
use yaml_rust2::YamlLoader;
use image::{ImageReader, imageops, DynamicImage, ImageBuffer};
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
    pub extra_header: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct Card{
    pub id: String,
    pub card_type: String,
    pub content: Option<String>,
    pub image_url: Option<String>,
    pub extra_class: Vec<String>,
    pub video_url: Option<String>,
    pub video_has_sound: bool,
    pub video_loop: bool,
    pub video_controls: bool,
    pub title: Option<String>,
    pub pngs: Vec<String>,
    pub pngs_fps: Option<i64>,
    pub pngs_loop: Option<bool>,
    pub fade_in: Option<i64>,
    pub stack: Vec<Card>
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
        if self.exists() {
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
        let content_path = PathBuf::from(&self.directory_root).join("content.yml");
        println!("✅ {}", content_path.to_str().unwrap_or_else(|| ""));
        std::fs::write(content_path, content_yml)?;

        // create the .ministry file
        let ministry_path = PathBuf::from(&self.directory_root).join(".ministry");
        println!("✅ {}", ministry_path.to_str().unwrap_or_else(|| ""));
        std::fs::write(ministry_path, "")?;

        // create the assets directory
        let assets_path = PathBuf::from(&self.directory_root).join("assets");
        println!("✅ {}", assets_path.to_str().unwrap_or_else(|| ""));
        if Path::new(&assets_path).exists(){
        }
        else{
            std::fs::create_dir(assets_path)?;
        }

        // the default bee.jpg file
        let content_bee = include_bytes!("bee.jpg");
        // write content_yml to the directory root
        let bee_path = PathBuf::from(&self.directory_root).join("assets/bee.jpg");
        println!("✅ {}", bee_path.to_str().unwrap_or_else(|| ""));
        std::fs::write(bee_path, content_bee)?;

        Ok(())
    }

    pub fn exists(&self) -> bool{
        // Check if the directory exists,
        //  if it does, check if it contains content.yml
        //  if it does, check if it contains .ministry
        //  if it does, check if it contains assets

        let path = Path::new(&self.directory_root);
        if !path.exists(){
            return false
        }
        let directory_root = path.to_str().unwrap();
        let content_path_location = PathBuf::from(&directory_root).join("content.yml");
        let content_path = Path::new(&content_path_location);
        if !content_path.exists(){
            return false
        }
        let ministry_path_location = PathBuf::from(&directory_root).join(".ministry");
        let ministry_path = Path::new(&ministry_path_location);
        if !ministry_path.exists(){
            return false
        }
        let assets_path_location = PathBuf::from(&directory_root).join("assets");
        let assets_path = Path::new(&assets_path_location);
        if !assets_path.exists(){
            return false
        }

        true
    }

    pub fn _get_content(&self) -> Result<String>{
        let content_path = PathBuf::from(&self.directory_root).join("content.yml");
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

        let image_url = match doc["image"].as_str(){
            Some(image_url) => {
                // test for the existence of image_url as a file
                let image_path = PathBuf::from(&self.directory_root).join("assets").join(image_url);
                if Path::new(&image_path).exists() {
                    println!("Image exists: {}", image_path.to_str().unwrap_or_else(|| ""));
                    Some(image_url.to_string())
                }
                else{
                    println!("Image does not exist: {}", image_path.to_str().unwrap_or_else(|| ""));
                    None
                }
            }
            None => None,
        };

        let dm = DeckMetadata{
            title: name_or_title.to_string(),
            slug: slugify!(name_or_title),
            author: author.to_string(),
            author_slug: slugify!(author),
            author_link: doc["author_link"].as_str().map(|s| s.to_string()),
            description: doc["description"].as_str().map(|s| s.to_string()),
            image_url: image_url,
            locale: doc["locale"].as_str().map(|s| s.to_string()),
            extra_header: doc["extra_header"].as_str().map(|s| s.to_string()),
        };
        // Pretty print
        println!("{:#?}", dm);
        Ok(dm)
    }

    fn parse_card(&self, doc: &yaml_rust2::Yaml, default_id: String) -> Card{
        let id = doc["id"].as_str().unwrap_or_else(|| &default_id).to_string();
        let mut card_type = doc["type"].as_str().unwrap_or_else(|| "").to_string();

        if card_type == "" {
            if doc["content"].as_str().is_some(){
                card_type = "markdown".to_string();
            }
            else if doc["image"].as_str().is_some(){
                card_type = "image".to_string();
            }
            else if doc["video"].as_str().is_some(){
                card_type = "video".to_string();
            }
            else{
                // this is our defaultiest default
                card_type = "title".to_string();
            }
        }

        let mut stack = Vec::new();
        if card_type == "stack"{

            // the card has multiple cards in it
            doc["pages"].as_vec().map(|list| {
                let mut counter = 0;
                for item in list
                {
                    let counter_string = counter.to_string();
                    let id = format!("{}-{}", id, counter_string);
                    stack.push(
                        self.parse_card(&item, id)
                    );
                    counter += 1;
                }
            });
        }

        let mut pngs = Vec::new();
        if card_type == "pngs" {
            let directory = doc["pngs"].as_str().unwrap_or_else(|| "");
            if directory != "" {
                let path = PathBuf::from(&self.directory_root).join(directory);
                // every file in the directory
                let paths = std::fs::read_dir(path).unwrap();
                for path in paths {
                    let path = path.unwrap().path();
                    let path = path.to_str().unwrap_or_else(|| "");
                    if path.ends_with(".png") {
                        pngs.push(path.replacen(&self.directory_root, "", 1).replace("\\", "/").to_string());
                    }
                }
            }
        }

        let mut extra_class = Vec::new();
        let default_vec = Vec::new();
        if doc["extra_class"].as_str().is_some(){
            extra_class.push(doc["extra_class"].as_str().unwrap().to_string());
        }
        for item in doc["extra_class"].as_vec().unwrap_or_else(|| &default_vec){
            if item.as_str().is_some(){
                extra_class.push(item.as_str().unwrap().to_string());
            }
        }
        println!("{:#?}", extra_class);

        Card{
            id,
            card_type,
            title: doc["title"].as_str().map(|s| s.to_string()),
            content: doc["content"].as_str().map(|s| s.to_string()),
            image_url: doc["image"].as_str().map(|s| s.to_string()),
            extra_class,
            video_url: doc["video"].as_str().map(|s| s.to_string()),
            video_has_sound: doc["video_has_sound"].as_bool().unwrap_or(false),
            video_loop: doc["video_loop"].as_bool().unwrap_or(false),
            video_controls: doc["video_controls"].as_bool().unwrap_or(false),
            pngs,
            pngs_fps: doc["pngs_fps"].as_i64(),
            pngs_loop: doc["pngs_loop"].as_bool(),
            fade_in: doc["fade_in"].as_i64(),
            stack,
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
                self.parse_card(&item, counter_string)
            );
            counter += 1;
        }

        Ok(deck)
    }

    pub fn get_asset_path(&self, asset_path: std::path::PathBuf) -> PathBuf{
        PathBuf::from(&self.directory_root).join("assets").join(asset_path)
    }

    pub async fn get_named_file(&self, asset_path: std::path::PathBuf, config: &crate::Config, file_directives: &crate::file_modifiers::FileDirectives) -> Result<rocket::fs::NamedFile>{
        let asset_path = &self.get_asset_path(asset_path);
        let filename = asset_path.file_name().unwrap_or_else(|| std::ffi::OsStr::new("")).to_str().unwrap_or_else(|| "");
        let do_not_modify_file = file_directives.unmodified.unwrap_or(false);

        if !do_not_modify_file && (filename.ends_with(".jpg") || filename.ends_with(".png") || filename.ends_with(".gif")){
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
                println!("Converting {} to {}", asset_path.to_str().unwrap_or(""), webp_path);
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
                    img = DynamicImage::ImageLumaA8(imageops::grayscale_alpha(&img));
                    img = DynamicImage::ImageRgba8(img.to_rgba8());
                }

                if file_directives.color.is_some() {
                    img = DynamicImage::ImageLumaA8(imageops::grayscale_alpha(&img));
                    img = DynamicImage::ImageRgba8(img.to_rgba8());

                    let (r, g, b) = file_directives.color().unwrap();

                    let existing_image = img.into_rgba8();
                    let e_width = existing_image.width();
                    let e_height = existing_image.height();

                    let buffer = ImageBuffer::from_fn(e_width, e_height, |x, y| {
                        let pixel = existing_image.get_pixel(x, y);
                        let mut pixel = pixel.0;
                        pixel[0] = (r as u32 * (255-pixel[0] as u32) / 255) as u8;
                        pixel[1] = (g as u32 * (255-pixel[1] as u32) / 255) as u8;
                        pixel[2] = (b as u32 * (255-pixel[2] as u32) / 255) as u8;
                        image::Rgba(pixel)
                    });

                    img = DynamicImage::ImageRgba8(buffer);
                }

                if file_directives.blur.unwrap_or(0.0) > 0.1 {
                    let blur_amount = file_directives.blur.unwrap_or(5.0);
                    img = DynamicImage::ImageRgba8(imageops::blur(&img, blur_amount));
                }

                if file_directives.flip_horizontal.unwrap_or(false) {
                    img = DynamicImage::ImageRgba8(imageops::flip_horizontal(&img));
                }

                if file_directives.flip_vertical.unwrap_or(false) {
                    img = DynamicImage::ImageRgba8(imageops::flip_vertical(&img));
                }

                if file_directives.flip_turnwise.unwrap_or(false) {
                    img = DynamicImage::ImageRgba8(imageops::rotate180(&img));
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

            Ok(opened_file)
        }
        else{
            // send it anyway
            //  earlier, I had a plan to only send files from an approved list of file extension or mimetypes
            //  but, remember, this is a content server for just ME, right?
            //  anyways, file extension is not a secure way to determine file type
            let opened_file = rocket::fs::NamedFile::open(asset_path).await?;

            Ok(opened_file)
        }
    }
}