use openlineage_sql::{parse_sql, QueryMetadata};

#[test]
fn select_simple() {
    assert_eq!(
        parse_sql("SELECT * FROM table0;").unwrap(),
        QueryMetadata {
            inputs: vec![String::from("table0")],
            output: None
        }
    )
}
#[test]
fn select_from_schema_table() {
    assert_eq!(
        parse_sql("SELECT * FROM schema0.table0;").unwrap(),
        QueryMetadata {
            inputs: vec![String::from("schema0.table0")],
            output: None
        }
    )
}
#[test]
fn select_join() {
    assert_eq!(
        parse_sql(
            "
                SELECT col0, col1, col2
                FROM table0
                JOIN table1
                ON t1.col0 = t2.col0"
        )
        .unwrap(),
        QueryMetadata {
            inputs: vec![String::from("table0"), String::from("table1")],
            output: None
        }
    )
}

#[test]
fn select_inner_join() {
    assert_eq!(
        parse_sql(
            "
                SELECT col0, col1, col2
                FROM table0
                INNER JOIN table1
                ON t1.col0 = t2.col0"
        )
        .unwrap(),
        QueryMetadata {
            inputs: vec![String::from("table0"), String::from("table1")],
            output: None
        }
    )
}

#[test]
fn select_left_join() {
    assert_eq!(
        parse_sql(
            "
            SELECT col0, col1, col2
            FROM table0
            LEFT JOIN table1
            ON t1.col0 = t2.col0"
        )
        .unwrap(),
        QueryMetadata {
            inputs: vec![String::from("table0"), String::from("table1")],
            output: None
        }
    )
}

#[test]
fn select_bigquery_excaping() {
    assert_eq!(
        parse_sql(
            "
            SELECT *
            FROM `random-project`.`dbt_test1`.`source_table`
            WHERE id = 1
        "
        )
        .unwrap(),
        QueryMetadata {
            inputs: vec![String::from("`random-project`.`dbt_test1`.`source_table`")],
            output: None
        }
    )
}