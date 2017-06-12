#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    ssid: String,
    portal: String,
    field_user: String,
    field_pwd: String,
    username: String,
    password: String,
    auto_login: u8,
    wifi_mode: u8,
}

impl Configuration {
    pub fn new(ssid: &str, portal: &str, field_user: &str, field_pwd: &str,
    username: &str, password: &str,
    auto_login: u8, wifi_mode: u8) -> Configuration {
        Configuration {
            ssid: ssid.to_owned(),
            portal: portal.to_owned(),
            field_user: field_user.to_owned(),
            field_pwd: field_pwd.to_owned(),
            username: username.to_owned(),
            password: password.to_owned(),
            auto_login: auto_login,
            wifi_mode: wifi_mode,
        }
    }
    pub fn get_ssid(&self) -> &str {
        &self.ssid   
    }
    pub fn get_portal(&self) -> &str {
        &self.portal
    }
    pub fn get_field_user(&self) -> &str {
        &self.field_user
    }
    pub fn get_field_pwd(&self) -> &str {
        &self.field_pwd
    }
    pub fn get_username(&self) -> &str {
        &self.username
    }
    pub fn get_password(&self) -> &str {
        &self.password
    }
    pub fn get_auto_login(&self) -> u8 {
        self.auto_login
    }
    pub fn get_wifi_mode(&self) -> u8 {
        self.wifi_mode
    }
}
