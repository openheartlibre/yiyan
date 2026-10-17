use super::super::{Candidate, CandidateKind};
use super::Cell;

/// 本地候选 + 云端词的分页排布。索引空间是「格」：第一页先本地后云端，之后各页全是本地候选。
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CandidateLayout {
    /// 本地候选，顺序就是 Engine 排好的顺序。
    local: Vec<Candidate>,

    /// 已到的云端词（≤ 槽位数，与本地候选去过重）。
    cloud: Vec<Candidate>,

    /// 敲的字母本身，占每页最后一格；`None` 是开关关着或没什么可放。
    raw: Option<Candidate>,

    /// 每页几格。
    page_size: usize,

    /// 配置的云端槽位数；0 表示不要云端词。
    slots: usize,
}

impl CandidateLayout {
    pub fn new(local: Vec<Candidate>, page_size: usize, slots: usize) -> Self {
        Self {
            local,
            cloud: Vec::new(),
            raw: None,
            page_size: page_size.max(1),
            slots,
        }
    }

    /// 把「敲的字母本身」固定到每页最后一格（[`crate::Engine::raw_candidate`]）。
    /// 每页只剩一格、或唯一的候选就是这串字母（英文直输段）时不放。
    ///
    /// 别的情况不去重：那串字母原样上屏过几次就进了个人英文词表（`mtp`），之后它作为英文候选
    /// 排在前面，去重会让最后一格时有时无——「最后一格永远是敲的字母」比不重复更要紧。
    pub fn with_raw(mut self, raw: Option<Candidate>) -> Self {
        self.raw = raw
            .filter(|raw| self.page_size > 1 && self.local.iter().map(|c| &c.text).ne([&raw.text]));
        self
    }

    pub fn page_size(&self) -> usize {
        self.page_size
    }

    pub fn local(&self) -> &[Candidate] {
        &self.local
    }

    pub fn cloud(&self) -> &[Candidate] {
        &self.cloud
    }

    pub fn raw(&self) -> Option<&Candidate> {
        self.raw.as_ref()
    }

    /// 每页能放几格本地候选与云端词：留着字母候选时最后一格不算。
    fn usable(&self) -> usize {
        self.page_size - usize::from(self.raw.is_some())
    }

    /// 第一页最多给云端几格：本地候选至少占第一格；没有本地候选（问字模式）时整页都给云端。
    pub fn capacity(&self) -> usize {
        if self.local.is_empty() {
            self.usable()
        } else {
            let fixed = self
                .local
                .iter()
                .filter_map(|c| match c.kind {
                    CandidateKind::Custom(n) => Some(n),
                    _ => None,
                })
                .max()
                .unwrap_or(1);
            self.slots.min(self.usable().saturating_sub(fixed.max(1)))
        }
    }

    /// 云端词到了：与本地候选同文的不要（本地已经能给），其余按顺序填进第一页末尾，多出来的丢掉。返回填进去的条数。
    pub fn set_cloud(&mut self, words: Vec<Candidate>) -> usize {
        self.cloud.clear();
        let capacity = self.capacity();
        for mut word in words {
            if self.cloud.len() >= capacity
                || self.local.iter().any(|c| c.text == word.text)
                || self.cloud.iter().any(|c| c.text == word.text)
            {
                continue;
            }
            word.kind = CandidateKind::Cloud;
            self.cloud.push(word);
        }
        self.cloud.len()
    }

    /// 本地格数包含最大固定位置之前的空格。
    fn local_len(&self) -> usize {
        self.local
            .iter()
            .filter_map(|c| match c.kind {
                CandidateKind::Custom(position) => Some(position),
                _ => None,
            })
            .max()
            .unwrap_or(0)
            .max(self.local.len())
    }

    /// 真实本地候选按固定位置放置，其余候选依次填入空格。
    fn local_cells(&self) -> Vec<Cell<'_>> {
        let mut cells = vec![Cell::Empty; self.local_len()];
        for candidate in &self.local {
            if let CandidateKind::Custom(position) = candidate.kind
                && let Some(cell) = position.checked_sub(1).and_then(|i| cells.get_mut(i))
            {
                *cell = Cell::Local(candidate);
            }
        }
        let mut normal = self
            .local
            .iter()
            .filter(|c| !matches!(c.kind, CandidateKind::Custom(_)));
        for cell in &mut cells {
            if matches!(cell, Cell::Empty)
                && let Some(candidate) = normal.next()
            {
                *cell = Cell::Local(candidate);
            }
        }
        cells
    }

    /// 全部格子按索引顺序排开；空位只属于布局。
    pub fn cells(&self) -> Vec<Cell<'_>> {
        let mut cells = self.local_cells();
        let first = cells.len().min(self.usable() - self.cloud.len());
        cells.splice(first..first, self.cloud.iter().map(Cell::Cloud));
        let Some(raw) = &self.raw else {
            return cells;
        };
        // 每页最后一格：不足一页的末页也补上，字母候选跟在最后一个真实候选后面而不是空着等到第 9 格
        let mut with_raw = Vec::with_capacity(self.len());
        for page in cells.chunks(self.usable()) {
            with_raw.extend_from_slice(page);
            with_raw.push(Cell::Local(raw));
        }
        if with_raw.is_empty() {
            with_raw.push(Cell::Local(raw));
        }
        with_raw
    }

    pub fn len(&self) -> usize {
        let cells = self.local_len() + self.cloud.len();
        match self.raw {
            Some(_) => cells + cells.div_ceil(self.usable()).max(1),
            None => cells,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn pages(&self) -> usize {
        self.len().div_ceil(self.page_size)
    }

    /// 第 `index` 格的候选；越界返回 `None`。
    pub fn candidate(&self, index: usize) -> Option<&Candidate> {
        self.cells().get(index).and_then(|cell| cell.candidate())
    }

    /// 第 `page` 页的格子。
    pub fn page(&self, page: usize) -> Vec<Cell<'_>> {
        self.cells()
            .into_iter()
            .skip(page * self.page_size)
            .take(self.page_size)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn local(text: &str) -> Candidate {
        Candidate {
            text: text.into(),
            kind: CandidateKind::Chinese,
            syllables: vec!["zhang".into(), "tao".into()],
            reading: None,
            translation: None,
            aux_code: None,
        }
    }

    fn cloud(text: &str) -> Candidate {
        Candidate {
            kind: CandidateKind::Cloud,
            ..local(text)
        }
    }

    fn raw(text: &str) -> Candidate {
        Candidate {
            kind: CandidateKind::Raw,
            syllables: Vec::new(),
            ..local(text)
        }
    }

    fn texts(cells: &[Cell<'_>]) -> Vec<String> {
        cells
            .iter()
            .map(|cell| match cell {
                Cell::Local(c) if c.kind == CandidateKind::Raw => format!("＿{}", c.text),
                Cell::Local(c) => c.text.clone(),
                Cell::Cloud(c) => format!("☁{}", c.text),
                Cell::Empty => "<empty>".into(),
            })
            .collect()
    }

    fn many(count: usize) -> Vec<Candidate> {
        (0..count).map(|i| local(&format!("本{i}"))).collect()
    }

    #[test]
    fn cloud_words_take_the_end_of_the_first_page_and_only_bump_those_cells() {
        let mut layout = CandidateLayout::new(many(12), 9, 3);
        assert_eq!(
            texts(&layout.page(0)),
            [
                "本0", "本1", "本2", "本3", "本4", "本5", "本6", "本7", "本8"
            ]
        );
        assert_eq!(layout.pages(), 2);

        // 四条云端词只取前三条；前六格不动，原来 7–9 格的本地候选挪到第二页开头
        let filled = layout.set_cloud(vec![cloud("云0"), cloud("云1"), cloud("云2"), cloud("云3")]);
        assert_eq!(filled, 3);
        assert_eq!(
            texts(&layout.page(0)),
            [
                "本0", "本1", "本2", "本3", "本4", "本5", "☁云0", "☁云1", "☁云2"
            ]
        );
        assert_eq!(
            texts(&layout.page(1)),
            ["本6", "本7", "本8", "本9", "本10", "本11"]
        );
        assert_eq!(layout.candidate(6).unwrap().text, "云0");
        assert_eq!(layout.candidate(9).unwrap().text, "本6");
    }

    #[test]
    fn duplicates_of_local_candidates_are_dropped_and_fewer_words_take_fewer_cells() {
        let mut layout = CandidateLayout::new(many(4), 9, 2);
        assert_eq!(layout.set_cloud(vec![cloud("本2"), cloud("云0")]), 1);
        assert_eq!(texts(&layout.page(0)), ["本0", "本1", "本2", "本3", "☁云0"]);
        assert_eq!(layout.local()[2].kind, CandidateKind::Chinese);
        assert_eq!(layout.pages(), 1);
    }

    #[test]
    fn no_words_means_nothing_changes() {
        let mut layout = CandidateLayout::new(many(12), 9, 2);
        assert_eq!(layout.set_cloud(Vec::new()), 0);
        assert_eq!(layout.page(0).len(), 9);
        assert_eq!(texts(&layout.page(1)), ["本9", "本10", "本11"]);
    }

    #[test]
    fn zero_slots_means_no_cloud_cells_at_all() {
        let mut layout = CandidateLayout::new(many(3), 9, 0);
        assert_eq!(layout.set_cloud(vec![cloud("云0")]), 0);
        assert_eq!(texts(&layout.page(0)), ["本0", "本1", "本2"]);
    }

    #[test]
    fn without_local_candidates_the_whole_page_goes_to_the_cloud() {
        let mut layout = CandidateLayout::new(Vec::new(), 9, 2);
        assert_eq!(
            layout.set_cloud(vec![cloud("森"), cloud("淼"), cloud("焱")]),
            3
        );
        assert_eq!(texts(&layout.page(0)), ["☁森", "☁淼", "☁焱"]);
        assert_eq!(layout.candidate(0).unwrap().text, "森");
    }

    #[test]
    fn slots_never_push_the_first_local_candidate_off_the_first_page() {
        let mut layout = CandidateLayout::new(many(5), 3, 5);
        assert_eq!(layout.capacity(), 2);
        layout.set_cloud(vec![cloud("云0"), cloud("云1"), cloud("云2")]);
        assert_eq!(texts(&layout.page(0)), ["本0", "☁云0", "☁云1"]);
        assert_eq!(texts(&layout.page(1)), ["本1", "本2", "本3"]);
    }

    #[test]
    fn sparse_custom_positions_are_layout_cells_not_candidates() {
        let fixed = Candidate {
            kind: CandidateKind::Custom(3),
            ..local("短语")
        };
        let mut layout = CandidateLayout::new(vec![fixed, local("普通")], 5, 2);
        assert_eq!(layout.local().len(), 2);
        assert_eq!(texts(&layout.page(0)), ["普通", "<empty>", "短语"]);
        assert!(layout.candidate(1).is_none());
        assert_eq!(layout.set_cloud(vec![cloud("云")]), 1);
        assert_eq!(texts(&layout.page(0)), ["普通", "<empty>", "短语", "☁云"]);
    }

    #[test]
    fn the_raw_letters_take_the_last_cell_of_every_page() {
        let layout = CandidateLayout::new(many(12), 9, 0).with_raw(Some(raw("mtp")));
        assert_eq!(
            texts(&layout.page(0)),
            [
                "本0", "本1", "本2", "本3", "本4", "本5", "本6", "本7", "＿mtp"
            ]
        );
        assert_eq!(
            texts(&layout.page(1)),
            ["本8", "本9", "本10", "本11", "＿mtp"]
        );
        assert_eq!(layout.pages(), 2);
        assert_eq!(layout.len(), 14);
        assert_eq!(layout.candidate(8).unwrap().kind, CandidateKind::Raw);
        assert_eq!(layout.candidate(13).unwrap().text, "mtp");
    }

    #[test]
    fn cloud_words_stay_on_the_first_page_next_to_the_raw_cell() {
        let mut layout = CandidateLayout::new(many(12), 9, 2).with_raw(Some(raw("mtp")));
        assert_eq!(layout.capacity(), 2);
        assert_eq!(layout.set_cloud(vec![cloud("云0"), cloud("云1")]), 2);
        assert_eq!(
            texts(&layout.page(0)),
            [
                "本0", "本1", "本2", "本3", "本4", "本5", "☁云0", "☁云1", "＿mtp"
            ]
        );
        assert_eq!(
            texts(&layout.page(1)),
            ["本6", "本7", "本8", "本9", "本10", "本11", "＿mtp"]
        );
    }

    #[test]
    fn the_raw_cell_shows_up_even_without_candidates_but_never_duplicates_one() {
        let layout = CandidateLayout::new(Vec::new(), 9, 0).with_raw(Some(raw("no-way")));
        assert_eq!(texts(&layout.page(0)), ["＿no-way"]);
        assert_eq!(layout.len(), 1);
        assert!(!layout.is_empty());

        // 英文直输段的唯一候选就是那串字母：不再多放一条
        let layout =
            CandidateLayout::new(vec![local("no-way")], 9, 0).with_raw(Some(raw("no-way")));
        assert_eq!(texts(&layout.page(0)), ["no-way"]);

        // 同一串字母已经进了个人英文词表、作为候选排在前面：最后一格照放，位置不能时有时无
        let layout = CandidateLayout::new(vec![local("mtp"), local("每天跑")], 9, 0)
            .with_raw(Some(raw("mtp")));
        assert_eq!(texts(&layout.page(0)), ["mtp", "每天跑", "＿mtp"]);

        // 一页只有一格时放不下，开关当作没开
        let layout = CandidateLayout::new(many(2), 1, 0).with_raw(Some(raw("mtp")));
        assert_eq!(texts(&layout.page(0)), ["本0"]);
        assert_eq!(layout.len(), 2);
    }

    #[test]
    fn ninth_position_stays_on_second_page_without_cloud_displacement() {
        let mut layout = CandidateLayout::new(
            vec![Candidate {
                kind: CandidateKind::Custom(9),
                ..local("第九")
            }],
            5,
            2,
        );
        assert_eq!(layout.pages(), 2);
        assert_eq!(layout.capacity(), 0);
        assert_eq!(layout.set_cloud(vec![cloud("云")]), 0);
        assert_eq!(layout.candidate(8).unwrap().text, "第九");
        assert!(layout.candidate(7).is_none());
    }
}
