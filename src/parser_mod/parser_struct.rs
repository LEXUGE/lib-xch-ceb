// Copyright 2017-2018 LEXUGE
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

// Overall: This is the source code of the Delta-3 Parser.

use std::collections::HashMap;
// inside uses
use super::get_token;
use api::handler::ErrorCases;
use api::handler::ErrorCases::NotFound;
use api::traits::CheckedType;
use public::{safe_calc, Operator};

pub struct FormulaDesc<T: CheckedType> {
    pub formula_self: String,
    pub times: T,
    pub all: String,
}

pub struct TokenDesc<T: CheckedType> {
    pub token_name: String,
    pub times: T,
}

// This is the data structure of describing the result of Delta-3 Parser.
// This is the form of the `list`:
// |     \     | formula_1 | formula_2 | ...       | formula_n |
// | element_1 | ...       | ...       | ...       | ...       |
// | element_2 | ...       | ...       | ...       | ...       |
// | ...       | ...       | ...       | ...       | ...       |
// | element_n | ...       | ...       | ...       | ...       |
pub struct TableDesc<T: CheckedType> {
    elements_table: HashMap<String, usize>, // store the index of elements
    list: Vec<Vec<T>>,
    formula_sum: usize,
}

impl<T: CheckedType> TableDesc<T> {
    pub fn store_in_table(
        &mut self,
        formula: &str,
        location: usize,
        neg: bool,
    ) -> Result<bool, ErrorCases> {
        for mut t in get_token(formula)? {
            if !self.elements_table.contains_key(&t.token_name) {
                let len = self.elements_table.len();
                self.elements_table.insert(t.token_name.clone(), len);
                self.update_list_vec();
            }

            {
                // store data in table
                let tmp = match self.elements_table.get(&t.token_name) {
                    Some(s) => *s,
                    None => return Err(NotFound),
                }; // It have been checked.
                if neg {
                    t.times = safe_calc(&t.times, &T::zero(), &Operator::Neg)?
                }
                self.list[tmp][location] =
                    safe_calc(&self.list[tmp][location], &t.times, &Operator::Add)?;
            }
        }
        Ok(true)
    }

    pub fn get_list(&self) -> Vec<Vec<T>> {
        (self.list).to_vec()
    }

    pub fn new(sum: usize) -> Self {
        // PLEASE call update_list_vec after new!
        Self {
            elements_table: HashMap::new(),
            list: Vec::new(),
            formula_sum: sum,
        }
    }

    pub fn update_list_vec(&mut self) {
        let v = self.generate_vec();
        self.list.push(v);
    }

    fn generate_vec(&self) -> Vec<T> {
        vec![T::zero(); self.formula_sum]
    }
}
