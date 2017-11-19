use prettytable::Table;
use prettytable::row::Row;
use prettytable::cell::Cell;

pub fn watched_collection() {
    println!("You have requested some saved items");
	 // Create the table
    let mut table = Table::new();

    // Add a row per time
    table.add_row(row!["ABC", "DEFG", "HIJKLMN"]);
    table.add_row(row!["foobar", "bar", "foo"]);
    // A more compicated way to add a row:
    table.add_row(Row::new(vec![
        Cell::new("foobar2"),
        Cell::new("bar2"),
        Cell::new("foo2")]));

    // Print the table to stdout
    table.printstd();
}