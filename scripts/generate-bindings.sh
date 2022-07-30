#!/bin/sh -e

vorbis_header_wrapper=$(mktemp)
cat <<VORBIS_HEADER_WRAPPER >"$vorbis_header_wrapper"
#include <vorbis/codec.h>
#include <vorbis/vorbisenc.h>
#include <vorbis/vorbisfile.h>
VORBIS_HEADER_WRAPPER
trap 'rm -f "$vorbis_header_wrapper"' EXIT INT QUIT TERM

bindgen --disable-header-comment --allowlist-function='ogg.*' \
--allowlist-type='ogg.*' \
--blocklist-item='__.*' \
--size_t-is-usize ogg_vendor/include/ogg/ogg.h -- -Iogg_vendor/include \
> packages/ogg_sys/src/bindings.rs

bindgen --disable-header-comment \
--allowlist-function='vorbis_.*' --allowlist-function='ov_.*' \
--blocklist-function='ov_open' --blocklist-function='ov_fopen' --blocklist-function='ov_test' \
--allowlist-type='vorbis_.*' --allowlist-type='ovectl.*' --allowlist-type='OggVorbis.*' \
--allowlist-var='OV_.*' --allowlist-var='NOTOPEN' --allowlist-var='PARTOPEN' \
--allowlist-var='OPENED' --allowlist-var='STREAMSET' --allowlist-var='INITSET' \
--blocklist-item='_IO.*' --blocklist-item='FILE' \
--blocklist-item='ogg_.*' \
--blocklist-item='__.*' \
--size_t-is-usize "$vorbis_header_wrapper" \
-- -Ivorbis_vendor/include -Ivorbis_vendor/lib -Iogg_vendor/include -DOV_EXCLUDE_STATIC_CALLBACKS \
> packages/vorbis_sys/src/bindings.rs
