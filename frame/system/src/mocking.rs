// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Provide types to help defining a mock environment when testing pallets.

use crate::BlockNumberOf;
use sp_runtime::generic;

/// An unchecked extrinsic type to be used in tests.
pub type MockUncheckedExtrinsic<T, Signature = (), Extra = ()> = generic::UncheckedExtrinsic<
	<T as crate::Config>::AccountId,
	<T as crate::Config>::RuntimeCall,
	Signature,
	Extra,
>;

/// An implementation of `sp_runtime::traits::Block` to be used in tests.
pub type MockBlock<T> = generic::Block<
	generic::Header<BlockNumberOf<T>, sp_runtime::traits::BlakeTwo256>,
	MockUncheckedExtrinsic<T>,
>;
