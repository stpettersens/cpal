#!/bin/sh
cd test
daemonize -c $(pwd) /usr/bin/php-cgi -b 9001
daemonize -c $(pwd) /usr/local/bin/caddy
