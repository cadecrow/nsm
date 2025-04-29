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

## Usage

```
nsm [OPTIONS]
```

### Command Line Arguments

|Argument|Short|Description|Default|
|--------|-----|-----------|-------|
| --project | -p | Path to your Next.js project | Current directory (.) |
| --xml-output  | | Output path for sitemap.xml  | sitemap.xml |
| --json-output | | Output path for sitemap.json | sitemap.json |
| --base-url | -b | Base URL for your website | https://example.com |

## Examples

Generate sitemaps for a Next.js project in the current directory:

```
nsm
```

Specify a different project directory and base URL:

```
next-sitemap-gen --project ./my-nextjs-app --base-url https://mywebsite.com
```

Customize output file paths:

```
next-sitemap-gen --xml-output ./public/sitemap.xml --json-output ./src/data/sitemap.json
```

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
