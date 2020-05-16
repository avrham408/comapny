#[cfg(test)]

mod tests {

    #[test]
    fn test_valid_employee_not_exist() {
        let mut company = crate::create_company();
        assert_eq!(
            crate::valid_employee_not_exist(&company, &String::from("avi")),
            false
        );
        assert_eq!(
            crate::valid_employee_not_exist(&company, &String::from("dana")),
            true
        );
        crate::handle_messages(&String::from("add dana to engineering"), &mut company);
        assert_eq!(
            crate::valid_employee_not_exist(&company, &String::from("dana")),
            false
        );
    }

    #[test]
    fn test_valid_department_exist() {
        let mut company = crate::create_company();
        assert_eq!(
            crate::valid_department_exist(&company, &String::from("engineering")),
            true
        );
        assert_eq!(
            crate::valid_department_exist(&company, &String::from("building")),
            false
        );
        crate::handle_messages(&String::from("create building"), &mut company);
        assert_eq!(
            crate::valid_department_exist(&company, &String::from("building")),
            true
        );
    }
    #[test]
    fn test_add_employee_to_department() {
        let mut company = crate::create_company();
        assert_eq!(
            crate::add_employee_to_department(
                &mut company,
                &String::from("engineering"),
                &String::from("dana")
            ),
            true
        );
        assert!(company["engineering"][1] == "dana");
    }
}
