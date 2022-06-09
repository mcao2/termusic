// mod internal_backend;
// mod crossbeam;
// mod symphonia_backend;
#[cfg(all(feature = "gst", not(feature = "mpv")))]
mod gstreamer_backend;
#[cfg(feature = "mpv")]
mod mpv_backend;
mod playlist;
#[cfg(not(any(feature = "mpv", feature = "gst")))]
mod rusty_backend;
use crate::config::Termusic;
use crate::ui::Status;
// #[cfg(not(any(feature = "mpv", feature = "gst")))]
// mod rodio_backend;
use anyhow::Result;
#[cfg(feature = "mpv")]
use mpv_backend::Mpv;
use std::sync::mpsc::Receiver;
// #[cfg(not(any(feature = "mpv", feature = "gst")))]
// use rodio_backend::RodioPlayer;
// use symphonia_backend::Symphonia;
pub use playlist::Playlist;

pub enum PlayerMsg {
    AboutToFinish,
}

pub struct GeneralPl {
    #[cfg(all(feature = "gst", not(feature = "mpv")))]
    player: gstreamer_backend::GStreamer,
    #[cfg(feature = "mpv")]
    player: Mpv,
    // player: RodioPlayer,
    // player: Symphonia,
    // player: crossbeam::Player,
    // player: symphonia_backend::Symphonia,
    #[cfg(not(any(feature = "mpv", feature = "gst")))]
    player: rusty_backend::Player,
    pub message_rx: Receiver<PlayerMsg>,
    pub playlist: Playlist,
    pub status: Status,
}

impl GeneralPl {
    pub fn new(config: &Termusic) -> Self {
        #[cfg(all(feature = "gst", not(feature = "mpv")))]
        let player = gstreamer_backend::GStreamer::new(config);
        #[cfg(feature = "mpv")]
        let player = Mpv::new(config);
        #[cfg(not(any(feature = "mpv", feature = "gst")))]
        let (player, message_rx) = rusty_backend::Player::new(config);
        let mut playlist = Playlist::default();
        if let Ok(p) = Playlist::new(config) {
            playlist = p;
        }
        Self {
            player,
            message_rx,
            playlist,
            status: Status::Stopped,
        }
    }
    pub fn toggle_gapless(&mut self) {
        self.player.gapless = !self.player.gapless;
    }
}

impl GeneralP for GeneralPl {
    fn start_play(&mut self) {
        for track in self
            .playlist
            .as_slice()
            .iter()
            .filter_map(|track| track.file())
        // .flatten()
        {
            self.player.enqueue(track);
        }
    }
    fn add_and_play(&mut self, current_track: &str) {
        self.player.add_and_play(current_track);
    }
    fn volume(&self) -> i32 {
        self.player.volume()
    }
    fn volume_up(&mut self) {
        self.player.volume_up();
    }
    fn volume_down(&mut self) {
        self.player.volume_down();
    }
    fn set_volume(&mut self, volume: i32) {
        self.player.set_volume(volume);
    }
    fn pause(&mut self) {
        self.player.pause();
    }
    fn resume(&mut self) {
        self.player.resume();
    }
    fn is_paused(&self) -> bool {
        self.player.is_paused()
    }
    fn seek(&mut self, secs: i64) -> Result<()> {
        self.player.seek(secs)
    }
    fn get_progress(&mut self) -> Result<(f64, i64, i64)> {
        self.player.get_progress()
    }

    fn set_speed(&mut self, speed: f32) {
        self.player.set_speed(speed);
    }

    fn speed_up(&mut self) {
        self.player.speed_up();
    }

    fn speed_down(&mut self) {
        self.player.speed_down();
    }

    fn speed(&self) -> f32 {
        self.player.speed()
    }

    fn stop(&mut self) {
        self.player.stop();
    }
}

pub trait GeneralP {
    fn start_play(&mut self);
    fn add_and_play(&mut self, current_track: &str);
    fn volume(&self) -> i32;
    fn volume_up(&mut self);
    fn volume_down(&mut self);
    fn set_volume(&mut self, volume: i32);
    fn pause(&mut self);
    fn resume(&mut self);
    fn is_paused(&self) -> bool;
    fn seek(&mut self, secs: i64) -> Result<()>;
    fn get_progress(&mut self) -> Result<(f64, i64, i64)>;
    fn set_speed(&mut self, speed: f32);
    fn speed_up(&mut self);
    fn speed_down(&mut self);
    fn speed(&self) -> f32;
    fn stop(&mut self);
}
