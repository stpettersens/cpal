#!/bin/sh
cd test
php-cgi -b 9001 | caddy
