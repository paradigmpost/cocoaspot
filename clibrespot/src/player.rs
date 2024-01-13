use ffi_helpers::null_pointer_check;
use tokio::runtime::Runtime;
use librespot::{
    core::{
        authentication::Credentials,
        config::SessionConfig,
        session::Session,
        spotify_id::{SpotifyId, SpotifyAudioType},
    },
    playback::{
        audio_backend,
        config::{AudioFormat, PlayerConfig},
        mixer::NoOpVolume,
        player::Player,
    },
};

pub struct Instance {
    _player: Player,
}

// librespot play example
pub fn create(runtime: *mut Runtime, user: &str, pass: &str) -> Option<Instance> {
    null_pointer_check!(runtime);

    let session_config = SessionConfig::default();
    let player_config = PlayerConfig::default();
    let audio_format = AudioFormat::default();
    let credentials = Credentials::with_password(user, pass);
    let backend = audio_backend::find(None).unwrap();

    println!("Connecting...");
    let connect_result = unsafe { (*runtime).block_on(
        Session::connect(session_config, credentials, None, false)
    ) };

    if let Err(e) = connect_result {
        println!("Error connecting: {}", e);
        return None;
    }

    let (session, _) = connect_result.unwrap();

    let (player, _channel) = Player::new(player_config, session, Box::new(NoOpVolume), move || {
        backend(None, audio_format)
    });

    return Some(Instance {
        _player: player,
    });
}

pub fn play_sync(runtime: *mut Runtime, instance: *mut Instance, track_id: &str) {
    null_pointer_check!(runtime);
    null_pointer_check!(instance);

    let mut track = unsafe { SpotifyId::from_base62(track_id).unwrap_unchecked() };
    track.audio_type = SpotifyAudioType::Track;

    unsafe { (*instance)._player.load(track, true, 0) };

    println!("Playing...");
    unsafe { (*runtime).block_on((*instance)._player.await_end_of_track()) };
}

pub fn pause(instance: *mut Instance) {
    null_pointer_check!(instance);

    unsafe { (*instance)._player.pause() }
}

pub fn resume(instance: *mut Instance) {
    null_pointer_check!(instance);

    unsafe { (*instance)._player.play() }
}
