// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

#![forbid(unsafe_code)]
#![allow(clippy::too_many_arguments)]

pub use snarkvm_console_types_address as address;
pub use snarkvm_console_types_address::Address;

pub use snarkvm_console_types_boolean as boolean;
pub use snarkvm_console_types_boolean::Boolean;

pub use snarkvm_console_types_field as field;
pub use snarkvm_console_types_field::Field;

pub use snarkvm_console_types_group as group;
pub use snarkvm_console_types_group::Group;

pub use snarkvm_console_types_integers as integers;
pub use snarkvm_console_types_integers::{U8, U16, U32, U64, U128, I8, I16, I32, I64, I128};

pub use snarkvm_console_types_scalar as scalar;
pub use snarkvm_console_types_scalar::Scalar;

pub use snarkvm_console_types_string as string;
pub use snarkvm_console_types_string::StringType;
