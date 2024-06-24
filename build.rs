use bbl_build::Config;

pub fn main() {
    let _dst = Config::new("oiio", "../bbl-oiio")
        .define("BBL_LANGUAGES", "rust")
        .build();

    println!("cargo:rerun-if-changed=../bbl-oiio");
}