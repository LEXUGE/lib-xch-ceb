// Copyright 2017-2019 LEXUGE
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

//! Public structs

// This struct is required to formating the equation
/// Description of the chemical equation
#[derive(Clone)]
pub struct ChemicalEquation {
    /// The number of the chemical formulas of the left side.
    pub left: usize,
    /// The number of the chemical formulas of the right side.
    pub right: usize,
    /// The sum of all the chemical formulas in the equation.
    pub sum: usize,
}

impl ChemicalEquation {
    pub(crate) fn new() -> Self {
        Self {
            left: 0,
            right: 0,
            sum: 0,
        }
    }
}
