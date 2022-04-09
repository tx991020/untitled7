

use anyhow::{Result};
use tokio::process::Command;
use std::cell::RefCell;







#[tokio::main]
async fn main()->Result<()>{
    let mut a = None;
    let mut b = Some(10);
     a = b.take();
    println!("{:?}",a );

    let x = Command::new("date").output().await?;
    let _result = std::str::from_utf8(x.stdout.as_slice())?;

    let c = RefCell::new("hello".to_owned());
    *c.borrow_mut() = "wow".to_string();



    Ok(())

}