// Copyright (C) 2024 Lily Lyons
//
// This file is part of sapphire.
//
// sapphire is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// sapphire is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with sapphire.  If not, see <http://www.gnu.org/licenses/>.

pub struct Script {
    pub name: String,
    pub script_text: String,
}

impl<'de> serde::Deserialize<'de> for Script {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Script;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("an array")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                use serde::de::Error;
                use std::io::Read;

                let Some(_) = seq.next_element::<serde::de::IgnoredAny>()? else {
                    return Err(A::Error::missing_field("id"));
                };

                let Some(name) = seq.next_element()? else {
                    return Err(A::Error::missing_field("name"));
                };

                let Some(data) = seq.next_element::<alox_48::RbString>()? else {
                    return Err(A::Error::missing_field("data"));
                };

                let mut decoder = flate2::bufread::ZlibDecoder::new(data.data.as_slice());
                let mut script = String::new();
                decoder
                    .read_to_string(&mut script)
                    .map_err(A::Error::custom)?;

                Ok(Script {
                    name,
                    script_text: script,
                })
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}
