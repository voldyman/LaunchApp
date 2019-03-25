extern crate clap;
extern crate gio;

use clap::{App, Arg};
use gio::{DesktopAppInfo, AppInfoExt};

fn main() {
    let matches = App::new("run-app")
        .version("1.0")
        .author("Akshay")
        .arg(Arg::with_name("file").multiple(true))
        .get_matches();

    let files = matches
        .values_of_lossy("file")
        .expect("must specify a file to launch");
    files
        .iter()
        .filter(|f| f.ends_with(".desktop"))
        .for_each(|f| launch_app(f));
}

fn launch_app(df: &String) {
    let app = match DesktopAppInfo::new_from_filename(df) {
        Some(val) => val,
        None => return,
    };
    let uris : &[gio::File] = &vec![];
    let ctx : Option<&gio::AppLaunchContext> = None;
    if let Err(err) = app.launch(uris, ctx) {
        println!("Could not launch app: {}", err);
    }
}
