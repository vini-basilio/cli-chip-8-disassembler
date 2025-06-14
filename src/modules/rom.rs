pub struct Rom {
    pub file_name: String,
}

impl Rom {
    pub fn new(args: &[String]) -> Result<Rom, &str> {
        if args.len() < 2 {
            return Err("File path not provided");
        }
        let file_name = args[1].clone();
        Ok(Rom { file_name })
    }
}