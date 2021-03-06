#
# Rakefile to create a dummy configuration file for `cpal` 
# in a CI environment such as Travis CI.
#
require 'json'

Config = Struct.new(:ssid, :portal, :field_user, :field_pwd, :username, :password, :auto_login, :wifi_mode)

task :data do
    config = Config.new("DUMMY_ID", "127.0.0.1:2015/login.php", "bs_username", "bs_password", "root", "toor", 1, 1)
    json = File.open(".cpal.json", "w")
    json.write(config.to_h.to_json)
    puts config.to_h.to_json
end
