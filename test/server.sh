#!/bin/sh
cd test
#daemonize -c $(pwd) php-cgi -b 9001
whereis php-cgi
daemonize -c $(pwd) /usr/local/bin/caddy
