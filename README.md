# Valinor

![Work in Progress](https://img.shields.io/badge/Status-Work%20In%20Progress-yellow)
![Pre-Alpha](https://img.shields.io/badge/Release-Pre--Alpha-red)
![version](https://img.shields.io/badge/version-0.0.1-blue)
![License](https://img.shields.io/badge/license-MPL%202.0-brightgreen)
![Rust](https://img.shields.io/badge/Language-Rust-orange)

File sequester analyzer

**V**érification et **A**na**L**yse des **IN**formations et **O**bjets **R**appatriés

Valinor is a tool designed to identify and process various file types based on their contents. It is made to process file sequesters owned by your company and provide informations about the files such as :
- The text content in the case of a PDF, Word Document, raw text format, etc.
- The hash of the file in any cases

A plug-in to push parsed text in an Elasticsearch instance is currently under development.

Built with Rust, this project aims to provide optimal performance while staying scalable.


## Project 

### Structure 
```
├── Cargo.toml
├── README.md
├── setup.sh
├── src
│ ├── analyzers # Modules for content analysis
│ ├── frontend # User interface and web server
│ ├── internals # Internal processing, file identification
│ ├── main.rs # Application entry point
│ ├── parsers # Modules for parsing different file formats
│ └── utils # Tools and utilities, e.g., logging
└── valinor.yaml # Configuration file
```
### Early view (0.0.1)
![Pre Alpha View](./docs/img/pre_alpha_view.png)


## Features

- File type identification based on content
- Web server for user interaction
- File processing and pertinent information extraction
- Efficient and configurable logging
- Capability to process entire directories for analysis
  
## Quick Start

1. Clone the repository:

```bash
git clone https://github.com/aristot3/valinor/ && cd valinor
chmod +x ./setup.sh
sudo ./setup.sh
```

2. Configure:
```bash
vi /etc/valinor/valinor.yml
```

3. Run :

```bash
systemctl start valinor
```

## License

This project is licensed under the [Mozilla Public License 2.0 (MPL 2.0)](LICENSE). Please refer to the [LICENSE](LICENSE) file for detailed information.