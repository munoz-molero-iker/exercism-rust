#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq + ToString + std::fmt::Display>(_first_list: &[T], _second_list: &[T]) -> Comparison {

    let _first_list_string: String = list_to_string(_first_list);
    let _second_list_string: String = list_to_string(_second_list);

    if _first_list_string == _second_list_string { return Comparison::Equal; }
    if _first_list_string.contains(&_second_list_string) { return Comparison::Superlist; }
    if _second_list_string.contains(&_first_list_string) { return Comparison::Sublist; }
    Comparison::Unequal

}

fn list_to_string<T: PartialEq + ToString + std::fmt::Display>(_list: &[T]) -> String {

    _list.iter().map(|number| format!("{:0width$}", number, width = 3)).collect::<Vec<String>>().join(",")

}