use std::env;

pub struct Notifier {
    from_number: String,
    account_sid: String,
    auth_token: String,
}

impl Notifier {
    pub fn new() -> Self {
        Notifier {
            from_number: env::var("TWILIO_NUMBER").unwrap(),
            account_sid: env::var("TWILIO_ACCOUNT_SID").unwrap(),
            auth_token: env::var("TWILIO_AUTH_TOKEN").unwrap(),
        }
    }

    pub async fn notify(&self, numbers: &[String], body: &str) {
        let client = twilio::Client::new(&self.account_sid, &self.auth_token);
        for number in numbers {
            if let Err(e) = client
                .send_message(twilio::OutboundMessage::new(
                    &self.from_number,
                    number,
                    body,
                ))
                .await
            {
                println!("Encountered error {}", e);
            }
        }
    }
}

impl Default for Notifier {
    fn default() -> Self {
        Self::new()
    }
}
