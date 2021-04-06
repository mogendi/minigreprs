use std::fs;
use std::error::Error;

pub struct Ctx {
    query: String,
    filename: String,
}

impl Ctx {
    pub fn new(mut args: std::env::Args) -> Result< Ctx, & 'static str> {
        args.next(); 
        let qr = match args.next() {
            Some(n) => n,
            None => return Err("No query string"),
        };

        let fl = match args.next() {
            Some(n) => n,
            None => return Err("No destination file"),
        };
        Ok(Ctx {query: qr, filename: fl})
    }

    pub fn grep_ctx(&self) -> Result<(), Box<dyn Error>> {
        let fcs: String = fs::read_to_string(&self.filename)?;
        let x: Vec<&str> = fcs.lines().filter(|ln| ln.contains(&self.query)).collect();
        println!("{:?}", x);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Ctx;
    use std::env;

    #[test]
    #[should_panic]
    fn search_test() {
        let ctx = Ctx::new(env::args()).unwrap_or_else(|err| {
            panic!("Error on: {}", err)
        });
        match ctx.grep_ctx() {
            Ok(_) => (),
            Err(e) => panic!("Unable to grep: {}", e)
        }
    }

    #[test]
    fn case_sensitivity() {
        let ctx = Ctx::new(env::args()).unwrap_or_else(|err| {
            panic!("Error on: {}", err)
        });
        assert_eq!(1, match ctx.grep_ctx(){
            Ok(_) => 1,
            Err(e) => panic!("Error: {}", e)
        });
    }
}