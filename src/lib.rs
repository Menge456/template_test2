use portus::ipc::Ipc;
use portus::lang::Scope;
use portus::{CongAlg, Datapath, DatapathInfo, DatapathTrait, Report};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tracing::{info, warn};



#[derive(Clone)]
pub struct TestConfig {
    pub our_packets:u32,
    pub other_packets:u32,
}

pub struct Test<T: Ipc> {
    our_packets: u32,
    other_packets: u32,
    sc: Scope,
    control_channel: Datapath<T>,

}


impl<T: Ipc> CongAlg<T> for TestConfig {
    type Flow = Test<T>;

    fn name() -> &'static str {
        "Test"
    }

    fn datapath_programs(&self) -> HashMap<&'static str, String> {
        let mut h = HashMap::default();
        h.insert(
            "TestBasicSerialize",
            "
            (def (Report.which 2))
            (when true
                (report)
            )"
            .to_owned(),
        );
        h
    }

    fn new_flow(&self, control: Datapath<T>, _info: DatapathInfo) -> Self::Flow {
        info!("hello");
        let s = Test{
            our_packets: 0,
            other_packets: 0,
            sc: Scope::new(),
            control_channel:control,
        };
        s
    }
}

impl <T:Ipc> portus::Flow for Test<T>{
    fn on_report(&mut self, _sock_id: u32, m: Report) {
        print!("hello");
        let which = m
            .get_field(&String::from("Report.which"), &self.sc)
            .expect("expected acked field in returned measurement") as u32;
        if which == 1{
            self.our_packets += 1;
        }else{
            self.other_packets += 1;
        }
        let our_packets = self.our_packets;
        let other_packets = self.other_packets;
        info!("ours: {our_packets}\ntheirs: {other_packets}");
        if let Err(e) = self
            .control_channel
            .update_field(&self.sc, &[("Cwnd", 8)])
        {
            warn!(err = ?e, "Cwnd update error");
        }
    }
}
