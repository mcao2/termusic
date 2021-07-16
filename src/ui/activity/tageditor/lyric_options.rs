/**
 * MIT License
 *
 * termusic - Copyright (c) 2021 Larry Hao
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
use super::TagEditorActivity;

// use crate::song::Song;
// use std::fs::{self, File};
// use std::io::{BufRead, BufReader, Write};
// use std::path::{Path, PathBuf};
// use std::str::FromStr;
use crate::lyric::SongTag;
use crate::ui::components::scrolltable;
use tuirealm::PropsBuilder;

use tuirealm::props::{TableBuilder, TextSpan};

impl TagEditorActivity {
    pub fn add_lyric_options(&mut self, items: Vec<SongTag>) {
        self.lyric_options = items;
        self.sync_items();
    }

    pub fn sync_items(&mut self) {
        let mut table: TableBuilder = TableBuilder::default();

        for (idx, record) in self.lyric_options.iter().enumerate() {
            if idx > 0 {
                table.add_row();
            }

            table.add_col(TextSpan::from(format!("{}", record)));
        }
        let table = table.build();

        if let Some(props) = self.view.get_props(super::COMPONENT_TE_SCROLLTABLE_OPTIONS) {
            let props = scrolltable::ScrollTablePropsBuilder::from(props.clone())
                .with_table(Some(props.texts.title.unwrap()), table)
                .build();
            self.view
                .update(super::COMPONENT_TE_SCROLLTABLE_OPTIONS, props);
        }
    }
}
