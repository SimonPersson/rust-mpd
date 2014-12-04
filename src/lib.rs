#![feature(macro_rules, slicing_syntax, if_let)]

extern crate libc;
extern crate time;

pub mod common;
pub mod connection;
pub mod settings;
pub mod status;
pub mod outputs;
pub mod tags;
pub mod songs;
pub mod playlists;


#[test]
fn test_conn() {
    //let c = MpdConnection::new(Some("192.168.1.10"), 6600);
    let c = MpdConnection::new(None, 6600);
    let mut conn = match c {
        None => panic!("connection is None"),
        Some(Err(e)) => panic!("connection error: {}", e),
        Some(Ok(c)) => c
    };

    println!("{}", conn.stop());
    println!("{}", conn.set_volume(0));
    println!("{}", conn.settings());

    let mut playlist: MpdResult<Playlist> = Err(MpdError::Other { kind: MpdErrorKind::Success, desc: "".to_string() });

    for pl in conn.playlists().unwrap() {
        println!("{}", pl);
        playlist = pl;
    }

    for s in playlist.unwrap().songs(&mut conn).unwrap() {
        println!("{}", s);
    }

    for o in conn.outputs().unwrap() {
        println!("{}", o);
    }

    panic!("{}", conn.current_song());
}

//#[test]
//fn test_live_status() {
    //let mut conn = MpdConnection::new("192.168.1.10:6600").unwrap();
    //panic!("{}", conn.status());
//}

//#[test]
//fn test_live_stats() {
    //let mut conn = MpdConnection::new("192.168.1.10:6600").unwrap();
    //panic!("{}", conn.stats());
//}

//#[test]
//fn test_live_search() {
    //let mut conn = MpdConnection::new("192.168.1.10:6600").unwrap();
    //panic!("{}", conn.search("file", ""));
//}
