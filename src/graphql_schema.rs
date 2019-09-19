use juniper::{EmptyMutation, RootNode};

struct Fruit {
    id: i32,
    name: String,
    description: String,
    video_url: String,
    video_thumb: String,
}

struct User {
    id: i32,
    name: String,
}

struct FruitOfTheDay {
    id: i32,
    user_id: i32,
    fruit_id: i32,
    time_stamp: String,
}

#[juniper::object(description = "A fruit!")]
impl Fruit {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    pub fn video_url(&self) -> &str {
        self.video_url.as_str()
    }

    pub fn video_thumb(&self) -> &str {
        self.video_thumb.as_str()
    }
}

#[juniper::object(description = "A user!")]
impl User {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}

#[juniper::object(description = "A fruit of the day!")]
impl FruitOfTheDay {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn user(&self) -> User {
        get_users()
            .into_iter()
            .find(|user| user.id == self.user_id)
            .unwrap()
    }

    pub fn time_stamp(&self) -> &str {
        self.time_stamp.as_str()
    }

    pub fn fruit(&self) -> Fruit {
        all_fruits()
            .into_iter()
            .find(|fruit| fruit.id == self.fruit_id)
            .unwrap()
    }
}

pub struct QueryRoot;

#[juniper::object]
impl QueryRoot {
    fn fruits() -> Vec<Fruit> {
        all_fruits()
    }

    fn fruit(id: i32) -> Fruit {
        all_fruits()
            .into_iter()
            .find(|fruit| fruit.id == id)
            .unwrap()
    }

    fn fruitsOfTheDay() -> Vec<FruitOfTheDay> {
        all_fruits_of_the_day()
    }
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<()>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new())
}

fn all_fruits() -> Vec<Fruit> {
    vec![
        Fruit {
            id: 1,
            name: "Apple".to_owned(),
            description: "An apple a day keeps the doctor away!".to_owned(),
            video_url: "https://www.youtube.com/watch?v=HD4o26HcHi0".to_owned(),
            video_thumb: "https://i.ytimg.com/an_webp/HD4o26HcHi0/mqdefault_6s.webp?du=3000&sqp=CNjvjOwF&rs=AOn4CLDXXfFl6DmdDeeYW8keteLo4-sV_A".to_owned(),
        },
        Fruit {
            id: 2,
            name: "Orange".to_owned(),
            description: "Orange in swedisih literally means chinese apple!".to_owned(),
            video_url: "https://www.youtube.com/watch?v=ZN5PoW7_kdA".to_owned(),
            video_thumb: "https://i.ytimg.com/an_webp/ZN5PoW7_kdA/mqdefault_6s.webp?du=3000&sqp=CICLjewF&rs=AOn4CLD6fMuupHGmNwKcCvbfQOKR0hYwuA".to_owned(),
        },
    ]
}

fn all_fruits_of_the_day() -> Vec<FruitOfTheDay> {
    vec![FruitOfTheDay {
        id: 1,
        user_id: 2,
        time_stamp: "2019-09-19 22:00".to_owned(),
        fruit_id: 2,
    }]
}

fn get_users() -> Vec<User> {
    vec![
        User {
            id: 1,
            name: "Kalle".to_owned(),
        },
        User {
            id: 2,
            name: "Ramona".to_owned(),
        },
        User {
            id: 3,
            name: "Jonte".to_owned(),
        },
    ]
}
