use serde;

#[derive(serde::Deserialize)]
struct Joke {
    joke: String,
}

pub async fn send_joke() -> Result<String, reqwest::Error> {
    let res = reqwest::get("https://v2.jokeapi.dev/joke/Miscellaneous,Dark,Christmas?blacklistFlags=religious,political&type=single").await?;
    let body = res.json::<Joke>().await?;

    Ok(body.joke)
}
