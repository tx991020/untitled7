use pest::Parser;
use pest_derive::Parser;
use serde::Deserialize;

// #[derive(Parser)]
// #[grammar = "/Users/andy/CLionProjects/untitled7/apps/sshd.pest"]
// pub struct SSHDParser;

#[derive(Deserialize)]
pub struct Entry {
    #[serde(rename = "MESSAGE")]
    message: String,
    #[serde(rename = "_PID")]
    pid: String,
}



#[derive(Parser)]
#[grammar = "/Users/andy/CLionProjects/untitled7/apps/sql.pest"]
pub struct SQLParser;

pub fn is_valid_sql(query: &str) -> bool {
    SQLParser::parse(Rule::sql, query).is_ok()
}

fn main() {
    // SSHDParser::parse(
    //     Rule::login
    // )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_login() {
        // SSHDParser::parse(
        //     Rule::login,
        //     "Accepted publickey for zhanglei.sec from 10.87.61.221 port 50998 ssh2: RSA SHA256:l9nMCPKgwkWtfRKH4INyvpU3e+PIXtdKsm3jrvXRuMo",
        // )
        //     .unwrap();

    }
    #[test]
    fn parses_select_many_columns() {
        let statement = "select something,another_one,yet_another,so_many_columns from test;";
        assert_eq!(true, is_valid_sql(statement))
    }
}
