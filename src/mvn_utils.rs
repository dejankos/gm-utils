use io::Result;
use std::io;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use std::process::Command;

use roxmltree::Document;

use crate::file_utils;

const SUCCESS: &str = "BUILD SUCCESS";
const POM_FILE: &str = "pom.xml";

pub fn set_new_version(ver: &str, project_path: PathBuf) -> Result<()> {
    let split: Vec<String> = split_current_ver(&project_path)?;
    let new_version = format!("{}-{}-{}", split[0], ver, split[1]);
    info!("Changing mvn project version to {}", &new_version);

    change_version(&new_version, &project_path)
}

pub fn reset_version(project_path: PathBuf) -> Result<()> {
    let split: Vec<String> = split_current_ver(&project_path)?;
    let new_version = format!("{}-{}", split[0], "SNAPSHOT");
    info!("Changing mvn project version to {}", &new_version);

    change_version(&new_version, &project_path)
}

fn split_current_ver(project_path: &PathBuf) -> Result<Vec<String>> {
    let current_ver = find_version(&project_path)?;
    info!("Current mvn project version {}", current_ver);
    let split: Vec<String> = current_ver.split('-').map(|s| s.to_string()).collect();

    Ok(split)
}

fn change_version(new_version: &str, project_path: &PathBuf) -> Result<()> {
    let out = Command::new("mvn")
        .args(&[
            "-f",
            format!("{}/{}", project_path.to_str().unwrap(), POM_FILE).as_str(),
            "versions:set",
            format!("-DnewVersion={}", new_version).as_str(),
        ])
        .output()
        .expect("Failed to change mvn version!");

    let out_str = String::from_utf8(out.stdout).unwrap();
    if !out_str.contains(SUCCESS) {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Never thought this was gonna work anyway",
        ));
    }

    Ok(())
}

fn find_version(project_path: &PathBuf) -> Result<String> {
    if let Some(res) = file_utils::find_in_dir(&project_path, POM_FILE) {
        let c = file_utils::read_file_content(&res)?;
        parse_pom_ver(c).ok_or_else(|| Error::new(
            ErrorKind::InvalidData,
            "Failed to parse project version",
        ))
    } else {
        Err(Error::new(ErrorKind::InvalidData, "Pom file not found"))
    }
}

fn parse_pom_ver(pom_xml: String) -> Option<String> {
    let doc = match Document::parse(pom_xml.as_str()) {
        Ok(r) => r,
        Err(_) => return None,
    };

    for n in doc.root().descendants() {
        if n.is_element()
            && n.has_tag_name("version")
            && !n.parent().unwrap().has_tag_name("parent")
            && n.text().is_some()
        {
            if let Some(txt) = n.text() {
                return Some(txt.into());
            }
        }
    }

    None
}
