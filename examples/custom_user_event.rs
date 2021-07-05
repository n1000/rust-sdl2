fn main() {
    struct SomeCustomEvent {
        a: i32,
        b: i32,
    }

    let sdl = sdl2::init().unwrap();
    let ev = sdl.event().unwrap();
    let mut ep = sdl.event_pump().unwrap();

    let mut cust_event_storage = sdl2::event::CustomEventStorage::new();

    ev.register_custom_event::<SomeCustomEvent>().unwrap();

    let event = SomeCustomEvent { a: 42, b: 10 };

    ev.push_custom_event(event, &mut cust_event_storage).expect("push_custom_event failure");

    let received = ep.poll_event().unwrap(); // or within a for event in ep.poll_iter()
    let received_2 = received.clone();

    if received.is_user_event() {
        // peek at the event, doesn't consume it
        let e1 = received.as_user_event_type_peek::<SomeCustomEvent>(&mut cust_event_storage).unwrap();
        assert_eq!(e1.a, 42);
        assert_eq!(e1.b, 10);

        // take the event, consumes it.
        let e2 = received.as_user_event_type::<SomeCustomEvent>(&mut cust_event_storage).unwrap();
        assert_eq!(e2.a, 42);
        assert_eq!(e2.b, 10);

        let e2 = received.as_user_event_type::<SomeCustomEvent>(&mut cust_event_storage);
        // already consumed on line 23
        assert!(e2.is_none());
    }

    if received_2.is_user_event() {
        let e2 = received_2.as_user_event_type::<SomeCustomEvent>(&mut cust_event_storage);
        // already consumed on line 23
        assert!(e2.is_none());
    }
}
