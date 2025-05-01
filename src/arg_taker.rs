pub mod arg_taker{
    pub fn get_arg(name: &str) -> Result<String, u32> {
        let args: Vec<String> = std::env::args().collect();

        for (i, arg) in args.iter().enumerate(){
            if arg == name {
                let r = args.get(i+1);
                match r{
                    Some(e) => return Ok(e.to_string()),
                    None  => return Err(args.len() as u32)
                }
            }
        }

        return Err(args.len() as u32)       
    }
}
