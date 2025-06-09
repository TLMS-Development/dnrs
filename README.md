# dnrs

**WIP**: This project is a work in progress and is not yet ready for production use.

**dnrs** is a DNS updater. It is designed to update DNS records for various providers, or custom providers. The tool supports configuration via environment variables, a configuration file, and command-line arguments.

## Supported Providers

- **Nitrado**
- **Netcup**

## Custom Providers

If your provider is not supported, you can implement your own provider by defining a custom HTTP request for DNS updates. This allows you to integrate with any DNS service that provides an API.

## Features

- **Dynamic DNS updates:** Update DNS record based on your public IP address. Configuration is flexible. Supports both IPv4 and IPv6. Supports raw and JSON-based responses.
- **Custom provider support:** Define your own HTTP requests for DNS updates.
- **Flexible configuration:** Use config files and/or environment variables.
- **Structured logging** with colored output. Yay 🎉

## Configuration

Documentation coming soon™
