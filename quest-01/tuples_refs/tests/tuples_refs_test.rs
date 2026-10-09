use tuples_refs::*;

#[test]
fn test_student() {
    let student = Student(20, "Pedro".to_string(), "Domingos".to_string());

    assert_eq!(id(&student), 20);
    assert_eq!(first_name(&student), "Pedro");
    assert_eq!(last_name(&student), "Domingos");
}
