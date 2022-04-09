trait Action {
    fn act(&self);
}

struct DefaultAction {
    content: String,
}

impl Action for DefaultAction {
    fn act(&self) {
        println!("do {}", self.content);
    }
}

struct User<'a> {
    action: &'a Action,
}

impl<'a> User<'a> {
    pub fn user_act(&self) {
        self.action.act()
    }
}


struct Class<'a> {
    name: &'a str,
}

impl<'a> Class<'a> {
    fn new() ->Self{
        Self{
            name: "zhang"
        }
    }
}


fn main() {
    let action = DefaultAction {
        content: "hello Rust".to_string(),
    };
    let user = User {
        action: &action,
    };
    user.user_act();

    let class = Class::new();

}
