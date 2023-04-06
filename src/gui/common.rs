use enum_map::Enum;

#[derive(Debug, Default, Enum, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    #[default]
    Game,
    Solution,
    TrainingMonthTable,
    TrainingYearTable,
}
