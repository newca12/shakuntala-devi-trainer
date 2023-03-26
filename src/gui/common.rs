#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    #[default]
    Game,
    Solution,
    TrainingMonthTable,
    TrainingYearTable,
}
