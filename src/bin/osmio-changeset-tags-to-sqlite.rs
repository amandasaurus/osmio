extern crate anyhow;
extern crate iter_progress;
extern crate osmio;
extern crate rusqlite;
extern crate serde;
extern crate serde_json;
use iter_progress::OptionalProgressableIter;
use rusqlite::{params, Connection};
use std::collections::HashMap;

use anyhow::{ensure, Result};
use osmio::changesets::{ChangesetTagReader};
use std::env::args;
use std::path::PathBuf;

fn main() -> Result<()> {
    let changeset_filename = args().nth(1).expect("provide osc filename as arg 1");
    let sqlite_filename = PathBuf::from(args().nth(2).expect("provide sqlite filename as arg 1"));

    //let osc = ChangesetReader::from_filename(&changeset_filename)?;
    let osc = ChangesetTagReader::from_filename(&changeset_filename)?;

    ensure!(
        !sqlite_filename.exists(),
        "Sqlite filename {} already exists",
        sqlite_filename.to_str().unwrap()
    );
    let mut conn = Connection::open(sqlite_filename)?;

    let _tag_columns = [
        "imagery_used",
        "locale",
        "source",
        "host",
        "changesets_count",
    ];

    conn.execute(
        "CREATE TABLE changeset_tags (
                  id              INTEGER PRIMARY KEY,
                  -- imagery_used            TEXT NOT NULL,
                  -- locale            TEXT NOT NULL,
                  -- source            TEXT NOT NULL,
                  -- host            TEXT NOT NULL,
                  -- changesets_count            TEXT NOT NULL,
                  other_tags            TEXT NOT NULL
          )",
        [],
    )?;

    let txn = conn.transaction()?;

    let mut cid;
    let mut tags: Vec<(String, String)>;
    let mut tags_json: Vec<u8> = Vec::new();
    let mut changeset;
    let mut tag_popularity: HashMap<String, usize> = HashMap::new();
    let mut num_changesets_with_tags = 0;

    //let mut imagery_used;
    //let mut locale;
    //let mut source;
    //let mut host;
    //let mut changesets_count;

    for (state, changeset_res) in osc
        .into_iter()
        .optional_progress(10000)
        .assume_size(110_000_000)
    {
        if let Some(state) = state {
            state.do_every_n_sec(2., |state| {
                println!(
                    "{:?}s {}k / {:.1}% done. eta: {} sec {:.0} per sec",
                    state.duration_since_start().as_secs(),
                    state.num_done() / 1000,
                    state.percent().unwrap_or(0.),
                    state
                        .eta()
                        .map_or_else(|| "N/A".to_string(), |d| d.as_secs().to_string()),
                    state.rate()
                );
            });
        }
        changeset = changeset_res?;
        cid = changeset.0;
        tags = changeset.1;
        num_changesets_with_tags += 1;


        //imagery_used = all_tags
        //    .remove("imagery_used")
        //    .unwrap_or_else(|| "".to_string());
        //locale = all_tags.remove("locale").unwrap_or_else(|| "".to_string());
        //source = all_tags.remove("source").unwrap_or_else(|| "".to_string());
        //host = all_tags.remove("host").unwrap_or_else(|| "".to_string());
        //changesets_count = all_tags
        //    .remove("changesets_count")
        //    .unwrap_or_else(|| "".to_string());
        //tags = all_tags.into_iter().collect();

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
            //"INSERT INTO changeset_tags (id, imagery_used, locale, source, host, changesets_count, other_tags) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            //params![cid, imagery_used, locale, source, host, changesets_count, tags_json],
            "INSERT INTO changeset_tags (id, other_tags) VALUES (?1, ?2)",
            params![cid, tags_json],
        )?;
    }
    txn.commit()?;

    println!(
        "Inserted {} changesets, and got {} unique tags",
        num_changesets_with_tags,
        tag_popularity.len()
    );
    let mut tag_popularity = tag_popularity
        .into_iter()
        .map(|(k, v)| (v, k))
        .collect::<Vec<(usize, String)>>();
    tag_popularity.sort();
    tag_popularity.reverse();
    for (num, key) in tag_popularity.iter().take(100) {
        if num * 1000 < num_changesets_with_tags {
            break;
        }
        println!(
            "{:>8} {:>3}% tagged {}",
            num,
            (num * 100) / num_changesets_with_tags,
            key
        );
    }

    Ok(())
}
