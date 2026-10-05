# Copyright 2026 Pranav R S.
# Use of this source code is governed by the MIT license that can be found in the LICENSE file.

FROM nginx:stable

COPY nginx/filebase.conf /etc/nginx/snippets/filebase.conf
COPY --chmod=755 static /srv/filebase/static
