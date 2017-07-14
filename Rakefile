require 'os'

target = "cpal"
tp = "target/release/cpal"
srcin = "_main.rs"
srcout = "src/main.rs"
servercmd = "sh test/server.sh"
ppcondition = "USE_CURL_EXT" # or USE_CURL_LIB

if OS.windows? then
    target = "cpal.exe"
    tp = "target\\release\\cpal.exe"
    srcin = "_main.rs"
    srcout = "src\\main.rs"
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

task :configure do
    # Configure dependencies with Full Monkey preprocessor.
    sh "fm --file _Cargo.toml --condition #{ppcondition} --out Cargo.toml"
    sh "fm --file #{srcin} --condition #{ppcondition} --out #{srcout}"
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

task :cleanpp do
    File.delete(srcout)
    File.delete("Cargo.toml")
end

task :server do
    sh "#{servercmd}"
    sleep 5
    sh "curl http://127.0.0.1:2015"
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
