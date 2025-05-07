use rust_todo::*; // Import your library (replace `rust_todo` with your crate name)

#[test]
fn test_todo_creation() {
    let todo = Todo::new("Test task".to_string(), 1);
    assert_eq!(todo.item, "Test task");
    assert_eq!(todo.id, 1);
    assert!(!todo.done);
}

#[test]
fn test_mark_done() {
    let mut todo = Todo::new("Test task".to_string(), 1);
    todo.mark_done();
    assert!(todo.done);
}

#[test]
fn test_mark_undone() {
    let mut todo = Todo::new("Test task".to_string(), 1);
    todo.mark_done();
    todo.mark_undone();
    assert!(!todo.done);
}

#[test]
fn test_to_string_done() {
    let mut todo = Todo::new("Test task".to_string(), 1);
    todo.mark_done();
    let output = todo.to_string();
    assert!(output.contains("T̶e̶s̶t̶ ̶t̶a̶s̶k̶"));
}

#[test]
fn test_to_string_not_done() {
    let todo = Todo::new("Test task".to_string(), 1);
    let output = todo.to_string();
    assert_eq!(output, "1 Test task");
}