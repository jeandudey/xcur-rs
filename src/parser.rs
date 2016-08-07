// xcur-rs - Parser for XCursor files in Rust
// Copyright (C) 2016  Jean Pierre Dudey
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.


//! # Basic usage
//! To parse a `XCursor` file you will need to use the `File::parse` function,
//! you need to pass to it an `u8` inmutable slice and voila! that's all.

use super::nom::IResult;

/// 32-bit unsigned integer
pub type CARD32 = u32;

/// 8-bit unsigned integer
pub type CARD8 = u8;

#[cfg(target_endian="big")]
fn card32(i: &[u8]) -> IResult<&[u8], u32> {
    use nom::be_u32;
    be_u32(i)
}

#[cfg(target_endian="little")]
fn card32(i: &[u8]) -> IResult<&[u8], u32> {
    use nom::le_u32;
    le_u32(i)
}

const HEADER_SIZE: usize = 12;

/// Represents the `XCur` file `Header`
#[derive(Debug, Clone, Copy)]
pub struct Header {
    /// Magic bytes
    pub magic: CARD32,

    /// Bytes in header
    pub header: CARD32,

    /// File version
    pub version: CARD32,

    /// Number of TOCs
    pub ntoc: CARD32,
}

impl Header {
    named!(parse(&[u8]) -> Header,
        chain!(
            magic: card32 ~
            header: card32  ~
            version: card32  ~
            ntoc: card32    ,
            || {
                Header {
                    magic: magic,
                    header: header,
                    version: version,
                    ntoc: ntoc,
                }
            }
        )
    );

    #[cfg(target_endian="big")]
    fn validate(&self) -> Result<(), &'static str> {
        if self.magic != 0x58637572 {
            Err("Invalid magic bytes")
        } else {
            Ok(())
        }
    }

    #[cfg(target_endian="little")]
    fn validate(&self) -> Result<(), &'static str> {
        if self.magic != 0x72756358 {
            Err("Invalid magic bytes")
        } else {
            Ok(())
        }
    }
}

const TABLE_OF_CONTENTS_SIZE: usize = 12;

/// Information of chunks
#[derive(Debug, Clone, Copy)]
pub struct TableOfContents {
    /// Entry type.
    pub type_: CARD32,

    /// Type-specific label, size for images.
    pub subtype: CARD32,

    /// Absolute position of table in file
    pub position: CARD32,
}

impl TableOfContents {
    named!(parse(&[u8]) -> TableOfContents,
        chain!(
            type_: card32 ~
            subtype: card32 ~
            position: card32,
            || {
                TableOfContents {
                    type_: type_,
                    subtype: subtype,
                    position: position,
                }
            }
        )
    );
}

/// The base header for all chunks
#[derive(Debug, Clone, Copy)]
pub struct ChunkHeader {
    /// Bytes used in the chunk
    pub header: CARD32,

    /// The chunk type
    pub type_: CARD32,

    /// The subtype
    pub subtype: CARD32,

    /// The chunk version
    pub version: CARD32,
}

impl ChunkHeader {
    named!(parse(&[u8]) -> ChunkHeader,
        chain!(
            header: card32~
            type_: card32~
            subtype: card32~
            version: card32,
            || {
                ChunkHeader {
                    header: header,
                    type_: type_,
                    subtype: subtype,
                    version: version,
                }
            }
        )
    );
}

const COMMENT_TYPE: u32 = 0xFFFE0001;
const COMMENT_VERSION: u32 = 1;

/// Comment subtype
pub const COMMENT_COPYRIGHT: u32 = 1;

/// Comment subtype
pub const COMMENT_LICENSE: u32 = 2;

/// Comment subtype
pub const COMMENT_OTHER: u32 = 3;

/// Represents a comment in `XCursor` file
#[derive(Debug, Clone)]
pub struct Comment {
    /// The chunk header
    pub chunkheader: ChunkHeader,

    /// The comment string length
    pub length: CARD32,

    /// The comment string
    pub string: String,
}

impl Comment {
    fn parse(i: &[u8]) -> IResult<&[u8], Comment> {
        let (i1, chunkheader) = try_parse!(i, ChunkHeader::parse);
        let (i2, length) = try_parse!(i1, card32);
        let (i3, string) = try_parse!(i2, take!(length));

        IResult::Done(i3,
                      Comment {
                          chunkheader: chunkheader,
                          length: length,
                          string: String::from_utf8(string.to_vec())
                              .unwrap_or_else(|_| String::new()),
                      })
    }
}

/// An image
#[derive(Debug, Clone)]
pub struct Image {
    /// The chunk header
    pub chunkheader: ChunkHeader,

    /// Image width
    pub width: CARD32,

    /// Image height
    pub height: CARD32,

    /// X Hot
    pub xhot: CARD32,

    /// Y Hot
    pub yhot: CARD32,

    /// Delay in ms
    pub delay: CARD32,

    /// The actual image data
    pub pixels: Vec<CARD32>,
}

const IMAGE_TYPE: u32 = 0xFFFD0002;
const IMAGE_VERSION: u32 = 1;
const IMAGE_MAX_SIZE: u32 = 0x7FFF;

impl Image {
    fn parse(i: &[u8]) -> IResult<&[u8], Image> {
        let (i1, chunkheader) = try_parse!(i, ChunkHeader::parse);
        let (i2, width) = try_parse!(i1, card32);
        let (i3, height) = try_parse!(i2, card32);
        let (i4, xhot) = try_parse!(i3, card32);
        let (i5, yhot) = try_parse!(i4, card32);
        let (_, delay) = try_parse!(i5, card32);

        // TODO: Optimize this shitty and slow way of getting pixels
        let mut pixels: Vec<CARD32> = Vec::with_capacity((width * height) as usize);
        let mut count: usize = 24;
        for _ in 0..width * height {
            let (_, pixel) = try_parse!(&i[count..], card32);
            pixels.push(pixel);

            count += ::std::mem::size_of::<CARD32>();
        }

        IResult::Done(i3,
                      Image {
                          chunkheader: chunkheader,
                          width: width,
                          height: height,
                          xhot: xhot,
                          yhot: yhot,
                          delay: delay,
                          pixels: pixels,
                      })
    }
}

/// The `XCursor` file
#[derive(Debug, Clone)]
pub struct File {
    /// The comments in file
    pub comments: Vec<Comment>,

    /// The images in file
    pub images: Vec<Image>,
}

impl File {
    /// Parses an XCursor file
    pub fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        let (_, header) = try_parse!(i, Header::parse);
        header.validate().unwrap();

        let mut tocs: Vec<TableOfContents> = Vec::with_capacity(header.ntoc as usize);
        let mut toc_count: usize = HEADER_SIZE + 4;
        for _ in 0..header.ntoc {
            let (_, toc) = try_parse!(&i[toc_count..], TableOfContents::parse);
            tocs.push(toc);
            toc_count += TABLE_OF_CONTENTS_SIZE;
        }

        let mut comments: Vec<Comment> = Vec::new();
        let mut images: Vec<Image> = Vec::new();

        for toc in tocs {
            match toc.type_ {
                COMMENT_TYPE => {
                    let (_, comment) = try_parse!(&i[(toc.position as usize)..], Comment::parse);
                    assert_eq!(comment.chunkheader.version, COMMENT_VERSION);
                    comments.push(comment);
                }
                IMAGE_TYPE => {
                    let (_, image) = try_parse!(&i[(toc.position as usize)..], Image::parse);
                    assert_eq!(image.chunkheader.version, IMAGE_VERSION);
                    images.push(image);
                }
                _ => unreachable!(),
            }
        }

        IResult::Done(i,
                      File {
                          comments: comments,
                          images: images,
                      })
    }
}
