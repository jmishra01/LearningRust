use std::cell::RefCell;

struct Software {
    name: String,
    version: usize,
}

impl Software {
    fn show(&self) {
        println!("{}: {}", self.name, self.version);
    }
}

fn update_name(software: &RefCell<Software>) {
    software.borrow_mut().name = "Microsoft".to_string();
    update_version(software);
}

fn update_version(software: &RefCell<Software>) {
    software.borrow_mut().version = 2;
}


fn main() {
    let software = Software {
        name: "Apple".to_string(),
        version: 1,
    };
    software.show();
    let refcell = RefCell::new(software);

    {
        // Without curley bracket this code will not compile
        let b = refcell.borrow();
        b.show();
    }
    // OR, write in the below way
    refcell.borrow().show();

    update_name(&refcell);
    
    // Show updated value
    refcell.borrow().show();
}

