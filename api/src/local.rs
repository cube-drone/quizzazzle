///
/// You're runninng ministry on your own machine, where it works more like... git.
///
use std::path::Path;
use std::fs::File;
use std::fs::create_dir;
use std::io::*;
use anyhow::Result;
use text_io::read;
use tera::Tera;

struct LocalDirectory{

}

pub fn init(slug: String) -> Result<()>{
    // "ministry init" just initializes a new ministry project.
    // the user will be prompted for a slug, title, and description
    // the new ministry project is comprised of:
    // .ministry/ - a hidden directory that stores ministry metadata
    // assets/ - a directory that stores static files
    // build/ - a directory that stores the built project
    // content.yml -
    // it won't create a new project if one already exists
    // if content.yml already exists, throw an error:
    if Path::new("content.yml").exists(){
        println!("Error: content.yml already exists. You can't create a new project here.");
    }

    let tera = Tera::new("templates_local/*").unwrap();
    let mut context = tera::Context::new();

    let mut slug = slug;
    if slug == "" {
        // prompt for a slug
        println!("Enter a slug for your project: [default: 'new-project']");
        slug = read!("{}\n");
    }
    if slug == ""{
        slug = "new-project".to_string();
    }
    println!("Creating a new project with slug: {}...", slug);

    context.insert("slug", &slug);
    let content_yml = tera.render("content.yml.tera", &context)?;
    println!("... content.yml");

    let mut file = File::create("content.yml")?;
    file.write_all(content_yml.as_bytes())?;

    create_dir("assets")?;

    Ok(())
}

pub fn status(){
    // "ministry status" shows the current status of the project
    // it will compare the current project with the last uploaded build
    // if there is no last uploaded build, it will show all files
}

pub fn diff(){
    // "ministry diff" shows the differences between the current project and the last uploaded build
    // for each file that's different, it will show the diff
    // (for large binary files, it will show a hash of the file)
}

pub fn login(){
    // "ministry login" logs the user into their ministry account
    // it will prompt the user for the address of their ministry server (default: https://ministry.groovelet.com)
    // it will prompt the user for their email and password
    // it will store the user's token in .ministry/token
}