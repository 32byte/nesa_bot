# Discord Webhook for Nesa

## What is Nesa?
Nesa is the website of our highschool where we can get information like our upcoming exams

## Why?
This is more of like an exercise for me to practice writing in rust.    
But this also has a use, since it's tedious to log in, go to the right tab, ..., this provides a simple way of viewing the upcoming exams.

## Setting it up

### Creating a webhook
Go to a discord channel you would like the message in and click on settings.    
Go to `Integrations > Webhooks` and create a webhook and copy the webhook url.    
It should look something like this:    
```
https://discord.com/api/webhooks/{your-webhook-id}/{your-webhook-token}
```

### Creating creds.rs
Create a file in the `src/` folder and name it creds.rs    
Then paste the following content inside it:
```rust
#[allow(dead_code)]
pub mod creds {
    pub static USERNAME: &str = "";
    pub static PASSWORD: &str = "";
    pub static WEBHOOK: &str = "";
    pub static TOKEN: &str = "";
    pub static MESSAGE_ID: &str = "";
}
```
Fill in your `username`, `password`, `webhook` and `token`.    

#### Getting the message id
First, open discord and go to `Settings > Appearance` and enable `Developer Mode`.    
Then run:
```bash
cargo run --example send-message
```
This should send a message to the channel you create the webhook in.    
Right-click the message and copy it's id.    
Paste the id into the creds.rs file.

Now you're pretty much done and can run:
```rust
cargo run --release
```

## Contributing
Feel free to contribute by simply making a pull request!