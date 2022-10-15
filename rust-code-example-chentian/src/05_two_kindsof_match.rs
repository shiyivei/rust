// 数据结构

#[allow(unused)]
#[derive(Debug)]
enum Gender {
    Unspecified,
    Male,
    Female,
}

#[derive(Debug, Copy, Clone)]
struct UserId(u64);

#[derive(Debug, Copy, Clone)]
struct TopicId(u64);

#[allow(unused)]
#[derive(Debug)]

struct Topic {
    id: TopicId,
    name: String,
    owner: UserId,
}

#[allow(unused)]
#[derive(Debug)]

enum Event {
    Join(UserId, TopicId),
    Leave(UserId, TopicId),
    Message(UserId, TopicId, String),
}

#[allow(unused)]
#[derive(Debug)]
struct User {
    id: UserId,
    name: String,
    gender: Gender,
}
fn process_event_match_case(event: Event) {
    match event {
        Event::Join(uid, _tid) => println!("User {:?} joined", uid),
        Event::Leave(uid, _tid) => println!("User {:?} left", uid),
        Event::Message(_, _, msg) => println!("broadcast: {:?}", msg),
    }
}

fn process_event_if_let_case(event: Event) {
    if let Event::Message(_, _, msg) = event {
        println!("broadcast: {:?}", msg)
    }
}

fn main() {
    let uid = UserId(1);
    let tid = TopicId(1);

    let msg = String::from("hello,any body here?");

    let e = Event::Message(uid, tid, msg);

    // 注意，结构体的实例化和枚举的实例化并不相同
    let event = Event::Join(uid, tid);

    process_event_match_case(event);
    process_event_if_let_case(e)
}