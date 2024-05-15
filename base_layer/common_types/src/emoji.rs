// Copyright 2020. The Tari Project
//
// Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
// following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following
// disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the
// following disclaimer in the documentation and/or other materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote
// products derived from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
// INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
// WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use std::{
    collections::HashMap,
    convert::TryFrom,
    fmt::{Display, Error, Formatter},
    str::FromStr,
};

use once_cell::sync::Lazy;
use thiserror::Error;

/// An emoji encoding of a byte vector.
/// This can be useful for representations of things like keys or addresses where visual differentiation is important.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct EmojiId {
    bytes: Vec<u8>,
}

const DICT_SIZE: usize = 256; // number of elements in the emojibet

/// The emojibet, mapping byte values to emoji characters
pub const BYTE_TO_EMOJI: [char; DICT_SIZE] = [
    '🦋', '📟', '🌈', '🌊', '🎯', '🐋', '🌙', '🤔', '🌕', '⭐', '🎋', '🌰', '🌴', '🌵', '🌲', '🌸', '🌹', '🌻', '🌽',
    '🍀', '🍁', '🍄', '🥑', '🍆', '🍇', '🍈', '🍉', '🍊', '🍋', '🍌', '🍍', '🍎', '🍐', '🍑', '🍒', '🍓', '🍔', '🍕',
    '🍗', '🍚', '🍞', '🍟', '🥝', '🍣', '🍦', '🍩', '🍪', '🍫', '🍬', '🍭', '🍯', '🥐', '🍳', '🥄', '🍵', '🍶', '🍷',
    '🍸', '🍾', '🍺', '🍼', '🎀', '🎁', '🎂', '🎃', '🤖', '🎈', '🎉', '🎒', '🎓', '🎠', '🎡', '🎢', '🎣', '🎤', '🎥',
    '🎧', '🎨', '🎩', '🎪', '🎬', '🎭', '🎮', '🎰', '🎱', '🎲', '🎳', '🎵', '🎷', '🎸', '🎹', '🎺', '🎻', '🎼', '🎽',
    '🎾', '🎿', '🏀', '🏁', '🏆', '🏈', '⚽', '🏠', '🏥', '🏦', '🏭', '🏰', '🐀', '🐉', '🐊', '🐌', '🐍', '🦁', '🐐',
    '🐑', '🐔', '🙈', '🐗', '🐘', '🐙', '🐚', '🐛', '🐜', '🐝', '🐞', '🐢', '🐣', '🐨', '🦀', '🐪', '🐬', '🐭', '🐮',
    '🐯', '🐰', '🦆', '🦂', '🐴', '🐵', '🐶', '🐷', '🐸', '🐺', '🐻', '🐼', '🐽', '🐾', '👀', '👅', '👑', '👒', '🧢',
    '💅', '👕', '👖', '👗', '👘', '👙', '💃', '👛', '👞', '👟', '👠', '🥊', '👢', '👣', '🤡', '👻', '👽', '👾', '🤠',
    '👃', '💄', '💈', '💉', '💊', '💋', '👂', '💍', '💎', '💐', '💔', '🔒', '🧩', '💡', '💣', '💤', '💦', '💨', '💩',
    '➕', '💯', '💰', '💳', '💵', '💺', '💻', '💼', '📈', '📜', '📌', '📎', '📖', '📿', '📡', '⏰', '📱', '📷', '🔋',
    '🔌', '🚰', '🔑', '🔔', '🔥', '🔦', '🔧', '🔨', '🔩', '🔪', '🔫', '🔬', '🔭', '🔮', '🔱', '🗽', '😂', '😇', '😈',
    '🤑', '😍', '😎', '😱', '😷', '🤢', '👍', '👶', '🚀', '🚁', '🚂', '🚚', '🚑', '🚒', '🚓', '🛵', '🚗', '🚜', '🚢',
    '🚦', '🚧', '🚨', '🚪', '🚫', '🚲', '🚽', '🚿', '🧲',
];

// The reverse emojibet, mapping emoji characters to byte values
pub static EMOJI_TO_BYTE: Lazy<HashMap<char, u8>> = Lazy::new(|| {
    let mut m = HashMap::with_capacity(DICT_SIZE);
    BYTE_TO_EMOJI.iter().enumerate().for_each(|(i, c)| {
        m.insert(*c, u8::try_from(i).expect("Invalid emoji"));
    });
    m
});

#[derive(Debug, Error, PartialEq)]
pub enum EmojiIdError {
    #[error("Invalid emoji character")]
    InvalidEmoji,
}

impl EmojiId {
    /// Get the bytes from an emoji ID
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl FromStr for EmojiId {
    type Err = EmojiIdError;

    /// Try to convert a string of emoji to an emoji ID
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Convert the emoji string to a byte vector
        let mut bytes = Vec::<u8>::with_capacity(s.chars().count());
        for c in s.chars() {
            if let Some(i) = EMOJI_TO_BYTE.get(&c) {
                bytes.push(*i);
            } else {
                return Err(EmojiIdError::InvalidEmoji);
            }
        }

        Ok(Self { bytes })
    }
}

impl From<&[u8]> for EmojiId {
    fn from(value: &[u8]) -> Self {
        Self::from(value.to_vec())
    }
}

impl From<Vec<u8>> for EmojiId {
    fn from(value: Vec<u8>) -> Self {
        Self { bytes: value }
    }
}

impl Display for EmojiId {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        // Convert the byte vector to an emoji string
        let emoji = self
            .bytes
            .iter()
            .map(|b| BYTE_TO_EMOJI[*b as usize])
            .collect::<String>();

        fmt.write_str(&emoji)
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::*;

    #[test]
    /// Check the the emoji table has no duplicates
    fn no_duplicate_emoji() {
        assert_eq!(
            BYTE_TO_EMOJI[..BYTE_TO_EMOJI.len()]
                .iter()
                .enumerate()
                .any(|(i, emoji)| BYTE_TO_EMOJI[i + 1..].contains(emoji)),
            false
        );
    }

    #[test]
    /// Test valid emoji ID from bytes
    fn valid_emoji_id_from_bytes() {
        // Generate an emoji ID from a complete ordered byte vector
        let bytes: Vec<u8> = (0..=u8::MAX).collect();
        let emoji_id = EmojiId::from(bytes.as_slice());

        // Ensure we get the corresponding emoji
        assert_eq!(emoji_id.to_string(), BYTE_TO_EMOJI.iter().collect::<String>());

        // Check that we get the original bytes back again
        assert_eq!(emoji_id.as_bytes(), &bytes);
    }

    #[test]
    /// Test valid emoji ID from string
    fn valid_emoji_id_from_string() {
        // Generate an emoji ID from a complete ordered emoji string
        let emoji: String = BYTE_TO_EMOJI.iter().collect();
        let emoji_id = EmojiId::from_str(&emoji).unwrap();

        // Ensure we get the corresponding bytes
        assert_eq!(emoji_id.as_bytes(), &(0..=u8::MAX).collect::<Vec<u8>>());

        // Check that we get the original emoji string back again
        assert_eq!(emoji_id.to_string(), emoji);
    }

    #[test]
    /// Test invalid emoji
    fn invalid_emoji() {
        // Choose an emoji not in the emojibet
        let invalid_emoji: char = '🎅';
        assert_eq!(EMOJI_TO_BYTE.contains_key(&invalid_emoji), false);

        // This emoji string contains the invalid emoji character
        let emoji_string = "🌴🦀🔌📌🚑🌰🎓🌴🐊🐌🔒💡🐜📜👛🍵👛🐽🎂🐻🦋🍓👶🐭🐼🏀🎪💔💵🥑🔋🎒🎅";
        assert!(emoji_string.contains(invalid_emoji));

        // We can't create an emoji ID from it
        assert_eq!(EmojiId::from_str(emoji_string), Err(EmojiIdError::InvalidEmoji));
    }
}
