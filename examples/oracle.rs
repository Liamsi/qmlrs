#[macro_use]
extern crate qmlrs;

struct DummyOracle;
impl DummyOracle {
    fn calculate(&self, x: bool) -> bool {
        x
    }
}

Q_OBJECT! { DummyOracle:
    slot fn calculate(bool);
//    signal fn test();
}

fn main() {
    let mut engine = qmlrs::Engine::new();

    engine.set_property("oracle", DummyOracle);
    engine.load_local_file("examples/oracle_ui.qml");

    engine.exec();
}
