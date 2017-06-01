require 'os'
require 'json'

Configuration = Struct.new(:ssid, :portal, :username, :password, :auto_login, :wifi_mode)

target = "cpal"
tp = "target/release/cpal"

if OS.windows? then
    target = "cpal.exe"
    tp = "target\\release\\cpal.exe"
end

task :default do
    sh "cargo build --release"
end

task :upx => [:default] do
    if File.exists?(target) then
        File.delete(target)
    end
    sh "upx -9 #{tp} -o #{target}"
end

task :clean do
    sh "cargo clean"
end

task :cleanlock do
    File.delete("Cargo.lock")
end

task :cleanconf do
    File.delete(".cpal.json")
end

task :data do
    config = Configuration.new("DUMMY_ID", "1.1.1.1", "sam", "mas", 1, 1)
    json = File.open(".cpal.json", "w")
    json.write(config.to_h.to_json)
    puts config.to_h.to_json
end

task :test do
    sh "#{tp} --help"
    puts
    sh "#{tp} configuration"
    puts 
    sh "#{tp} status"
end
