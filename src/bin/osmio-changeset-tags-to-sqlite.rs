extern crate osmio;
extern crate serde;
extern crate anyhow;
extern crate serde_json;
extern crate rusqlite;
extern crate iter_progress;
use rusqlite::{params, Connection};
use iter_progress::OptionalProgressableIter;
use std::collections::HashMap;

use osmio::changesets::ChangesetReader;
use std::env::args;
use std::path::{PathBuf};
use anyhow::{ensure, Result};

fn main() -> Result<()> {
    let changeset_filename = args().nth(1).expect("provide osc filename as arg 1");
    let sqlite_filename = PathBuf::from(args().nth(2).expect("provide sqlite filename as arg 1"));

    let osc = ChangesetReader::from_filename(&changeset_filename)?;

    ensure!(!sqlite_filename.exists(), "Sqlite filename {} already exists", sqlite_filename.to_str().unwrap());
    let mut conn = Connection::open(sqlite_filename)?;

    conn.execute(
        "CREATE TABLE changeset_tags (
                  id              INTEGER PRIMARY KEY,
                  created_by            TEXT NOT NULL,
                  comment            TEXT NOT NULL,
                  other_tags            TEXT NOT NULL
          )",
        [],
    )?;

    let txn = conn.transaction()?;

    let mut cid;
    let mut all_tags: HashMap<String, String>;
    let mut tags: Vec<(String, String)>;
    let mut tags_json: Vec<u8> = Vec::new();
    let mut changeset;
    let mut tag_popularity: HashMap<String, usize> = HashMap::new();
    let mut num_changesets = 0;
    let mut num_changesets_with_tags = 0;
    let mut tag_created_by;
    let mut tag_comment;
    for (state, changeset_res) in osc.into_iter().optional_progress(10000).assume_size(110_000_000) {
        if let Some(state) = state {
            state.do_every_n_sec(2., |state| {
                println!("{:?}s {}k / {:.1}% done. eta: {} sec {:.0} per sec", state.duration_since_start().as_secs(), state.num_done()/1000, state.percent().unwrap_or(0.), state.eta().map_or_else(|| "N/A".to_string(), |d| d.as_secs().to_string()) , state.rate());
            });
        }
        changeset = changeset_res?;
        num_changesets += 1;
        if changeset.tags.is_empty() {
            continue;
        }
        num_changesets_with_tags += 1;

        cid = changeset.id as usize;
        all_tags = changeset.into_tags();
        tag_created_by = all_tags.remove("created_by").unwrap_or_else(|| "".to_string());
        tag_comment = all_tags.remove("comment").unwrap_or_else(|| "".to_string());
        tags = all_tags.into_iter().collect();
        for (k, _) in tags.iter() {
            if !tag_popularity.contains_key(k) {
                tag_popularity.insert(k.to_owned(), 1);
            } else {
                *tag_popularity.get_mut(k).unwrap() += 1;
            }
        }
        tags_json.truncate(0);
        serde_json::to_writer(&mut tags_json, &tags)?;

        txn.execute(
            "INSERT INTO changeset_tags (id, created_by, comment, other_tags) VALUES (?1, ?2, ?3, ?4)",
            params![cid, tag_created_by, tag_comment, tags_json],
        )?;


    }
    txn.commit()?;

    println!("Inserted {} changesets, and got {} unique tags", num_changesets, tag_popularity.len());
    let mut tag_popularity = tag_popularity.into_iter().map(|(k, v)| (v, k)).collect::<Vec<(usize, String)>>();
    tag_popularity.sort();
    tag_popularity.reverse();
    for (num, key) in tag_popularity.iter().take(100) {
        if num*100 < num_changesets_with_tags {
            break;
        }
        println!("{:>8} {:>3}% all {:>3}% tagged {}", num, (num*100)/num_changesets, (num*100)/num_changesets_with_tags, key);
    }
    

    Ok(())
}
