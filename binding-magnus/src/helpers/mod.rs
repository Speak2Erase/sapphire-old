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

use crate::window::CursorRectProvider;
mod macros;
pub(crate) use macros::{provider_enum, provider_enum_impl};

pub trait Provider<T> {
    fn provide<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&T) -> R;

    fn provide_mut<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut T) -> R;

    fn provide_copy(&self) -> T
    where
        T: Copy,
    {
        self.provide(|v| *v)
    }
}

#[derive(Clone, Copy)]
pub enum ProviderVal<T, P>
where
    P: Provider<T>,
{
    Value(T),
    Provider(P),
}

impl<T, P> Default for ProviderVal<T, P>
where
    P: Provider<T>,
    T: Default,
{
    fn default() -> Self {
        Self::Value(T::default())
    }
}

impl<T, P> ProviderVal<T, P>
where
    P: Provider<T>,
{
    pub fn val(v: impl Into<T>) -> Self {
        Self::Value(v.into())
    }

    pub fn provider(p: impl Into<P>) -> Self {
        Self::Provider(p.into())
    }
}

impl<T, P> Provider<T> for ProviderVal<T, P>
where
    P: Provider<T>,
{
    fn provide<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&T) -> R,
    {
        match self {
            Self::Value(v) => f(v),
            Self::Provider(p) => p.provide(f),
        }
    }

    fn provide_mut<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut T) -> R,
    {
        match self {
            Self::Value(v) => f(v),
            Self::Provider(p) => p.provide_mut(f),
        }
    }
}

provider_enum! {
  #[derive(Clone, Copy)]
  pub enum RectProvider: librgss::Rect {
    CursorRect(CursorRectProvider)
  }
}
