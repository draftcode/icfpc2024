# VM provisioning

* Debian 12 bookworm
  * sudo su
  * apt install nginx
  * apt install git
  * apt install snapd
  * apt install build-essential
  * Install docker https://docs.docker.com/engine/install/debian/
  * curl -sSf https://rye.astral.sh/get | bash
  * Add `. "$HOME/.rye/env"` to .bashrc and modify .profile
  * curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  * snap install --classic certbot
  * ln -s /snap/bin/certbot /usr/bin/certbot
  * Copy the deploy key to ~/.ssh/id_ed25519. chmod 600
  * cd && git clone git@github.com:draftcode/icfpc2024.git
  * cd icfpc2024 && rye sync
  * cd backend_py && docker compose up -d
  * cp /root/icfpc2024/production/nginx-sites.conf /etc/nginx/sites-available/default
  * ln -s /root/icfpc2024/production/backend_py.service /etc/systemd/system/backend_py.service
  * systemctl daemon-reload
  * systemctl enable --now backend_py


* Cloudflare DNS setup
  * Set up a new A record for the domain
