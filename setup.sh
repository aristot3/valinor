#!/bin/bash

SERVICE_NAME="valinor"

# Check if the script is run with root privileges
if [[ $EUID -ne 0 ]]; then
   echo "This script must be run as root." 
   exit 1
fi

# Check if git is installed and install if not present
if ! command -v git &> /dev/null; then
    echo "git is not installed. Installing now..."
    apt-get update
    apt-get install -y git
fi

# Check if curl is installed and install if not present
if ! command -v curl &> /dev/null; then
    echo "curl is not installed. Installing now..."
    apt-get update
    apt-get install -y curl
fi

# Install Rust if not present
if ! command -v rustc &> /dev/null; then
    echo "Rust is not installed. Installing now..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source $HOME/.cargo/env
else
    echo "Rust is already installed."
fi

# Install build-essential if not present
if ! dpkg -l | grep -q build-essential; then
    echo "Installing build-essential..."
    apt-get install -y build-essential
else
    echo "build-essential is already installed."
fi

# Compile valinor binary
echo "Compiling valinor..."
if [[ -f "Cargo.toml" ]]; then
    cargo build --release
    if [[ $? -ne 0 ]]; then
        echo "Error compiling valinor. Exiting..."
        exit 1
    fi
else
    echo "Cargo.toml not found. Make sure to run the setup script from the project root directory."
    exit 1
fi

# Copy the compiled binary to /usr/bin
echo "Copying binary to /usr/bin..."
cp target/release/valinor /usr/bin/

# 1. Create a user and group for the service
adduser --system --no-create-home --group $SERVICE_NAME

# 2. Create a directory for the service's logs in /var/log
LOG_DIR="/var/log/$SERVICE_NAME"
if [[ ! -d $LOG_DIR ]]; then
    mkdir $LOG_DIR
    chown $SERVICE_NAME:$SERVICE_NAME $LOG_DIR
else
    echo "Directory $LOG_DIR already exists."
fi

# 3. Set up permissions for your log file
LOG_FILE="$LOG_DIR/valinor.json.log"
if [[ ! -f $LOG_FILE ]]; then
    touch $LOG_FILE
    chown $SERVICE_NAME:$SERVICE_NAME $LOG_FILE
else
    echo "File $LOG_FILE already exists."
fi

# 4. Create configuration directory and file
CONFIG_DIR="/etc/valinor"
CONFIG_FILE="$CONFIG_DIR/valinor.yaml"
if [[ ! -d $CONFIG_DIR ]]; then
    mkdir $CONFIG_DIR
fi

if [[ ! -f $CONFIG_FILE ]]; then
    cp valinor.yaml $CONFIG_DIR/
else
    echo "Configuration file $CONFIG_FILE already exists."
fi

# 5. Configure the systemd service
SYSTEMD_PATH="/etc/systemd/system/$SERVICE_NAME.service"
BINARY_PATH="/usr/bin/valinor"

if [[ ! -f $SYSTEMD_PATH ]]; then
    cat > $SYSTEMD_PATH <<EOL
[Unit]
Description=Valinor Service
After=network.target

[Service]
Type=simple
User=$SERVICE_NAME
Group=$SERVICE_NAME
ExecStart=$BINARY_PATH -f $CONFIG_FILE
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
EOL

    # Reload systemd services and enable Valinor service
    systemctl daemon-reload
    systemctl enable $SERVICE_NAME
    echo "Systemd service configured for $SERVICE_NAME."
else
    echo "Systemd service for $SERVICE_NAME already exists."
fi

# Display a success message
echo "Configuration of curl, git, Rust, build-essential, user, group, logs, config file, and systemd service for $SERVICE_NAME is complete."
