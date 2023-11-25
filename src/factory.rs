use image::GenericImageView;
use md5::compute;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;
// use std::path::PathBuf;
// use flate2::read::GzDecoder;
// use tar::Archive;

#[derive(Debug)]
pub struct Factory {
    pub path: String,
}

impl Factory {
    pub fn create_outfile(&self) -> String {
        let digest = compute(&self.path);
        let a = "/media/pipi/e9535df1-d952-4d78-b5d7-b82e9aa3a975/Converted/".to_string();
        let b = format!("{:?}", digest) + ".jpg";
        let newfilename = a + &b;

        newfilename
    }

    pub fn create_rename_output_file(&self) -> String {
        let fparts = self.path.split(".").collect::<Vec<&str>>();
        let filename = fparts.first().unwrap().replace(" ", "_");
        let addr = "/media/pipi/e9535df1-d952-4d78-b5d7-b82e9aa3a975/Converted/".to_string()
            + &filename
            + ".jpg";

        addr
    }

    pub fn create_bad_image_output_file(&self) -> String {
        let fparts = self.path.split("/").collect::<Vec<&str>>();
        let filename = fparts.last().unwrap().replace(" ", "_");
        let addr = "/media/pipi/0123-4567/BadImages/".to_string() + &filename;

        // let addr = "/media/pipi/e9535df1-d952-4d78-b5d7-b82e9aa3a975/BadImages/".to_string()
        //     + &filename
        //     + ".jpg";

        addr
    }

    pub fn create_dedup_filename(&self) -> String {
        let digest = compute(&self.path);
        let fdigest = format!("{:?}", digest);
        let addr = fdigest + ".json";

        addr
    }
    pub fn create_gz_out_dir(&self) -> String {
        let digest = compute(&self.path);
        let fdigest = format!("{:?}", digest);
        let addr = "/media/pipi/e9535df1-d952-4d78-b5d7-b82e9aa3a975/GZ/".to_string() + &fdigest;

        addr
    }

    pub fn create_fixed_filename(&self) -> String {
        let digest = compute(&self.path);
        let fdigest = format!("{:?}", digest);
        let addr = "/media/pipi/e9535df1-d952-4d78-b5d7-b82e9aa3a975/Master/".to_string()
            + &fdigest
            + ".jpg";

        addr
    }
    pub fn resize_landscape_image(&self) {
        let image = image::open(self.path.clone()).unwrap();
        let img_dims = image.dimensions();
        let aspect = img_dims.0 as f64 / img_dims.1 as f64;
        let (newwidth, newheight) =
            calc_new_landscape_dims(image.width() as f64, image.height() as f64, aspect);
        let resized = image.resize(
            newwidth as u32,
            newheight as u32,
            image::imageops::FilterType::Lanczos3,
        );
        let _save_image = resized.save(self.create_outfile()).unwrap();
    }

    pub fn dir_count(&self) -> i32 {
        let mut file_count = 0;

        for entry in fs::read_dir(self.path.clone()).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                file_count += 1;
            }
        }

        file_count
    }
}

pub fn calc_new_landscape_dims(oldwidth: f64, oldheight: f64, aspect: f64) -> (f64, f64) {
    let mut newwidth = 0.0;
    let mut newheight = 0.0;
    if oldwidth > 800.0 as f64 {
        newwidth = 800.0 as f64;
        newheight = 800.0 as f64 / aspect.clone();
    } else if oldwidth < oldheight {
        println!("portrait");
        if oldheight > 800.0 as f64 {
            newheight = 800.0 as f64;
            newwidth = 800.0 as f64 / aspect.clone();
        };
    } else {
        println!("square");
    };
    println!(
        "width: {}\nheight: {}\naspect_ratio: {}\nnewwidth: {}\nnewheight: {}\n",
        oldwidth, oldheight, aspect, newwidth, newheight
    );

    (newwidth, newheight)
}

pub fn gen_ext_list(apath: String) -> Vec<String> {
    let mut ext_list: Vec<String> = Vec::new();
    for e in WalkDir::new(apath.clone())
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            let fname = e.path();
            if let Some(extension) = fname.extension() {
                let ext = extension.to_owned().to_str().unwrap().to_string();
                if !ext_list.contains(&ext) {
                    ext_list.push(ext);
                };
            };
        }
    }

    ext_list
}

pub fn convert_image_to_jpg(a_path: String) {
    let apath = Path::new(a_path.as_str());
    let pf = Factory {
        path: a_path.clone(),
    };
    let outfile = pf.create_outfile();
    let image_results = image::open(apath);
    let myimage = match image_results {
        Ok(myimage) => myimage,
        Err(e) => {
            println!("Error: {}", e);
            let ofile = pf.create_bad_image_output_file();
            std::fs::rename(apath, ofile.clone()).unwrap();
            println!("Moved: {}", apath.display());
            return;
        }
    };
    let _save_image = myimage.save(outfile).unwrap();
}


pub fn rename_fixed_files(apath: String) {
    for e in WalkDir::new(apath.clone())
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            let fname = e.path().to_string_lossy().to_string();
            let pf = Factory {
                path: fname.clone(),
            };
            let outfile = pf.create_fixed_filename();
            fs::rename(fname, outfile.clone()).unwrap();
            println!("Moved: {}", outfile);

        }
    }
}