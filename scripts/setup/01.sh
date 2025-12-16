#!/bin/bash

# Check if script is run as root
if [ "$(id -u)" -ne 0 ]; then
    echo "This script must be run as root" 1>&2
    exit 1
fi

# Enable the "universe" repository.
add-apt-repository --yes universe

# Update all software packages.
apt update


echo "Installing essential packages..."

apt install -y \
    fail2ban \
    curl \
    ufw \
    ca-certificates

echo "Installing Docker..."

# Add Docker's official GPG key
install -m 0755 -d /etc/apt/keyrings
curl -fsSL https://download.docker.com/linux/ubuntu/gpg -o /etc/apt/keyrings/docker.asc
chmod a+r /etc/apt/keyrings/docker.asc

# Add the repository to Apt sources
echo \
  "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.asc] https://download.docker.com/linux/ubuntu \
  $(. /etc/os-release && echo "$VERSION_CODENAME") stable" | \
  tee /etc/apt/sources.list.d/docker.list > /dev/null
apt update

# Install Docker packages
apt install -y docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin

# Prompt for username
read -p "Enter the username for the new user: " username

# Check if user already exists
if id "$username" &>/dev/null; then
    echo "User $username already exists. Aborting."
    exit 1
fi

while true; do
    read -s -p "Enter password for $username: " password
    echo
    read -s -p "Confirm password: " password2
    echo
    [ "$password" = "$password2" ] && break
    echo "Passwords do not match. Please try again."
done


# Create new user with sudo privileges
echo "Creating new user: $username"
# Create user and set password non-interactively
useradd -m -s /bin/bash "$username"
echo "$username:$password" | chpasswd

echo "Adding sudo to the user..."
# Add user to sudo group
usermod -aG sudo "$username"

echo "Adding user to docker group..."
# Add user to the docker group.
usermod -aG docker "$username"

# # Prompt for SSH public key
read -p "Enter the SSH public key you want to add for $username: " ssh_key

# Create SSH directory if it doesn't exist
mkdir -p "/home/$username/.ssh"

# Append SSH public key to authorized_keys
echo "$ssh_key" >> "/home/$username/.ssh/authorized_keys"

# Set correct permissions
chown -R "$username:$username" "/home/$username/.ssh"
chmod 700 "/home/$username/.ssh"
chmod 600 "/home/$username/.ssh/authorized_keys"

echo "SSH key added successfully for $username"

# Configure UFW (firewall)
# echo "Configuring firewall..."
# ufw default deny incoming
# ufw default allow outgoing
# ufw allow 22/tcp
# ufw allow 80/tcp
# ufw allow 443/tcp
# echo "y" | ufw enable


echo "Copying fail2ban config..."

# Copy the default jail configuration to jail.local
cp /etc/fail2ban/jail.conf /etc/fail2ban/jail.local

# Restart Fail2Ban to apply changes
service fail2ban restart

# Enable Fail2Ban to start on boot
systemctl enable fail2ban

# Enable docker
systemctl enable docker


echo "
========================================
Setup completed successfully!
----------------------------------------
"

echo "Script complete! Rebooting..."

reboot