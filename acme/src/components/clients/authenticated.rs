/*
    Appellation: authenticated <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum AuthClient {
    Basic {
        username: String,
        password: String
    },
    OAuth {
        access_token: String,
        username: String,
        hashed_password: String
    }
}


#[cfg(test)]
mod tests {
    use super::AuthClient;


    #[test]
    fn test_auth_client_default() {
        let a = AuthClient::Basic { username: "".to_string(), password: "".to_string() };
        let b = a.clone();
        assert_eq!(a, b)
    }

}