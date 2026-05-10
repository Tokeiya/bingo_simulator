#!/usr/bin/env fish


argparse \
    'h/host=' \
    'p/path=' \
    -- $argv
or exit 1

if not set -q _flag_host
    echo "err: host not defined"
    exit 1
end

set host $_flag_host
set path $_flag_path


if not set -q _flag_path
    set path "repos/" 
end

echo "host = $host"   
echo "path = $path"

rsync -avr --exclude 'target/' --exclude '.idea/' ./rust $host:/home/tokeiya3/$path
