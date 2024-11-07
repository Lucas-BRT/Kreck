use kenku_control::Controller;
use rocket::{get, put, State};
use serde_json::{json, Value};

#[get("/sounds")]
pub async fn get(controller: &State<Controller>) -> Result<Value, ()> {
    match controller.get_soundboard().await {
        Ok(sounds) => Ok(json!(sounds)),
        Err(e) => Err(println!("{e}")),
    }
}

#[put("/sounds/play/<sound_id>")]
pub async fn play(controller: &State<Controller>, sound_id: String) {
    match controller.get_soundboard().await {
        Ok(sounds) => {
            for sound in sounds.sounds {
                if sound.id == sound_id {
                    sound.play(controller).await.unwrap();
                }
            }
        }
        Err(e) => println!("{e}"),
    }
}

#[put("/sounds/stop/<sound_id>")]
pub async fn stop(controller: &State<Controller>, sound_id: String) {
    match controller.get_soundboard().await {
        Ok(sounds) => {
            for sound in sounds.sounds {
                if sound.id == sound_id {
                    sound.stop(controller).await.unwrap();
                }
            }
        }
        Err(e) => println!("{e}"),
    }
}
