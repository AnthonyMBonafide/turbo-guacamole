use std::{sync::mpsc, thread};

#[derive(Debug)]
enum Event {
    Update,
    Add,
    Delete,
    Expire,
}

#[derive(Debug)]
enum ConnectionType {
    ClientDirect(Vec<String>), // Collection of IP addresses in cluster
    Broadcast, // Send a broadcast message over the network and any live nodes will respond with
               // their IP address
}
trait Connection {
    fn connect(conn: ConnectionType) -> (mpsc::Sender<Event>, mpsc::Receiver<Event>);
}

/// Uses a bi-directional websocket to communicate to the rest of the cluster events
struct WebSocketConnection {
    /// channel for pushing incoming events to a listening consumer
    /// When changes happen to other nodes in the cluster, the event will be read in, serialized
    /// and sent to the RX side of the channel which can be used to update and come into sync with
    /// the cluster
    incoming_events: mpsc::Sender<Event>,

    /// channel for sending out events to the cluster
    /// When changes happen that need to be communicated out the user can send the event on the TX
    /// side of this channel
    outgoing_events: mpsc::Receiver<Event>,
}

impl Connection for WebSocketConnection {
    fn connect(conn: ConnectionType) -> (mpsc::Sender<Event>, mpsc::Receiver<Event>) {
        let (incoming_tx, incoming_rx) = mpsc::channel();
        let (outgoing_tx, outgoing_rx) = mpsc::channel();

        // Handle pushing out events
        thread::spawn(move || loop {
            match outgoing_rx.recv() {
                Ok(_) => todo!(),
                Err(_) => todo!(),
            }
        });

        // Handle reading in events
        thread::spawn(move || loop {
            // Make connection to other node(s) here and get next event
            println!("{:?}", conn);
            let new_event = Event::Update;
            let _ = incoming_tx.send(new_event);
        });

        (outgoing_tx, incoming_rx)
    }
}
