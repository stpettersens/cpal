require 'os'

target = "cpal"
tp = "target/release/cpal"
servercmd = "sh test/server.sh"

if OS.windows? then
    target = "cpal.exe"
    tp = "target\\release\\cpal.exe"
    servercmd = "test\\server.cmd"
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

task :server do
    sh "#{servercmd}"
    sleep 3
    sh "curl --data bs_username=root&bs_password=toor http://127.0.0.1:2015/login.php"
end

task :test do
    sh "#{tp} --help"
    puts
    sh "#{tp} configuration"
    puts 
    sh "#{tp} login"
    puts
    sh "#{tp} status"
end
