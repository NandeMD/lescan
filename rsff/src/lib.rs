//! # rsff
//!
//! `rsff` (scanlation file format) is the core library of an application designed to
//! facilitate the work of teams translating content such as manga, manhwa, manhua, webtoons, etc.

pub use balloon::Balloon;
pub use consts::{OUT, TYPES};

use std::ffi::OsStr;
use std::fs::File;
use std::io::{Cursor, Read, Write};
use std::path::Path;

use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;

use serde::{Deserialize, Serialize};

pub mod balloon;
pub mod consts;
mod docx_handlers;
pub mod img_data;
mod serde_overwrites;

#[cfg(feature = "async-io")]
use tokio::{
    fs,
    io::{AsyncReadExt, AsyncWriteExt},
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// A document containing all of your translation data.
///
/// # Examples
///
/// ```
/// use rsff::Document;
/// use rsff::balloon::Balloon;
///
/// // Create a default document.
/// let mut d: Document = Document::default();
///
/// // Create a default balloon.
/// let mut b: Balloon = Balloon::default();
///
/// // Add content to the balloon.
/// b.tl_content.push("This is a translation line.".to_string());
///
/// // Add balloon to the document.
/// d.balloons.push(b);
/// ```
#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    /// sff (Scanlation File Format) version. No big changes expected.
    pub METADATA_SCRIPT_VERSION: String,
    /// If you use this library for an app, it may come in handy to indicate your app's version.
    pub METADATA_APP_VERSION: String,
    /// Some other info you want to give/specify.
    pub METADATA_INFO: String,
    /// There is your balloons m8.
    pub balloons: Vec<Balloon>,
    /// Optional image paths for the work
    pub images: img_data::DocumentImage,
}

impl Default for Document {
    /// ```notrust
    /// METADATA_SCRIPT_VERSION: String::from("Scanlation Script File v0.2.0"),
    /// METADATA_APP_VERSION: String::new(),
    /// METADATA_INFO: String::from("Num"),
    /// balloons: Vec::new()
    /// ```
    fn default() -> Self {
        Self {
            METADATA_SCRIPT_VERSION: String::from("Scanlation Script File v0.1.0"),
            METADATA_APP_VERSION: String::new(),
            METADATA_INFO: String::from("Num"),
            balloons: Vec::new(),
            images: None,
        }
    }
}

impl Document {
    /// Open a supported sffx, sffz or txt file and generate a document.
    ///
    /// `fp`: full path for the file.
    ///
    /// # Examples
    ///
    /// ```
    /// use rsff::Document;
    ///
    /// let mut d: Document = Document::open("test.sffx").unwrap();
    /// ```
    pub fn open<P: ?Sized + AsRef<Path>>(file_path: &P) -> std::io::Result<Document> {
        let p = file_path.as_ref();

        if !p.exists() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "File does not exists!",
            ));
        }

        match p.extension() {
            None => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "No file ext!",
            )),
            Some(e) => {
                if e == OsStr::new("txt") {
                    let text = Self::read_file_to_string(p)?;
                    Ok(Self::txt_to_doc(text).map_err(|e| {
                        std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string())
                    })?)
                } else if e == OsStr::new("sffx") {
                    let jsn = Self::read_file_to_string(p)?;
                    Ok(Self::json_to_doc(jsn).map_err(|e| {
                        std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string())
                    })?)
                } else if e == OsStr::new("sffz") {
                    let compressed = Self::read_file_to_vecu8(p)?;
                    let mut jsn = String::new();
                    let mut decoder = ZlibDecoder::new(&*compressed);
                    decoder.read_to_string(&mut jsn)?;
                    Ok(Self::json_to_doc(jsn).map_err(|e| {
                        std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string())
                    })?)
                } else if e == OsStr::new("docx") {
                    let f = File::open(p)?;
                    Self::docx_to_doc(f).map_err(|e| {
                        std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string())
                    })
                } else {
                    Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "Unsupported file type!",
                    ))
                }
            }
        }
    }

    /// Async version of `open` function.
    /// Open a supported sffx, sffz or txt file and generate a document.
    ///
    /// `fp`: full path for the file.
    ///
    /// # Examples
    ///
    /// ```
    /// use rsff::Document;
    ///
    /// let mut d: Document = Document::open("test.sffx").await.unwrap();
    /// ```
    #[cfg(feature = "async-io")]
    pub async fn open_async<P: ?Sized + AsRef<Path>>(file_path: &P) -> tokio::io::Result<Document> {
        let p = file_path.as_ref();

        if !p.exists() {
            return Err(tokio::io::Error::new(
                tokio::io::ErrorKind::NotFound,
                "File does not exists!",
            ));
        }

        match p.extension() {
            None => Err(tokio::io::Error::new(
                tokio::io::ErrorKind::InvalidInput,
                "No file ext!",
            )),
            Some(e) => {
                if e == OsStr::new("txt") {
                    let text = Self::async_read_file_to_string(p).await?;
                    Ok(Self::txt_to_doc(text).map_err(|e| {
                        tokio::io::Error::new(tokio::io::ErrorKind::InvalidData, e.to_string())
                    })?)
                } else if e == OsStr::new("sffx") {
                    let jsn = Self::async_read_file_to_string(p).await?;
                    Ok(Self::json_to_doc(jsn).map_err(|e| {
                        tokio::io::Error::new(tokio::io::ErrorKind::InvalidData, e.to_string())
                    })?)
                } else if e == OsStr::new("sffz") {
                    let compressed = Self::async_read_file_to_vecu8(p).await?;
                    let mut jsn = String::new();
                    let mut decoder = ZlibDecoder::new(&*compressed);
                    decoder.read_to_string(&mut jsn)?;
                    Ok(Self::json_to_doc(jsn).map_err(|e| {
                        tokio::io::Error::new(tokio::io::ErrorKind::InvalidData, e.to_string())
                    })?)
                } else if e == OsStr::new("docx") {
                    let mut f = fs::File::open(p).await?;
                    let mut uwu = Vec::new();
                    f.read_to_end(&mut uwu).await?;
                    Self::docx_to_doc(Cursor::new(uwu)).map_err(|e| {
                        tokio::io::Error::new(tokio::io::ErrorKind::InvalidData, e.to_string())
                    })
                } else {
                    Err(tokio::io::Error::new(
                        tokio::io::ErrorKind::InvalidInput,
                        "Unsupported file type!",
                    ))
                }
            }
        }
    }

    fn docx_to_doc<R: Read + std::io::Seek>(r: R) -> Result<Document> {
        let docx_str = docx_handlers::parse_docx_to_string(r)?;

        Self::txt_to_doc(docx_str)
    }

    // Generate a document from lossy text.
    // Why did i write this?
    // This is probably most unnecessary code ib this crate.
    fn txt_to_doc(txt: String) -> Result<Document> {
        let mut d = Document::default();
        let mut texts: Vec<String> = Vec::with_capacity(200);

        let splitted = txt
            .split("\n")
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>();
        let mut is_previous_double_slash: bool = false;

        for i in 0..splitted.len() {
            if splitted[i].contains("//") {
                continue;
            }

            let current = splitted[i];

            let mut b = Balloon {
                btype: Self::decide_b_type_from_txt_line_headers(current),
                ..Default::default()
            };

            let next = splitted.get(i + 1).unwrap_or(&"");

            if !next.contains("//") {
                if is_previous_double_slash {
                    texts.push(current[4..current.len()].trim().to_string());
                    b.tl_content = texts.clone();
                    d.balloons.push(b);
                    is_previous_double_slash = false;
                    continue;
                } else {
                    b.tl_content
                        .push(current[4..current.len()].trim().to_string());
                    d.balloons.push(b);
                    is_previous_double_slash = false;
                    continue;
                }
            } else {
                texts.push(current[4..current.len()].trim().to_string());
                is_previous_double_slash = true;
            }
        }

        Ok(d)
    }

    // Generate a document from JSON string.
    pub fn json_to_doc(json: String) -> Result<Document> {
        let d = serde_json::from_str::<Document>(&json)?;

        Ok(d)
    }

    fn decide_b_type_from_txt_line_headers(ln: &str) -> TYPES {
        let s = &ln[0..2];

        match s {
            "()" => TYPES::DIALOGUE,
            "OT" => TYPES::OT,
            "[]" => TYPES::SQUARE,
            "ST" => TYPES::ST,
            "{}" => TYPES::THINKING,
            _ => TYPES::DIALOGUE,
        }
    }

    // Generate text of the whole document.
    fn read_file_to_string(p: &Path) -> std::io::Result<String> {
        let mut s = String::new();
        let mut f = File::open(p)?;
        f.read_to_string(&mut s)?;

        Ok(s)
    }

    // Async Generate text of the whole document.
    #[cfg(feature = "async-io")]
    async fn async_read_file_to_string(p: &Path) -> tokio::io::Result<String> {
        let mut s = String::new();
        let mut f = fs::File::open(p).await?;
        f.read_to_string(&mut s).await?;

        Ok(s)
    }

    // Open a file and return it's byte content.
    fn read_file_to_vecu8(p: &Path) -> std::io::Result<Vec<u8>> {
        let mut buff: Vec<u8> = Vec::new();
        let mut f = File::open(p)?;
        f.read_to_end(&mut buff)?;

        Ok(buff)
    }

    // Open a file and return it's byte content.
    #[cfg(feature = "async-io")]
    async fn async_read_file_to_vecu8(p: &Path) -> tokio::io::Result<Vec<u8>> {
        let mut buff: Vec<u8> = Vec::new();
        let mut f = fs::File::open(p).await?;
        f.read_to_end(&mut buff).await?;

        Ok(buff)
    }

    /// Total character count of all translation content.
    /// *(Spaces included.)*
    pub fn tl_chars(&self) -> usize {
        self.balloons.iter().map(|b| b.tl_chars()).sum()
    }

    /// Total character count of all proofread content.
    /// *(Spaces included.)*
    pub fn pr_chars(&self) -> usize {
        self.balloons.iter().map(|b| b.pr_chars()).sum()
    }

    /// Total character count of all comments.
    /// *(Spaces included.)*
    pub fn comment_chars(&self) -> usize {
        self.balloons.iter().map(|b| b.comments_chars()).sum()
    }

    /// Total line count of the whole document.
    /// Counts pr content lines if balloon has pr content, otherwise counts tl content lines.
    pub fn line_count(&self) -> usize {
        self.balloons.iter().map(|b| b.line_count()).sum()
    }

    /// Total balloon count.
    pub fn len(&self) -> usize {
        self.balloons.len()
    }

    pub fn is_empty(&self) -> bool {
        self.balloons.is_empty()
    }

    /// Add a balloon to the document.
    pub fn add_balloon(&mut self, b: Balloon) {
        self.balloons.push(b);
    }

    /// Add an empty balloon to the document.
    pub fn add_balloon_empty(&mut self) {
        self.balloons.push(Balloon::default());
    }

    /// Add balloon with a single translation line
    pub fn add_balloon_with_tl(&mut self, tl: impl Into<String>) {
        let mut b = Balloon::default();
        b.add_tl(tl);
        self.balloons.push(b);
    }

    /// Add balloon with a single proofread line
    pub fn add_balloon_with_pr(&mut self, pr: impl Into<String>) {
        let mut b = Balloon::default();
        b.add_pr(pr);
        self.balloons.push(b);
    }

    /// Add balloon with a single comment
    pub fn add_balloon_with_comment(&mut self, c: impl Into<String>) {
        let mut b = Balloon::default();
        b.add_comment(c);
        self.balloons.push(b);
    }

    /// Generates an JSON string of the balloon. No data loss so you can use this whenever you want.
    ///
    /// **Note:** Raw image data will be converted to a b64 encoded string.
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    // Save as a raw JSON file.
    fn save_raw(&self, fp: impl Into<std::path::PathBuf>) -> std::io::Result<()> {
        let mut file = File::create(fp.into())?;
        file.write_all(self.to_json().as_bytes())?;
        Ok(())
    }

    // Save as a compressed JSON file.
    fn save_zlib(&self, fp: impl Into<std::path::PathBuf>) -> std::io::Result<()> {
        let mut f = File::create(fp.into())?;
        let mut enc = ZlibEncoder::new(Vec::new(), Compression::best());
        enc.write_all(self.to_json().as_bytes())?;
        let encoded = enc.finish()?;
        f.write_all(&encoded)?;
        Ok(())
    }

    // Save as a .docx file
    fn save_docx(&self, fp: impl Into<std::path::PathBuf>) -> std::io::Result<()> {
        let f = File::create(fp.into())?;
        docx_handlers::string_to_docx(&self.to_string())
            .build()
            .pack(f)?;
        Ok(())
    }

    // Async save as a raw JSON file
    #[cfg(feature = "async-io")]
    async fn async_save_raw(&self, fp: impl Into<std::path::PathBuf>) -> std::io::Result<()> {
        let mut f = fs::File::create(fp.into()).await?;
        f.write_all(self.to_json().as_bytes()).await?;
        Ok(())
    }

    // Async save as a compressed JSON file
    #[cfg(feature = "async-io")]
    async fn async_save_zlib(&self, fp: impl Into<std::path::PathBuf>) -> std::io::Result<()> {
        let mut f = fs::File::create(fp.into()).await?;
        let mut enc = ZlibEncoder::new(Vec::new(), Compression::best());
        enc.write_all(self.to_json().as_bytes())?;
        let encoded = enc.finish()?;
        f.write_all(&encoded).await?;
        Ok(())
    }

    // Async save as a .docx file
    #[cfg(feature = "async-io")]
    async fn async_save_docx(&self, fp: impl Into<std::path::PathBuf>) -> std::io::Result<()> {
        let mut f = fs::File::create(fp.into()).await?;
        let mut v: Cursor<Vec<u8>> = Cursor::new(Vec::new());
        docx_handlers::string_to_docx(&self.to_string())
            .build()
            .pack(&mut v)?;
        f.write_all(&v.into_inner()).await?;
        Ok(())
    }

    /// Save your document as raw JSON, compressed JSON or .txt file.
    ///
    /// # Examples
    ///
    /// ```
    /// use rsff::Document;
    /// use rsff::consts::OUT;
    ///
    /// let d = Document::default();
    ///
    /// // Save as raw JSON:
    /// d.save(OUT::RAW, "raw_JSON");
    ///
    /// // Save as ZLIB compressed JSON:
    /// d.save(OUT::ZLIB, "compressed_JSON");
    ///
    /// // Save as raw text:
    /// d.save(OUT::TXT, "raw_text");
    /// ```
    pub fn save(&self, fp: impl Into<std::path::PathBuf>) -> std::io::Result<String> {
        let mut pb: std::path::PathBuf = fp.into();
        let out_type = {
            if let Some(ext) = pb.extension() {
                match ext.to_str().unwrap() {
                    "txt" => OUT::TXT,
                    "docx" => OUT::DOCX,
                    "sffx" => OUT::RAW,
                    "sffz" => OUT::ZLIB,
                    _ => return Err(std::io::Error::other("Unsupported Extension!")),
                }
            } else {
                pb.set_extension("sffz");
                OUT::ZLIB
            }
        };

        match out_type {
            OUT::RAW => self.save_raw(pb.clone())?,
            OUT::TXT => {
                let mut f = File::create(pb.clone())?;
                f.write_all(self.to_string().as_bytes())?;
            }
            OUT::ZLIB => self.save_zlib(pb.clone())?,
            OUT::DOCX => self.save_docx(pb.clone())?,
        }
        Ok(pb.display().to_string())
    }

    /// Async implementation of `save` function.
    /// Save your document as raw JSON, compressed JSON or .txt file.
    ///
    /// # Examples
    ///
    /// ```
    /// use rsff::Document;
    /// use rsff::consts::OUT;
    ///
    /// let d = Document::default();
    ///
    /// // Save as raw JSON:
    /// d.save(OUT::RAW, "raw_JSON").await;
    ///
    /// // Save as ZLIB compressed JSON:
    /// d.save(OUT::ZLIB, "compressed_JSON").await;
    ///
    /// // Save as raw text:
    /// d.save(OUT::TXT, "raw_text").await;
    /// ```
    #[cfg(feature = "async-io")]
    pub async fn save_async(&self, fp: impl Into<std::path::PathBuf>) -> std::io::Result<String> {
        let mut pb: std::path::PathBuf = fp.into();
        let out_type = {
            if let Some(ext) = pb.extension() {
                match ext.to_str().unwrap() {
                    "txt" => OUT::TXT,
                    "docx" => OUT::DOCX,
                    "sffx" => OUT::RAW,
                    "sffz" => OUT::ZLIB,
                    _ => return Err(std::io::Error::other("Unsupported Extension!")),
                }
            } else {
                pb.set_extension("sffz");
                OUT::ZLIB
            }
        };

        match out_type {
            OUT::RAW => self.async_save_raw(pb.clone()).await?,
            OUT::TXT => {
                let mut f = File::create(pb.clone())?;
                f.write_all(self.to_string().as_bytes())?;
            }
            OUT::ZLIB => self.async_save_zlib(pb.clone()).await?,
            OUT::DOCX => self.async_save_docx(pb.clone()).await?,
        }
        Ok(pb.display().to_string())
    }
}

/// Generates stringified version of the document.
/// Use this with caution because of data loss.
///
/// **IMPORTANT NOTE:** ***Metadata and balloon_img are lost during the creation of the text!!!***
impl std::fmt::Display for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            Vec::from_iter(self.balloons.iter().map(|b| b.to_string())).join("\n\n")
        )
    }
}

#[cfg(test)]
mod document_related {
    use crate::balloon::Balloon;
    use crate::consts::TYPES;
    use crate::Document;

    #[test]
    fn document_tl_chars() {
        let mut d = Document::default();
        let mut b1 = Balloon::default();
        let mut b2 = Balloon::default();

        b1.tl_content.push(String::from("num"));
        b2.tl_content.push(String::from("num"));
        b2.tl_content.push(String::from("namnam"));

        d.balloons.push(b1);
        d.balloons.push(b2);

        assert_eq!(d.tl_chars(), 12)
    }

    #[test]
    fn document_pr_chars() {
        let mut d = Document::default();
        let mut b1 = Balloon::default();
        let mut b2 = Balloon::default();

        b1.pr_content.push(String::from("num"));
        b2.pr_content.push(String::from("num"));
        b2.pr_content.push(String::from("namnam"));

        d.balloons.push(b1);
        d.balloons.push(b2);

        assert_eq!(d.pr_chars(), 12)
    }

    #[test]
    fn document_comment_chars() {
        let mut d = Document::default();
        let mut b1 = Balloon::default();
        let mut b2 = Balloon::default();

        b1.comments.push(String::from("num"));
        b2.comments.push(String::from("num"));
        b2.comments.push(String::from("namnam"));

        d.balloons.push(b1);
        d.balloons.push(b2);

        assert_eq!(d.comment_chars(), 12)
    }

    #[test]
    fn document_line_count() {
        let mut d = Document::default();
        let mut b1 = Balloon::default();
        let mut b2 = Balloon::default();

        b1.tl_content.push(String::from("num"));
        b2.tl_content.push(String::from("num"));
        b2.pr_content.push(String::from("namnam"));

        d.balloons.push(b1);
        d.balloons.push(b2);

        assert_eq!(d.line_count(), 2)
    }

    #[test]
    fn document_length() {
        let mut d = Document::default();
        let b1 = Balloon::default();
        let b2 = Balloon::default();

        d.balloons.push(b1);
        d.balloons.push(b2);

        assert_eq!(d.len(), 2)
    }

    #[test]
    fn document_to_string() {
        let mut d = Document::default();
        let mut b1 = Balloon::default();
        let mut b2 = Balloon::default();

        b1.tl_content.push(String::from("num"));
        b1.tl_content.push(String::from("nam"));
        b1.pr_content.push(String::from("numnam"));
        b1.btype = TYPES::OT;

        b2.tl_content.push(String::from("num"));

        d.balloons.push(b1);
        d.balloons.push(b2);

        assert_eq!(d.to_string(), String::from("OT: numnam\n\n(): num"))
    }

    #[test]
    fn document_open_txt() {
        let d = Document::open("test.txt").unwrap();

        assert_eq!(d.line_count(), 2);
        assert_eq!(d.balloons.len(), 2);
        assert_eq!(d.balloons[0].btype, TYPES::OT);
        assert_eq!(d.balloons[0].tl_content[0], "numnam");
        assert_eq!(d.balloons[1].btype, TYPES::DIALOGUE);
        assert_eq!(d.balloons[1].tl_content[0], "num");
    }

    #[test]
    fn document_unsupported_file_ext() {
        let r = Document::open("test.test");
        assert!(r.is_err())
    }
}
