
pub struct UpYunImpl {
    service: String,
    username: Option<String>,
    password: Option<String>,
    auth_server: Option<String>,
    endpoint: String,
    chunksize: u32,
    timeout: u32,
}

impl UpYunImpl {
    pub(crate) fn new(

    ) -> Result<Self>
}