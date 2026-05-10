#!/usr/bin/env fish

argparse \
    'h/host='\
    'p/path=' \
    -- $argv
or exit 1

if not set -q _flag_host
    echo 'err:host not defied'
    exit 1
end

if not set -q _flag_path
    echo 'err:path not defined'
    exit 1
end


set path $_flag_path
set host $_flag_host

