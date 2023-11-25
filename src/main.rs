// use std::fs;
// use std::fs::rename;
// use std::path::Path;
use std::sync::mpsc::channel;
// use std::time::Instant;
use threadpool::ThreadPool;
// use image::GenericImageView;

pub mod dedup;
pub mod factory;
pub mod walk_dirs;

fn main() {
    let url2 = "/media/pipi/0123-4567/png_to_jpg/".to_string();
    // let bad_image_dir = "/media/pipi/e9535df1-d952-4d78-b5d7-b82e9aa3a975/BadImages/";
    let bad_image_dir = "/media/pipi/0123-4567/BadImages/";
    let to_remove_addr ="/media/pipi/0123-4567/ToRemove/".to_string();


    let pic_list3 = walk_dirs::walk_dir(url2.clone());
    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = channel();
    for pic in pic_list3.clone() {
        let tx = tx.clone();
        pool.execute(move || {
            dedup::calc_hash_test(pic.clone(), bad_image_dir);
            tx.send(()).unwrap();
        });
    }
    drop(tx);
    for t in rx.iter() {
        let _info = t;
        // println!("info: {:?}", info)
    }

    let pic_list2 = walk_dirs::walk_dir(url2.clone());
    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = channel();
    for jpg in pic_list2 {
        println!("jpg {}", jpg);
        let tx = tx.clone();
        pool.execute(move || {
            let dd = dedup::calc_hash(jpg.clone());
            tx.send(dd).unwrap();
        });
    }
    drop(tx);
    let mut img_hash_list = Vec::new();
    for t in rx.iter() {
        let info = t;
        img_hash_list.push(info.clone());
        println!("info: {:?}", info.clone());
    }

    let file_list = walk_dirs::walk_dir(url2.clone());

    let mut dup_results = Vec::new();
    for jpg in file_list {
        println!("jpg: {}", jpg);
        let image_hash_list2 = img_hash_list.clone();
        let dd = dedup::compare_hashes(jpg.clone(), image_hash_list2.clone(), to_remove_addr.clone());
        dup_results.push(dd.clone());
    }

    println!("dup_results: {:?}", dup_results.clone());
}
