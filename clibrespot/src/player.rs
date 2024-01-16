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

pub async fn create(user: &str, pass: &str) -> Option<Player> {
    let session_config = SessionConfig::default();
    let player_config = PlayerConfig::default();
    let audio_format = AudioFormat::default();
    let credentials = Credentials::with_password(user, pass);
    let backend = audio_backend::find(None).unwrap();

    println!("Connecting...");
    let connect_result = Session::connect(session_config, credentials, None, false).await;

    if let Err(e) = connect_result {
        println!("Error connecting: {}", e);
        return None;
    }

    let (session, _) = connect_result.ok()?;

    let (player, _channel) = Player::new(player_config, session, Box::new(NoOpVolume), move || {
        backend(None, audio_format)
    });

    Some(player)
}

pub fn play(player: &mut Player, track_id: &str) {
    let Ok(mut track) = SpotifyId::from_base62(track_id) else { return };
    track.audio_type = SpotifyAudioType::Track;

    player.load(track, true, 0);
    player.play();
}
