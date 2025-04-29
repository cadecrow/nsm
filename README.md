# nsm

A Next.js Sitemap Generator

A command-line tool written in Rust that scans your Next.js project to automatically generate both XML and JSON sitemaps. The tool analyzes your project's file structure, identifies all routes based on the App Router conventions, and creates sitemap files that can be used for SEO and navigation.

## Features

- Automatically detects routes based on Next.js App Router conventions
- Generates standard sitemap.xml for search engines
- Creates a detailed sitemap.json for building navigation components
- Handles dynamic routes (e.g., `[id]`, `[...slug]`)
- Preserves file modification times for `lastmod` entries

## Installation

### Prereqs

- Rust 1.70.0 or higher

### Building from source

```
# Clone the repository
git clone https://github.com/cadecrow/nsm.git
cd nsm

# Build the project
cargo build --release
```

### Recommended: Install with cargo

from within nsm directory

```
cargo install --path .
```

If you do not use cargo, you can add the Release Directory to your PATH or copy the Binary to a PATH directory.
I will not explain this here. If you do not know what this means but you want to use one of these options, you'd be best served doing some googling or perplexitying to learn what this means.

## Usage

```
nsm [OPTIONS]
```

### Command Line Arguments

| Argument      | Short | Description                  | Default               |
| ------------- | ----- | ---------------------------- | --------------------- |
| --project     | -p    | Path to your Next.js project | Current directory (.) |
| --xml-output  |       | Output path for sitemap.xml  | sitemap.xml           |
| --json-output |       | Output path for sitemap.json | sitemap.json          |
| --base-url    | -b    | Base URL for your website    | https://example.com   |

## Examples

Generate sitemaps for a Next.js project in the current directory:

```
nsm
```

Specify a different project directory and base URL:

```
nsm --project ./my-nextjs-app --base-url https://mywebsite.com
```

Customize output file paths:

```
nsm --xml-output ./public/sitemap.xml --json-output ./src/data/sitemap.json
```

## Configuration

You can create an `nsm.config.json` file in your project root to set default options:
### Example:
```json
{
  "project": ".",
  "xml_output": "public/sitemap.xml",
  "json_output": "src/data/sitemap.json",
  "base_url": "https://mywebsite.com"
}
```
If no configuration file exists, the tool will create one with default values when first run.

Command line arguments will always override settings in the configuration file.


## How This Works

1. When the program starts, it looks for an `nsm.config.json` file
2. If the file exists, it loads the settings
3. If the file doesn't exist, it creates one with default values
4. Command line arguments take precedence over config file settings
5. The merged configuration is used to run the program

This approach gives you flexibility:
- You can set project-specific defaults in the config file
- You can override specific settings via command line when needed
- The config file is automatically created with sensible defaults

The configuration file is particularly useful for CI/CD pipelines or when running the tool regularly, as users won't need to specify the same arguments each time.




## Output Files

### sitemap.xml

```xml
<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <url>
    <loc>https://example.com/</loc>
    <lastmod>2023-04-29T10:42:19Z</lastmod>
  </url>
  <url>
    <loc>https://example.com/about</loc>
    <lastmod>2023-04-28T15:30:00Z</lastmod>
  </url>
</urlset>
```

### sitemap.json

```json
[
  {
    "route": "/",
    "path": "app/page.tsx",
    "label": "Home",
    "description": "",
    "last_modified": "2023-04-29T10:42:19Z"
  },
  {
    "route": "/about",
    "path": "app/about/page.tsx",
    "label": "About",
    "description": "",
    "last_modified": "2023-04-28T15:30:00Z"
  }
]
```

## Notes

- The tool follows Next.js App Router conventions, looking for page.tsx files
- Dynamic routes (with `[brackets]`) are detected and included in the sitemap
- Routes in directories starting with underscore (\_) or inside api directories are excluded
- You can edit the generated JSON to add custom descriptions and labels

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
