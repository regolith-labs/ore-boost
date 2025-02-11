use anyhow::Result;
use reqwest::Client;

/// notify admin on error
pub async fn notify() -> Result<()> {
    // configure auth
    let account_sid = account_sid()?;
    let auth_token = auth_token()?;
    // configure sender and receiver
    let from = from_address()?;
    let to = to_address()?;
    // build endpoint
    let url = format!(
        "https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json",
        account_sid
    );
    // build payload
    let form_params = [
        ("To", to),
        ("From", from),
        (
            "ContentSid",
            "HXb5b62575e6e4ff6129ad7c8efe1f983e".to_string(),
        ),
        ("ContentVariables", r#"{"1":"2/1","2":"3pm"}"#.to_string()),
    ];
    // POST
    let client = Client::new();
    let response = client
        .post(&url)
        .basic_auth(account_sid, Some(auth_token)) // username = SID, password = AuthToken
        .form(&form_params)
        .send()
        .await?;
    // log response
    let status = response.status();
    let text = response.text().await?;
    println!("notifier status: {}", status);
    println!("notifier response: {}", text);
    Ok(())
}

fn account_sid() -> Result<String> {
    let string = std::env::var("TWILIO_ACCOUNT_SID")?;
    Ok(string)
}

fn auth_token() -> Result<String> {
    let string = std::env::var("TWILIO_AUTH_TOKEN")?;
    Ok(string)
}

fn from_address() -> Result<String> {
    let string = std::env::var("TWILIO_FROM")?;
    let string = format!("whatsapp:{}", string);
    Ok(string)
}

fn to_address() -> Result<String> {
    let string = std::env::var("TWILIO_TO")?;
    let string = format!("whatsapp:{}", string);
    Ok(string)
}
