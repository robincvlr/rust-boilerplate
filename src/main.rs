mod oop;

fn main() {

    // Create a stakeholder -> Engineer
    let mut stakeholder_engineer = oop::oop::Stakeholder {
        first_name: String::from("Zinedine"),
        last_name: String::from("Zidane"),
        role: oop::oop::StakeholderRole::Engineer,
    };

    // Create a stakeholder -> Business
    let stakeholder_business = oop::oop::Stakeholder {
        first_name: String::from("Thierry"),
        last_name: String::from("Henry"),
        role: oop::oop::StakeholderRole::Business,
    };

    // Create a stakeholder -> ProjectManager
    let stakeholder_project_manager = oop::oop::Stakeholder {
        first_name: String::from("Didier"),
        last_name: String::from("Deschamps"),
        role: oop::oop::StakeholderRole::ProjectManager,
    };

    stakeholder_engineer.info();
    stakeholder_business.info();
    stakeholder_project_manager.info();

    // Rename one of the stakeholders
    stakeholder_engineer.rename(Some("Emmanuel".to_string()), None);
    stakeholder_engineer.info();

}
