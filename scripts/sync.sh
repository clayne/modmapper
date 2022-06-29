#!/bin/bash
export $(grep -v '^#' .env | xargs -d '\n')
rclone sync --fast-list --checksum cells ${STATIC_SERVER_REMOTE}:${STATIC_SERVER_BUCKET}/cells
rclone sync --fast-list --checksum mods ${STATIC_SERVER_REMOTE}:${STATIC_SERVER_BUCKET}/mods
rclone sync --fast-list --checksum plugins_data ${STATIC_SERVER_REMOTE}:${STATIC_SERVER_BUCKET}/plugins_data
rclone sync --fast-list --checksum files ${STATIC_SERVER_REMOTE}:${STATIC_SERVER_BUCKET}/files