use crate::consts::TYPES;
use crate::img_data::BalloonImage;
use serde::{Deserialize, Serialize};

/// A struct represents a balloon.
///
/// Contains translation and proofred contents, comments, balloon image (if has any). Must have a distinct type.
/// # Examples
///
/// ```
/// use rsff::balloon::Balloon;
///
/// let mut b: Balloon = Balloon::default();
/// b.tl_content.push("This is a tl line.".to_string());
/// ```
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Balloon {
    pub tl_content: Vec<String>,
    pub pr_content: Vec<String>,
    pub comments: Vec<String>,
    pub btype: TYPES,
    pub balloon_img: Option<BalloonImage>,
}

impl Balloon {
    /// Add image to balloon. Creates a `BalloonImage` struct and adds to the balloon.
    /// `img_type` is a string defines image's extention. '.jpg' etc.
    /// `img_data` is raw image as bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use rsff::balloon::Balloon;
    /// use image;
    ///
    /// let mut b = Balloon::default();
    /// let test_img = image::open("testimg.jpg").unwrap();
    /// b.add_image(
    ///     "jpg".to_string(),
    ///     test_img.into_bytes()
    /// );
    /// ```
    pub fn add_image(&mut self, img_type: String, img_data: Vec<u8>) {
        self.balloon_img = Some(BalloonImage { img_type, img_data });
    }

    /// Removes the image from balloon.
    pub fn remove_img(&mut self) {
        self.balloon_img = None;
    }

    /// Total character count of all translation content.
    /// *(Spaces included.)*
    pub fn tl_chars(&self) -> usize {
        self.tl_content.iter().map(|text| text.len()).sum()
    }

    /// Total character count of all proofread content.
    /// *(Spaces included.)*
    pub fn pr_chars(&self) -> usize {
        self.pr_content.iter().map(|text| text.len()).sum()
    }

    /// Total character count of all comments.
    /// *(Spaces included.)*
    pub fn comments_chars(&self) -> usize {
        self.comments.iter().map(|text| text.len()).sum()
    }

    /// Total line count of the balloon.
    /// Counts pr content lines if balloon has pr content, otherwise counts tl content lines.
    pub fn line_count(&self) -> usize {
        if !self.pr_content.is_empty() {
            self.pr_content.len()
        } else {
            self.tl_content.len()
        }
    }

    /// Generates an JSON string of the balloon. No data loss so you can use this whenever you want.
    ///
    /// **Note:** Raw image data will be converted to a b64 encoded string.
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    /// Adds a translation line to the balloon.
    pub fn add_tl(&mut self, tl: impl Into<String>) {
        self.tl_content.push(tl.into());
    }

    /// Adds a proofread line to the balloon.
    pub fn add_pr(&mut self, pr: impl Into<String>) {
        self.pr_content.push(pr.into());
    }

    /// Adds a comment line to the balloon.
    pub fn add_comment(&mut self, comment: impl Into<String>) {
        self.comments.push(comment.into());
    }
}

/// Generates stringified version of the balloon.
/// Use this with caution because of data loss.
///
/// **IMPORTANT NOTE:** ***Metadata and balloon_img are lost during the creation of the text!!!***
impl std::fmt::Display for Balloon {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let type_str = match self.btype {
            TYPES::DIALOGUE => "(): ",
            TYPES::OT => "OT: ",
            TYPES::SQUARE => "[]: ",
            TYPES::ST => "ST: ",
            TYPES::THINKING => "{}: ",
        };

        // If balloon has pr content, generate balloon text from pr content
        // else, generate balloon text from tl content
        if !self.pr_content.is_empty() {
            write!(
                f,
                "{}",
                self.pr_content
                    .iter()
                    .map(|pr| format!("{}{}", type_str, pr))
                    .collect::<Vec<String>>()
                    .join("\n//\n")
            )
        } else {
            write!(
                f,
                "{}",
                self.tl_content
                    .iter()
                    .map(|tl| format!("{}{}", type_str, tl))
                    .collect::<Vec<String>>()
                    .join("\n//\n")
            )
        }
    }
}

#[cfg(test)]
mod ballon_tests {
    use super::Balloon;
    use image;

    #[test]
    fn ballo0n_add_img() {
        let mut b = Balloon::default();
        let test_img = image::open("testimg.jpg").unwrap();
        b.add_image("jpg".to_string(), test_img.into_bytes());
    }

    #[test]
    fn balloon_remove_img() {
        let mut b = Balloon::default();
        let test_img = image::open("testimg.jpg").unwrap();
        b.add_image("jpg".to_string(), test_img.into_bytes());
        b.remove_img();
    }

    #[test]
    fn balloon_get_tl_chars() {
        let mut b = Balloon::default();

        b.tl_content.push("Text 1".to_string());
        b.tl_content.push("Text 2".to_string());

        assert_eq!(b.tl_chars(), 12);
    }

    #[test]
    fn balloon_get_tl_len() {
        let mut b = Balloon::default();

        b.tl_content.push("Text 1".to_string());
        b.tl_content.push("Text 2".to_string());

        assert_eq!(b.tl_content.len(), 2);
    }

    #[test]
    fn balloon_get_pr_chars() {
        let mut b = Balloon::default();

        b.pr_content.push("Text 1".to_string());
        b.pr_content.push("Text 2".to_string());

        assert_eq!(b.pr_chars(), 12);
    }

    #[test]
    fn balloon_get_pr_len() {
        let mut b = Balloon::default();

        b.pr_content.push("Text 1".to_string());
        b.pr_content.push("Text 2".to_string());

        assert_eq!(b.pr_content.len(), 2);
    }

    #[test]
    fn balloon_get_comment_chars() {
        let mut b = Balloon::default();

        b.comments.push("Text 1".to_string());
        b.comments.push("Text 2".to_string());

        assert_eq!(b.comments_chars(), 12);
    }

    #[test]
    fn balloon_get_comment_len() {
        let mut b = Balloon::default();

        b.comments.push("Text 1".to_string());
        b.comments.push("Text 2".to_string());

        assert_eq!(b.comments.len(), 2);
    }

    #[test]
    fn balloon_to_string() {
        let mut b = Balloon::default();

        b.tl_content.push("a".to_string());
        b.pr_content.push("a".to_string());
        b.comments.push("a".to_string());
        b.pr_content.push("ZZZZZ".to_string());

        let test_img = image::open("testimg.jpg").unwrap();
        b.add_image("jpg".to_string(), test_img.into_bytes());

        let str = b.to_string();

        let intended_result = String::from("(): a\n//\n(): ZZZZZ");
        assert_eq!(str, intended_result);
    }

    #[test]
    fn balloon_to_json() {
        let mut b = Balloon::default();

        b.tl_content.push("a".to_string());
        b.pr_content.push("a".to_string());
        b.comments.push("a".to_string());
        b.pr_content.push("ZZZZZ".to_string());

        let test_img = image::open("testimg.jpg").unwrap();
        b.add_image("jpg".to_string(), test_img.into_bytes());

        let str = b.to_json();

        let intended_json = String::from(
            r#"{"tl_content":["a"],"pr_content":["a","ZZZZZ"],"comments":["a"],"btype":"DIALOGUE","balloon_img":{"img_type":"jpg","img_data":"2be18zs71c_P0dPS1NTS0tPX09HS17-_81BR_6in0dLU709P4ZKV09TW1dPU2tnX2tzZ7u_x6srL_gwL7u7u7Kin8zs70dHP2dnZ5eXl5uTl09PT_v7-6Hh22dfa0cvN70dG5n-A09HU09XU09PV1cfH7Jua9EJC1tbW2NjY2ru5-CEf3pSV53Bs8zs5-hob8UlJ44WF5Hp65IB-7U5L_Rgd-hgZ52tr4qal-fTw3Nzc09PT-DAw8m5s_bOy7uDf91FT9oqK1NTS2tne3d3d19fV3t7e_v__9fXz19nY-tzc_0ZE47az1dPU1NTU1NTU1tbW3t7e2NjY2tra2tra4YuM9jU23d3d09PT1dXV29vb4-Pj3Nzc1tbW1tbW2dnZ_woJ2NTT29vb1tbW"}}"#,
        );
        assert_eq!(str, intended_json)
    }
}
