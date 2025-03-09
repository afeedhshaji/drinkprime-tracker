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

### API Endpoints Used

The DrinkPrime filter exposes HTTP endpoints that provide device data:

* `GET` `http://<filter-ip>/getValidity` – Returns JSON with the dispensed value (total water dispensed in milliliters

### Roadmap

* Build a interactive UI to visualize and filter the water usage