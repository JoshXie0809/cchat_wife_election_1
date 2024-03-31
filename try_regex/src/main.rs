use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;
use rusqlite::Connection;

fn combine_strs(strings: &[&str]) -> String {
    strings.join(" ")
}

fn main() {
    // 创建正则表达式以拆分行
    let re = Regex::new(r"\s+").unwrap();
    let mut nameid_vec: Vec<String> = vec![];
     // 打开 SQLite 数据库连接
     let mut conn = Connection::open("../vote.sqlite").unwrap();
     conn.execute(
         "CREATE TABLE IF NOT EXISTS vote (
             nameid TEXT PRIMARY KEY NOT NULL,
             tweet_num INTEGER NOT NULL
         )",
         [],
     ).unwrap();

     conn.execute(
         "CREATE TABLE IF NOT EXISTS content_tbl (
             id INTEGER PRIMARY KEY,
             push TEXT NOT NULL,
             nameid TEXT NOT NULL,
             content TEXT,
             time TEXT NOT NULL,
             game TEXT NOT NULL
         )",
         [],
     ).unwrap();

    // 遍历游戏列表
    for game in ["Game1", "Game2", "Game3", "final"].iter() {

        let file = File::open(&format!("../{}.txt", game))
            .unwrap_or_else(|err| panic!("Error opening file {}: {}", game, err));

        // 使用 BufReader 以缓冲方式读取文件
        let reader = BufReader::new(file);
        let mut n = 0;

        let transaction = conn.transaction().unwrap();

        // 遍历文件中的每一行
        for line in reader.lines() {
            if let Ok(line) = line {
                let sline = re.split(&line).collect::<Vec<&str>>();
                n += 1;

                if !["推", "噓", "→"].contains(&sline[0]) {
                    continue;
                }

                let nameid = sline[1].replace(":", "");

                // 检查 nameid 是否已存在于数据库中
                let exists = nameid_vec.contains(&nameid);
                if !exists {
                    nameid_vec.push(String::from(&nameid));
                    transaction.execute("INSERT INTO vote (nameid, tweet_num) VALUES (?1, 1)", &[&nameid]).unwrap();
                } else {
                    transaction.execute("UPDATE vote SET tweet_num = tweet_num + 1 WHERE nameid = ?1", &[&nameid]).unwrap();
                }

                let vec_n = sline.len();
                let content = combine_strs(&sline[2..(vec_n - 2)]);
                let time = combine_strs(&sline[(vec_n - 2)..vec_n]);

                transaction.execute(
                    "INSERT INTO content_tbl (nameid, content, push, time, game) VALUES (?1, ?2, ?3, ?4, ?5)",
                    &[&nameid, &content, sline[0], &time, game]
                ).unwrap();
            } else {
                eprintln!("Error reading line");
            }
        }

        let _ = transaction.commit();
        println!("Batch insert completed successfully.");
        println!("There are {} lines written to database.\n", n);
    }
}