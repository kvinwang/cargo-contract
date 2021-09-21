// Copyright 2018-2021 Parity Technologies (UK) Ltd.
// This file is part of cargo-contract.
//
// cargo-contract is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// cargo-contract is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with cargo-contract.  If not, see <http://www.gnu.org/licenses/>.

pub mod build;
pub mod metadata;
pub mod new;
pub mod test;

pub(crate) use self::{
    build::{BuildCommand, CheckCommand},
    test::TestCommand,
};
mod extrinsics;

pub(crate) use self::extrinsics::{
    call::CallCommand, instantiate::InstantiateCommand, instantiate_with_code::InstantiateWithCode,
};
