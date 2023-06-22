pub struct Args {
    pub host: String,
    pub user: String,
    pub database: String,
}

impl Args {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let mut args = pico_args::Arguments::from_env();

        Ok(Args {
            host: args.value_from_str(["-h", "--host"])?,
            user: args.value_from_str(["-U", "--username"])?,
            database: args.value_from_str(["-d", "--database"])?,
        })
    }
}
