pub mod oop {  

    pub enum StakeholderRole {
        Engineer,
        Business,
        ProjectManager,
    }

    impl StakeholderRole {

        fn as_str(&self) -> &'static str {
            match *self { // *self has type Direction
                StakeholderRole::Engineer => "Engineer",
                StakeholderRole::Business => "Business",
                StakeholderRole::ProjectManager => "ProjectManager",
            }
        }

    }

    pub struct Stakeholder {
        pub first_name: String, 
        pub last_name: String,
        pub role: StakeholderRole, 
    }

    impl Stakeholder {

        pub fn info(&self) {
            println!(
                "First name: {}\nLast name: {}\nRole: {}\n",
                self.first_name, self.last_name, self.role.as_str()
            )
        }

        pub fn rename(&mut self, first_name: Option<String>, last_name: Option<String>) {
            
            match first_name {
                Some(x) => self.first_name = x,
                None => println!("Keeping original first name for {}", self.first_name)
            }
            
            match last_name {
                Some(x) => self.last_name = x,
                None => println!("Keeping original last name for {}", self.last_name)
            }
        }

    }

}


#[test]
fn test_stakeholder_rename_firstname() {
    let mut stakeholder_engineer = oop::Stakeholder {
        first_name: String::from("Zinedine"),
        last_name: String::from("Zidane"),
        role: oop::StakeholderRole::Engineer,
    };
    stakeholder_engineer.rename(Some("Emmanuel".to_string()), None);
    assert_eq!(stakeholder_engineer.first_name, "Emmanuel".to_string())
}