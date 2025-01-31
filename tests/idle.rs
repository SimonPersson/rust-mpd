extern crate mpdrs;

mod helpers;
use helpers::Daemon;

use mpdrs::Idle;

#[test]
fn idle() {
    let daemon = Daemon::start();
    let mut mpd = daemon.connect();
    let idle = mpd.idle(&[]).unwrap();

    let mut mpd1 = daemon.connect();
    mpd1.consume(true).unwrap();
    mpd1.consume(false).unwrap();

    let sys = idle.get().unwrap();
    assert_eq!(&*sys, &[mpdrs::Subsystem::Options]);
}
