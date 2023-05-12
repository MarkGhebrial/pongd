#/bin/bash

cp pongd.service /etc/systemd/system/pongd.service

systemctl daemon-reload
systemctl enable pongd.service
systemctl start pongd.service