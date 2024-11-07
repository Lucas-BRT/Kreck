use kenku_control::Controller;
use rocket::{put, State};

#[put("/tracks/play/<track_id>")]
pub async fn play(controller: &State<Controller>, track_id: String) {
    match controller.get_playlist().await {
        Ok(tracks) => {
            for track in tracks.tracks {
                if track.id == track_id {
                    track.play(controller).await.unwrap();
                }
            }
        }
        Err(e) => println!("{e}"),
    };
}
