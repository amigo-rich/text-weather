pub enum Day {
    Today,
    Tomorrow,
    Overmorrow,
}

impl std::fmt::Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let day_rep = match self {
            Day::Today => "Today",
            Day::Tomorrow => "Tomorrow",
            Day::Overmorrow => "Overmorrow",
        };
        write!(f, "{}", day_rep)
    }
}
