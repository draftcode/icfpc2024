# VM provisioning

* Debian 12 bookworm
  * sudo su
  * apt install nginx
  * apt install git
  * apt install snapd
  * apt install build-essential
  * Install docker https://docs.docker.com/engine/install/debian/
  * curl -sSf https://rye.astral.sh/get | bash
  * Add `. "$HOME/.rye/env"` to .bashrc
  * curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  * snap install --classic certbot
  * ln -s /snap/bin/certbot /usr/bin/certbot
  * Copy the deploy key to ~/.ssh/id_ed25519. chmod 600
  * cd && git clone git@github.com:draftcode/icfpc2024.git
  * cd icfpc2024 && rye sync
  * cd backend_py && docker compose up -d
  * ln -s /root/icfpc2024/production/nginx-sites.conf /etc/nginx/sites-available/default
  * ln -s /root/icfpc2024/production/backend_py.service /etc/systemd/system/backend_py.service
  * systemctl daemon-reload
  * systemctl enable --now backend_py
  * cp /root/icfpc2024/production/id_ed25519_icfpc2024_gha.pub /root/.ssh/authorized_keys
  * Update /etc/ssh/sshd_config
    * PermitRootLogin prohibit-password
  * systemctl reload sshd
  * gcloud auth configure-docker us-central1-docker.pkg.dev
  * cd frontend && docker compose up --detach --force-recreate


* Cloudflare DNS setup
  * Set up a new A record for the domain

* GCP
  * Create workload identity pool
  * Create GCP SA
    * Allow Artifact Registry Writer
  * Create Artifact Registry repository
