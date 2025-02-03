# rsff

rsff` (scanlation file format) is the core library of an application designed to facilitate the work of teams translating content such as manga, manhwa, manhua, webtoons, etc.

## Some Examples:

```rust
use rsff::{Document, Balloon};

// Create a default document.
let mut d: Document = Document::default();

// Create a default balloon.
let mut b: Balloon = Balloon::default();

// Add content to the balloon.
b.tl_content.push("This is a translation line.".to_string());

// Add balloon to the document.
d.balloons.push(b);
```

## Basic Raw SFF JSON File:

```json
{
  "METADATA_SCRIPT_VERSION": "Scanlation Script File v0.2.0",
  "METADATA_APP_VERSION": "",
  "METADATA_INFO": "Num",
  "balloons": [
    {
      "tl_content": ["num", "nam"],
      "pr_content": ["numnam"],
      "comments": [],
      "btype": "OT",
      "balloon_img": null
    },
    {
      "tl_content": ["num"],
      "pr_content": [],
      "comments": [],
      "btype": "DIALOGUE",
      "balloon_img": null
    }
  ],
  "images": null
}
```
