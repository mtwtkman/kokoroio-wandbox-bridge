$cmd = $Args[0]
$opt = $Args[1]
$name = "kokoro-eval"

$d = "docker exec -ti $name"

function convert_with_path($cmd)
{
    $DOCKER_CONVERT_WINDOWS_PATHS = 1
    iex $cmd
}

switch ($cmd)
{
    "docker-build" {convert_with_path "docker build -t $name ."}
    "up" {convert_with_path "docker run -ti --rm -d --name $name -v ${PWD}:/app -p 55301:55301 $name sh"}
    "run" {convert_with_path "docker exec -ti -e RUST_BACKTRACE=1 -e KOKORO_IO_ACCESS_TOKEN=${access_token} -e KOKORO_IO_CALLBACK_SECRET=${callback_secret} $name cargo run"}
    "build" {convert_with_path "$d cargo build"}
    "cargo" {convert_with_path "$d cargo $opt"}
    "stop" {convert_with_path "docker stop $name"}
    default {convert_with_path "$d $opt"}
}