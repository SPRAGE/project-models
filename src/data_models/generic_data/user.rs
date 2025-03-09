use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;
use time::macros::format_description;
use redis::ToRedisArgs;
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMeta {
    pub demat_consent: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub access_token: String,
    pub api_key: String,
    pub avatar_url: Option<String>,
    pub broker: String,
    pub email: String,
    pub enctoken: String,
    pub exchanges: Vec<String>,
    pub login_time: PrimitiveDateTime,
    pub meta: UserMeta,
    pub order_types: Vec<String>,
    pub products: Vec<String>,
    pub public_token: String,
    pub refresh_token: String,
    pub user_id: String,
    pub user_name: String,
    pub user_shortname: String,
    pub user_type: String,
}

impl User {
    pub fn from_json(json: Value) -> Result<User, String> {
        let avatar_url = json["avatar_url"].as_str().map(|s| s.to_string());
        // let avatar_url = match avatar_url {
        //     Some(avatar_url) => Some(avatar_url.to_string()),
        //     None => None,
        // };

        let date_format = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
        let login_time = match json["login_time"].as_str() {
            Some(login_time) => {
                let login_time = PrimitiveDateTime::parse(login_time, &date_format);
                match login_time {
                    Ok(login_time) => login_time,
                    Err(e) => {
                        return Err(format!("Error parsing login_time: {}", e));
                    }
                }
            }
            None => {
                return Err("login_time is missing".to_string());
            }
        };

        Ok(User {
            user_type: json["user_type"].as_str().unwrap().to_string(),
            email: json["email"].as_str().unwrap().to_string(),
            user_name: json["user_name"].as_str().unwrap().to_string(),
            user_shortname: json["user_shortname"].as_str().unwrap().to_string(),
            broker: json["broker"].as_str().unwrap().to_string(),
            exchanges: serde_json::from_value(json["exchanges"].clone()).unwrap(),
            products: serde_json::from_value(json["products"].clone()).unwrap(),
            order_types: serde_json::from_value(json["order_types"].clone()).unwrap(),
            avatar_url,
            user_id: json["user_id"].as_str().unwrap().to_string(),
            api_key: json["api_key"].as_str().unwrap().to_string(),
            access_token: json["access_token"].as_str().unwrap().to_string(),
            public_token: json["public_token"].as_str().unwrap().to_string(),
            enctoken: json["enctoken"].as_str().unwrap().to_string(),
            refresh_token: json["refresh_token"].as_str().unwrap().to_string(),
            login_time,
            meta: serde_json::from_value(json["meta"].clone()).unwrap(),
        })
    }
}

impl ToRedisArgs for User {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        // Write each field as a separate hash field
        out.write_arg("user_type".as_bytes());
        out.write_arg(self.user_type.as_bytes());

        out.write_arg("email".as_bytes());
        out.write_arg(self.email.as_bytes());

        out.write_arg("user_name".as_bytes());
        out.write_arg(self.user_name.as_bytes());

        out.write_arg("user_shortname".as_bytes());
        out.write_arg(self.user_shortname.as_bytes());

        out.write_arg("broker".as_bytes());
        out.write_arg(self.broker.as_bytes());

        out.write_arg("exchanges".as_bytes());
        out.write_arg(
            serde_json::to_string(&self.exchanges)
                .unwrap()
                .as_bytes(),
        );

        out.write_arg("products".as_bytes());
        out.write_arg(
            serde_json::to_string(&self.products)
                .unwrap()
                .as_bytes(),
        );

        out.write_arg("order_types".as_bytes());
        out.write_arg(
            serde_json::to_string(&self.order_types)
                .unwrap()
                .as_bytes(),
        );

        //aVATAR URL CAN BE NULL

        // out.write_arg("avatar_url".as_bytes());
        // out.write_arg(self.avatar_url.as_bytes());

        out.write_arg("user_id".as_bytes());
        out.write_arg(self.user_id.as_bytes());

        out.write_arg("api_key".as_bytes());
        out.write_arg(self.api_key.as_bytes());

        out.write_arg("access_token".as_bytes());
        out.write_arg(self.access_token.as_bytes());

        out.write_arg("public_token".as_bytes());
        out.write_arg(self.public_token.as_bytes());

        out.write_arg("enctoken".as_bytes());
        out.write_arg(self.enctoken.as_bytes());

        out.write_arg("refresh_token".as_bytes());
        out.write_arg(self.refresh_token.as_bytes());

        out.write_arg("login_time".as_bytes());
        out.write_arg(self.login_time.to_string().as_bytes());

        out.write_arg("meta".as_bytes());
        out.write_arg(serde_json::to_string(&self.meta).unwrap().as_bytes());
    }
}

