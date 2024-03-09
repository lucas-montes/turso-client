# Turso Platform API
A turso client for the platform api endpoints

## Example
```rust
use turso::{
    DatabasesPlatform, TursoClient
};

async fn get_db_url(
    client: &TursoClient<DatabasesPlatform>,
) -> String {
    let default_name = "awesome-db";
    let databases = client.list("my-organization").await.unwrap();
    let default_db = databases.databases.iter().find(|g| g.name == default_name);
    let mut remote = String::from("libsql://");
    if default_db.is_some() {
        remote.push_str(&default_db.unwrap().hostname.to_owned());
    } else {
        let new_db = client
            .create(&self.organization, default_name, &self.group)
            .await
            .unwrap();
        remote.push_str(&new_db.database.hostname.to_owned());
    }
    remote
}

async fn init() {
    let client = TursoClient::new().databases();
    get_db_url(&client).await;
}
```
