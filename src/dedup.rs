extern crate img_hash;
use crate::factory;
use img_hash::{HasherConfig, ImageHash};
use serde::Deserialize;
use serde::Serialize;
use std::io::Write;
// use std::fs;
// use image::GenericImageView;

#[derive(Clone, Debug)]
pub struct ImgHashStruct {
    pub img_path: String,
    pub hash: ImageHash,
}
pub fn calc_hash(apath: String) -> ImgHashStruct {
    let image_results = image::open(apath.clone()).expect(apath.clone().as_str());
    let hasher_config = HasherConfig::new().to_hasher();
    let hashed = hasher_config.hash_image(&image_results);
    let imghash = ImgHashStruct {
        img_path: apath.clone(),
        hash: hashed,
    };

    imghash
}

pub fn calc_hash_test(apath: String, bad_image_dir: &str) -> bool {
    let image_results = image::open(apath.clone());
    let _image = match image_results {
        Ok(_) => return true,
        Err(e) => {
            println!("Error: {}", e);
            let bisplit = apath.split("/").collect::<Vec<&str>>();
            let bfilename = bisplit.last().unwrap().to_string();
            let bad_image_path = bad_image_dir.to_owned() + bfilename.as_str();
            let _ = std::fs::rename(apath.clone(), bad_image_path.clone());
            println!(
                "Moved: \n\t{} \nto \n\t{}",
                apath.clone(),
                bad_image_path.clone()
            );
            return false;
        }
    };
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DupsEntry {
    pub filename: String,
    pub duplicates: Vec<String>,
}
pub fn compare_hashes(afile: String, img_hash_list: Vec<ImgHashStruct>, addr: String) -> DupsEntry {
    let info = calc_hash(afile.clone());
    let in_filename = info.img_path.clone();
    let in_hash = info.hash.clone();
    let mut duplicates = Vec::new();
    for bfile in img_hash_list.clone() {
        let fnn = bfile.img_path.clone();

        let fnn_split = fnn.split("/").collect::<Vec<&str>>();
        let out_filename = fnn_split.last().unwrap().to_string();
        let out_hash = bfile.hash.clone();
        if in_filename != out_filename {
            let hammer = in_hash.dist(&out_hash);
            if hammer < 5 {
                duplicates.push(out_filename.clone());
            }
        };
    }

    let dups_entry = DupsEntry {
        filename: in_filename.clone(),
        duplicates: duplicates.clone(),
    };

    if duplicates.len() > 0 {
        let f = factory::Factory {
            path: afile.clone(),
        };
        let ddoutfile = f.create_dedup_filename();

        let transform = transform_dup_entry_struct(dups_entry.clone(), ddoutfile.clone());

        let json = serde_json::to_string(&transform).unwrap();

        let ofile = addr.clone() + ddoutfile.as_str();
        // let ofile = "/media/pipi/e9535df1-d952-4d78-b5d7-b82e9aa3a975/ToRemove/".to_string()
        //     + ddoutfile.as_str();

        let mut output_file_results = std::fs::File::create(ofile.clone()).unwrap();
        output_file_results.write_all(json.as_bytes()).unwrap();
    }

    dups_entry
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TransDupsEntry {
    pub jsonfilename: String,
    pub filename: String,
    pub httpfilename: String,
    pub duplicates: Vec<DupStruct>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DupStruct {
    pub strdups: String,
    pub httpdups: String,
}

fn transform_dup_entry_struct(dups_entry: DupsEntry, jsonfilename: String) -> TransDupsEntry {
    let filename = dups_entry.filename.clone();
    let filename_parts = filename.split("/").collect::<Vec<&str>>();
    let fname = filename_parts.len() - 1;
    let http_filename = "http://192.168.0.91:8181/image/".to_string() + filename_parts[fname];
    let mut comp_duplicates = Vec::new();
    for dup in dups_entry.duplicates.clone() {
        let dup_parts = dup.split("/").collect::<Vec<&str>>();
        let dp = dup_parts.len() - 1;
        let http_dup = "http://192.168.0.91:8181/image/".to_string() + dup_parts[dp];
        let dupsstruct = DupStruct {
            strdups: dup.clone(),
            httpdups: http_dup.clone(),
        };
        comp_duplicates.push(dupsstruct);
    }

    let trans_dup_entry = TransDupsEntry {
        filename: filename.clone(),
        jsonfilename: jsonfilename.clone(),
        httpfilename: http_filename.clone(),
        duplicates: comp_duplicates.clone(),
    };

    println!("dups_entry: {:#?}", trans_dup_entry);

    trans_dup_entry
}
