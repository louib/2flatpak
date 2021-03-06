#!/usr/bin/env bash
# Get the packages of type "source" from the most common
# debian and debian derivative repositories.

die() { echo "🔥 Error: $*" 1>&2; exit 1; }

set -e

output_dir="$PB_OUT_DIR"
if [[ -z "$output_dir" ]]; then
    die "You must define the PB_OUT_DIR variable."
fi

if [[ ! -d "$output_dir" ]]; then
    die "$output_dir is not a directory!"
fi

if [[ ! -f "$output_dir/sources.txt" ]]; then
    wget --no-check-certificate http://us.archive.ubuntu.com/ubuntu/dists/devel/main/source/Sources.gz
    gzip -d Sources.gz
    mv Sources "$output_dir/ubuntu_devel_main_sources.txt"

    wget --no-check-certificate http://us.archive.ubuntu.com/ubuntu/dists/devel/universe/source/Sources.gz
    gzip -d Sources.gz
    mv Sources "$output_dir/ubuntu_devel_universe_sources.txt"

    wget --no-check-certificate http://us.archive.ubuntu.com/ubuntu/dists/devel/multiverse/source/Sources.gz
    gzip -d Sources.gz
    mv Sources "$output_dir/ubuntu_devel_multiverse_sources.txt"

    wget --no-check-certificate https://ftp.nl.debian.org/debian/dists/sid/main/source/Sources.gz
    gzip -d Sources.gz
    mv Sources "$output_dir/debian_sid_main_sources.txt"

    wget --no-check-certificate https://repo.pureos.net/pureos/dists/green/main/source/Sources.xz
    unxz -d Sources.xz
    mv Sources "$output_dir/pureos_green_main_sources.txt"

    wget --no-check-certificate https://repo.pureos.net/pureos/dists/landing/main/source/Sources.xz
    unxz -d Sources.xz
    mv Sources "$output_dir/pureos_landing_main_sources.txt"

    wget --no-check-certificate https://repo.pureos.net/pureos/dists/amber/main/source/Sources.xz
    unxz -d Sources.xz
    mv Sources "$output_dir/pureos_amber_main_sources.txt"

    wget --no-check-certificate https://repo.pureos.net/pureos/dists/byzantium/main/source/Sources.xz
    unxz -d Sources.xz
    mv Sources "$output_dir/pureos_byzantium_main_sources.txt"

    cat "$output_dir"/*_sources.txt > "$output_dir/sources.txt"
    rm "$output_dir"/*_sources.txt
    echo "👍 Fetched sources from common debian repos."
else
    echo "👍 No need to fetch sources from common debian repos."
fi
