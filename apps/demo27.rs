


extern crate rexpect;

use rexpect::{spawn, spawn_bash};
use rexpect::errors::*;

fn do_ftp() -> Result<()> {
    let mut p = spawn_bash(Some(1000))?;
    p.execute("ssh root@39.107.33.253", "bytes of data")?;
    p.send_line("Aa@1234567")?;
    p.send_line("pwd")?;
    p.send_line("exit")?;
    p.exp_eof()?;
    Ok(())
}


fn main()->Result<()> {
    let x = do_ftp()?;
    dbg!(x);
    Ok(())
}