use rand::{self, Rng};
use rocket::request::FromParam;
use std::borrow::Cow;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct NekoupId<'a>(Cow<'a, str>);

impl NekoupId<'_> {
    pub fn new() -> NekoupId<'static> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Did you just time-traveled to the past??")
            .as_secs()
            & 0xffffffffffc00000;

        let worker_id = 0; // currently unused
                           // may be useful in the future for horizontal-scaling

        let process_id = 0; // currently unused too, for the same reason as above

        let mut rng = rand::thread_rng();
        let noise = rng.gen::<u64>();

        NekoupId::build(timestamp, worker_id, process_id, noise)
    }

    pub fn build(timestamp: u64, worker_id: u64, process_id: u64, noise: u64) -> NekoupId<'static> {
        let id = (timestamp << 22)
            | ((worker_id & 0x3e0000) << 17)
            | ((process_id & 0x1f000) << 12)
            | (noise & 0xfff);

        NekoupId(Cow::Owned(id.to_string()))
    }

    pub fn file_path(&self) -> PathBuf {
        Path::new("").join(self.0.as_ref())
    }
}

impl TryFrom<&str> for NekoupId<'_> {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let decoded = value
            .to_string()
            .parse::<u64>()
            .expect("Unable to decode ID param");

        let timestamp = decoded >> 22;
        let worker_id = (decoded & 0x3e0000) >> 17;
        let process_id = (decoded & 0x1f000) >> 12;
        let noise = decoded & 0xfff;

        let is_timestamp_higher_than_epoch = timestamp > 0;
        let is_worker_id_zero = worker_id == 0;
        let is_process_id_zero = process_id == 0;

        if is_timestamp_higher_than_epoch && is_worker_id_zero && is_process_id_zero {
            return Ok(NekoupId::build(timestamp, worker_id, process_id, noise));
        }

        Err(format!(
            "a NekoupID could not be built out of this &str value: {}",
            value
        )
        .to_string())
    }
}

impl<'a> FromParam<'a> for NekoupId<'a> {
    type Error = &'a str;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        param
            .chars()
            .all(|c| c.is_ascii_digit())
            // this is awful but i didn't manage to find any
            // other way around
            .then(|| NekoupId::try_from(param).unwrap())
            .ok_or(param)
    }
}

impl ToString for NekoupId<'_> {
    fn to_string(&self) -> String {
        self.0.as_ref().to_string()
    }
}
