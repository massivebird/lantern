use ratatui::text::Line;
use strum::{Display, EnumIter, FromRepr};

#[derive(Default, Copy, Clone, Display, FromRepr, EnumIter, PartialEq, Eq)]
pub enum SelectedTab {
    #[default]
    Live,
    Log,
}

impl SelectedTab {
    pub fn next(self) -> Self {
        let current_idx: usize = self as usize;
        let next_idx: usize = current_idx.saturating_add(1);
        Self::from_repr(next_idx).unwrap_or(self)
    }

    pub fn prev(self) -> Self {
        let current_idx: usize = self as usize;
        let prev_idx: usize = current_idx.saturating_sub(1);
        Self::from_repr(prev_idx).unwrap_or(self)
    }

    pub fn title(self) -> Line<'static> {
        format!("  {self}  ").into()
    }
}
