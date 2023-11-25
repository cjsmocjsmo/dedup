use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub fn walk_dir(apath: String) -> Vec<String> {
    let mut keeper_vec = Vec::new();
    let mut idx = 0;
    let keeplist = [
        "jpg", "JPG", "jpeg", "JPEG", "bmp", "BMP", "gif", "GIF", "png", "PNG", "tif", "TIF", "json",
    ];

    for e in WalkDir::new(apath)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            idx += 1;
            let fname = e.path().to_string_lossy().to_string();
            let path = Path::new(&fname);
            if path.is_file() {
                let parts = &fname.split(".").collect::<Vec<&str>>();
                let ext = parts.last().unwrap();
                if keeplist.contains(&ext) {
                    keeper_vec.push(fname.clone());
                };
            };
        };
    }
    println!("Total files: {}\n", idx);

    keeper_vec
}

pub fn mv_small_images(oldwidth: f64, oldheight: f64, fname: String) {
    if oldwidth < 200.0 as f64 || oldheight < 100.0 as f64 {
        let old_fn = fname.clone();
        let fn_parts = old_fn.split("/").collect::<Vec<&str>>();
        let fnam = fn_parts.last().unwrap();
        let new_fn = "/media/pipi/0123-4567/SmallPics/".to_string() + fnam;
        match fs::rename(&fname, &new_fn) {
            Ok(_) => println!("Moved: {}", new_fn),
            Err(e) => println!("Error: {}", e),
        };
    }
}

// pub fn calc_new_dims(oldwidth: f64, oldheight: f64, aspect: f64) -> (f64, f64) {
//     let mut newwidth = 0.0;
//     let mut newheight = 0.0;
//     if oldwidth > 800.0 as f64 {
//         newwidth = 800.0 as f64;
//         newheight = 800.0 as f64 / aspect.clone();
//     } else if oldwidth < oldheight {
//         println!("portrait");
//         if oldheight > 800.0 as f64 {
//             newheight = 800.0 as f64;
//             newwidth = 800.0 as f64 / aspect.clone();
//         };
//     } else {
//         println!("square");
//     };
//     println!(
//         "width: {}\nheight: {}\naspect_ratio: {}\nnewwidth: {}\nnewheight: {}\n",
//         oldwidth, oldheight, aspect, newwidth, newheight
//     );

//     (newwidth, newheight)
// }

// pub fn get_aspect_ratio(apath: String) -> Vec<String> {
//     println!("apath: {}", apath);
//     let image = image::open(apath.clone()).expect(&apath);
//     let (width, height) = image.dimensions();
//     let oldwidth = width.clone() as f64;
//     let oldheight = height.clone() as f64;
//     let aspect_ratio = oldwidth / oldheight;
//     let mut av_vec = Vec::new();
//     av_vec.push(apath);
//     av_vec.push(oldwidth.clone().to_string());
//     av_vec.push(oldheight.clone().to_string());
//     av_vec.push(aspect_ratio.clone().to_string());
//     println!(
//         "width: {}\nheight: {}\naspect_ratio: {}\n",
//         oldwidth, oldheight, aspect_ratio);

//     av_vec
// }
