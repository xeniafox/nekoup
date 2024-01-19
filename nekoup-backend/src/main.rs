#[macro_use]
extern crate rocket;

use nekoup_id::NekoupId;
use rocket::data::{Data, ToByteUnit};
use rocket::State;
use std::fs;
use std::path::Path;

use nekoup_config::{load_config, NekoupConfig};
use rocket::tokio::fs::{read_to_string, File};

#[get("/")]
async fn homepage(config: &State<NekoupConfig>) -> String {
    let host = &config.backend.host;

    let homepage = read_to_string(concat!("data", "/", "homepage"))
        .await
        .expect("unable to read homepage HTML page file!");

    /* format! does not work with non-literal strings,
    so this is a little hack to proper replace the placeholders
    without the need of a whole template engine */
    return str::replace(&homepage, "{host}", host);
}

#[get("/<id>/<filename>")]
async fn retrieve(config: &State<NekoupConfig>, id: &str, filename: &str) -> Option<File> {
    let root_dir = &config.backend.upload_dir;
    let filepath = Path::new(root_dir).join(id).join(filename);

    File::open(&filepath).await.ok()
}

#[put("/<filename>", data = "<file>")]
async fn upload_file(
    config: &State<NekoupConfig>,
    filename: &str,
    file: Data<'_>,
) -> std::io::Result<String> {
    let host = &config.backend.host;
    let upload_dir = &config.backend.upload_dir;

    let id = NekoupId::new();
    let id_path = Path::new(upload_dir).join(id.file_path());
    fs::create_dir(&id_path).expect("Unable to create ID folder!");

    let filepath = id_path.join(filename);
    file.open(1024.mebibytes()).into_file(filepath).await?;

    Ok(format!("{}/{}/{}", host, id.to_string(), filename))
}

#[launch]
fn rocket() -> _ {
    let config = load_config();

    rocket::build()
        .manage(config)
        .mount("/", routes![homepage, upload_file, retrieve])
}
