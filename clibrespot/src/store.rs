use std::fmt;

use librespot::playback::player::{Player, PlayerEvent, PlayerEventChannel};
use tokio::{runtime::Runtime, sync::{mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender}, Mutex}};

use crate::player;

#[derive(Default)]
pub struct State {
    pub is_playing: bool,
    pub is_track_loaded: bool,
    player: Option<Player>,
    listeners: Vec<UnboundedSender<Vec<StateChange>>>,
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.debug_struct("PlayerState")
            .field("is_playing", &self.is_playing)
            .field("is_track_loaded", &self.is_track_loaded)
            .finish()
    }
}

pub enum Action {
    Created { player: Player },
    PlayTrack { track_id: String },
    Pause,
    Resume,
    Stopped,
}

impl fmt::Debug for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let mut ds = f.debug_struct(&format!("Action::{}", match self {
            Action::Created {..} => "Created",
            Action::PlayTrack {..} => "PlayTrack",
            Action::Pause => "Pause",
            Action::Resume => "Resume",
            Action::Stopped => "Stopped",
        }));

        match self {
            Action::PlayTrack { track_id } => ds.field("track_id", track_id),
            _ => &mut ds,
        }.finish()
    }
}

#[derive(Copy, Clone, Debug)]
pub enum StateChange {
    IsPlaying(bool),
    IsTrackLoaded(bool),
}

impl State {
    fn get_player_event_channel(&self) -> Option<PlayerEventChannel> {
        Some(self.player.as_ref()?.get_player_event_channel())
    }

    fn handle(&mut self, action: Action) {
        println!("handling action: {:?}", action);

        let state_changes = match action {
            Action::Created { player } => {
                if let Some(player) = self.player.as_ref() {
                    player.stop();
                }

                self.player = Some(player);

                vec![
                    StateChange::IsPlaying(false),
                    StateChange::IsTrackLoaded(false),
                ]
            },
            Action::PlayTrack { track_id } => {
                match self.player.as_mut() {
                    Some(player) => {
                        player::play(player, &track_id);

                        vec![
                            StateChange::IsPlaying(true),
                            StateChange::IsTrackLoaded(true),
                        ]
                    },
                    None => {
                        println!("attepted to play song without a player");
                        vec![]
                    }
                }
            },
            Action::Pause => {
                match self.player.as_ref() {
                    Some(player) => {
                        player.pause();

                        vec![
                            StateChange::IsPlaying(false),
                        ]
                    },
                    None => {
                        println!("attempted to pause without a player");
                        vec![]
                    }
                }
            },
            Action::Resume => {
                if self.is_track_loaded {
                    self.player.as_ref().unwrap().play();

                    vec![
                        StateChange::IsPlaying(true),
                    ]
                } else {
                    println!("attempted to resume without a loaded track");
                    vec![]
                }
            },
            Action::Stopped => {
                vec![
                    StateChange::IsPlaying(false),
                    StateChange::IsTrackLoaded(false)
                ]
            }
        };

        for change in state_changes.iter() {
            match change {
                StateChange::IsPlaying(is_playing) => {
                    self.is_playing = *is_playing
                },
                StateChange::IsTrackLoaded(is_track_loaded) => {
                    self.is_track_loaded = *is_track_loaded
                },
            }

        }

        let listeners: &Vec<UnboundedSender<Vec<StateChange>>> = self.listeners.as_ref();
        println!("notifying {} listeners of {} events...", listeners.len(), state_changes.len());

        for sender in listeners.iter() {
            sender.send(state_changes.to_vec()).ok();
        }
    }
}

pub struct ViewModel {
    pub runtime: Runtime,
    state: Mutex<State>,
}

pub fn create_view_model() -> ViewModel {
    let runtime = Runtime::new().unwrap();
    let state: Mutex<State> = Mutex::new(State {
        is_playing: false,
        is_track_loaded: false,
        player: None,
        listeners: vec!(),
    });

    return ViewModel {
        runtime,
        state,
    };
}

impl ViewModel {
    pub fn create_player(&'static self, user: &str, pass: &str) {
        self.runtime.block_on(self._create_player(user, pass));
        self.runtime.spawn(self._handle_player_events());
    }

    async fn _create_player(&self, user: &str, pass: &str) {
        let Some(player) = player::create(user, pass).await else { return; };

        let action = Action::Created { player };
        self.state.lock().await.handle(action);
    }

    async fn _handle_player_events(&self) {
        let Some(mut channel) = self.state.lock().await.get_player_event_channel() else {
            println!("failed to listen to player events, did the player fail to initialize?");
            return
        };

        while let Some(event) = channel.recv().await {
            match event {
                PlayerEvent::Stopped {..} | PlayerEvent::EndOfTrack {..} => {
                    println!("handing player event! {:?}", event);
                    let mut state = self.state.lock().await;
                    state.handle(Action::Stopped);
                },
                _ => {}
            }
        }
    }

    pub async fn listen(&mut self) -> UnboundedReceiver<Vec<StateChange>> {
        let mut state = self.state.lock().await;
        let (tx, rx) = unbounded_channel::<Vec<StateChange>>();
        state.listeners.append(&mut vec![tx]);
        println!("created listener");

        return rx;
    }

    pub async fn play_track(&self, track_id: String) {
        self.state.lock().await.handle(Action::PlayTrack { track_id });
    }

    pub fn play_track_sync(&self, track_id: String) {
        self.runtime.block_on(self.play_track(track_id));
    }

    pub async fn pause(&self) {
        self.state.lock().await.handle(Action::Pause {});
    }

    pub fn pause_sync(&self) {
        self.runtime.block_on(self.pause());
    }

    pub async fn resume(&self) {
        self.state.lock().await.handle(Action::Resume {});
    }

    pub fn resume_sync(&self) {
        self.runtime.block_on(self.resume());
    }
}

