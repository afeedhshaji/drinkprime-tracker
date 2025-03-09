## DrinkPrime Water Usage Tracker

### Overview

This project tracks water usage from a DrinkPrime IoT filter and logs it to a SQLite database. The DrinkPrime app only provides total usage to date, making it difficult to track daily or monthly consumption. This tool addresses that gap by periodically fetching data from the filter's HTTP API and sending daily usage reports to a Discord channel.

### Configuration
Update the following settings in `config.json.sample` and copy it to `config.json`:
```json
{
  "db_file": "/path/to/water_usage.db",
  "filter_ip": "192.168.29.69",
  "discord_webhook": "https://discord.com/api/webhooks/..."
}
```

### Tracker Setup

### Step 1: Build the binary
```bash
cd <repo>
cargo build --release
```
This generates a binary at `./target/release/drinkprime-tracker`.

### Step 2: Populate the configuration file

Use your favourite editor to edit the `config.json.sample` and add your device IP, discord webhook URL etc. Copy the file to `config.json` to be used later when setting up the systemd service.


### Step 3: Create a Systemd Timer for Fetching Data

* Create the service unit (`/etc/systemd/system/water_fetch.service`):

```
[Unit]
Description=Fetch water usage data
After=network.target

[Service]
User=user
ExecStart=<path/to/binary> -c <path/to/config> fetch
```

* Now, create a timer (`/etc/systemd/system/water_fetch.timer`):

```
[Unit]
Description=Run water fetch script every hour

[Timer]
OnCalendar=hourly
Persistent=true

[Install]
WantedBy=timers.target
```
* Enable and start the timer:

```
sudo systemctl daemon-reload
sudo systemctl enable water_fetch.timer
sudo systemctl start water_fetch.timer
```


#### Step 4: Set Up a Systemd Timer for Daily Report

* Create a service file for the report (`/etc/systemd/system/water_report.service`):

```
[Unit]
Description=Send daily water usage report to Discord
After=network.target

[Service]
User=<user>
ExecStart=<path/to/binary> -c <path/to/config> report
```
* Create a timer (`/etc/systemd/system/water_report.timer`):

```
[Unit]
Description=Run daily water usage report

[Timer]
OnCalendar=daily
Persistent=true

[Install]
WantedBy=timers.target
```

* Enable and start
```
sudo systemctl daemon-reload
sudo systemctl enable water_report.timer
sudo systemctl start water_report.timer
```

### API Endpoints Used

The DrinkPrime filter exposes HTTP endpoints that provide device data:

* `GET` `http://<filter-ip>/getValidity` – Returns JSON with the dispensed value (total water dispensed in milliliters

### Roadmap

* Build a interactive UI to visualize and filter the water usage