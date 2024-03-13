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

macro_rules! provider_enum {
  (
    $(#[$enum_item:meta])*
    $enum_vis:vis enum $enum_name:ident: $provider_type:ty
    {
      $($variant_name:ident($variant_type:ty)),+ $(,)?
    }
  ) => {
    $(#[$enum_item])*
    $enum_vis enum $enum_name {
      $($variant_name($variant_type),)+
    }

    $crate::helpers::provider_enum_impl!($enum_name: $provider_type { $($variant_name,)+ });

    $(
      impl From<$variant_type> for $enum_name {
        fn from(value: $variant_type) -> Self {
          Self::$variant_name(value)
        }
      }
    )+
  };
}
pub(crate) use provider_enum;

macro_rules! provider_enum_impl {
  ($enum_name:ident: $provider_type:ty { $($variant_name:ident),+ $(,)? }) => {
    impl Provider<$provider_type> for $enum_name {
      fn provide<F, R>(&self, f: F) -> R
      where
      F: FnOnce(&$provider_type) -> R,
      {
        match self {
          $(Self::$variant_name(v) => v.provide(f),)+
        }
      }

      fn provide_mut<F, R>(&mut self, f: F) -> R
      where
      F: FnOnce(&mut $provider_type) -> R,
      {
        match self {
          $(Self::$variant_name(v) => v.provide_mut(f),)+
        }
      }
    }
  };
}
pub(crate) use provider_enum_impl;
