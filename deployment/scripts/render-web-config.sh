#!/bin/sh
set -eu

# These upstreams are substituted into default.conf.template below. If either
# is unset, envsubst renders a scheme-less proxy_pass (e.g. "proxy_pass /...;")
# and nginx aborts at startup with a cryptic "invalid URL prefix". Require them
# explicitly so the failure names the missing variable instead.
: "${WEB_ESPLORA_API_UPSTREAM:?WEB_ESPLORA_API_UPSTREAM must be set}"
: "${WEB_PRICES_API_UPSTREAM:?WEB_PRICES_API_UPSTREAM must be set}"

envsubst '${WEB_ESPLORA_API_UPSTREAM} ${WEB_PRICES_API_UPSTREAM}' \
  < /deployment/configs/nginx/default.conf.template \
  > /etc/nginx/conf.d/default.conf

# Validate the rendered config before handing off to nginx, so a bad render
# fails fast with a clear message rather than crash-looping the container.
nginx -t

exec "$@"
