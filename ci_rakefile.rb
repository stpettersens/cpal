#
# Rakefile to create a dummy configuration file for `cpal` 
# in a CI enviroment such as Travis CI.
#
require 'json'

Configuration = Struct.new(:ssid, :portal, :username, :password, :auto_login, :wifi_mode)

task :data do #[ignore]
    config = Configuration.new("DUMMY_ID", "1.1.1.1", "root", "toor", 1, 1)
    json = File.open(".cpal.json", "w")
    json.write(config.to_h.to_json)
    puts config.to_h.to_json
end
