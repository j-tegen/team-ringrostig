use juniper::{EmptyMutation, RootNode};

struct Fruit {
    id: i32,
    name: String,
    description: String,
    video_url: String,
    video_thumb: String,
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

pub struct QueryRoot;

#[juniper::object]
impl QueryRoot {
    fn fruits() -> Vec<Fruit> {
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
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<()>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new())
}