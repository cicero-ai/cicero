
use std::fs;
use verax::{forge, License};
use cicero_interfaces::ForgeAPI;
use crate::{tools, CONFIG};

pub struct Packager { }

impl Packager {

    pub fn new() -> Self {
        Self { }
    }

    /// release package
    pub fn release(&self, license: &License) {

        // Prepare release dir
        let release_dir = format!("{}/release", CONFIG.datadir);
        tools::prepare_dir(&release_dir);

        // Read directory
        let dirname = format!("{}/{}", CONFIG.datadir, license.product);
        let (files, dirs) = tools::read_dir(&dirname);

        // Go through files
        for file in files.iter() {
        let filename = format!("{}/{}", dirname, file);
    let dec_contents = fs::read(&filename).unwrap();
            let contents = license.encrypt(&dec_contents.as_slice());

            let res_file = format!("{}/release/{}", CONFIG.datadir, file);
            fs::write(&res_file, &contents).unwrap();
        }

        // Save license
        let license_file = format!("{}/release/license.dat", CONFIG.datadir);
        license.save(&license_file);
    }

}



