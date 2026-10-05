# Copyright 2026 Pranav R S.
# Use of this source code is governed by the MIT license that can be found in the LICENSE file.

FROM alpine:3.24
RUN apk add --no-cache sqlite

# Copy and run the backup script.
COPY --chmod=755 backup.sh /backup.sh
CMD ["/backup.sh"]