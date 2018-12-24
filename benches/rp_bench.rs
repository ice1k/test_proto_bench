#[macro_use]
extern crate criterion;
extern crate test_proto_bench;

use criterion::Criterion;
use protobuf::{Message, RepeatedField, parse_from_bytes};

use test_proto_bench::rp::raft_cmdpb::{RaftCmdRequest, Request, RaftRequestHeader, GetRequest, PutRequest, AdminRequest, CmdType, AdminCmdType};
use test_proto_bench::rp::eraftpb::Message as EraMessage;
use test_proto_bench::rp::eraftpb::{Entry, Snapshot, EntryType, MessageType};

fn mock_req_header() -> RaftRequestHeader {
    let mut a = RaftRequestHeader::new();
    a.region_id = 2;
    a.read_quorum = true;
    a.set_uuid(vec![1, 2, 3, 4]);
    a.term = 2;
    a.sync_log = false;
    a
}

fn mock_get_req() -> GetRequest {
    let mut a = GetRequest::new();
    a.cf = String::from("Jojo! This is the last of my hamon!");
    a.key = vec![1, 2, 3, 4];
    a
}

fn mock_put_req() -> PutRequest {
    let mut a = PutRequest::new();
    a.cf = String::from("Jojo! This is the last of my hamon!");
    a.key = vec![1, 2, 3, 4];
    a.value = vec![5, 4, 3, 2];
    a
}

fn mock_req() -> Request {
    let mut a = Request::new();
    a.cmd_type = CmdType::Put; // 3
    a.set_get(mock_get_req());
    a.set_put(mock_put_req());
    a
}

fn mock_admin_req() -> AdminRequest {
    let mut a = AdminRequest::new();
    a.cmd_type = AdminCmdType::VerifyHash; // 6
    a
}

fn mock_raft_cmd_req() -> RaftCmdRequest {
    let mut a = RaftCmdRequest::new();
    a.set_header(mock_req_header());
    a.set_requests(RepeatedField::from_vec(vec![mock_req(), mock_req()]));
    a.set_admin_request(mock_admin_req());
    a
}

fn mock_entry() -> Entry {
    let mut a = Entry::new();
    a.entry_type = EntryType::EntryConfChange; // 1
    a.term = 7;
    a.index = 8;
    a.data = vec![1, 2, 3];
    a.context = vec![2, 3, 4];
    a.sync_log = true;
    a
}

fn mock_snapshot() -> Snapshot {
    let mut a = Snapshot::new();
    a.data = vec![1, 2, 3];
    a
}

fn mock_message() -> EraMessage {
    let mut a = EraMessage::new();
    a.msg_type = MessageType::MsgHeartbeat; // 8
    a.to = 1;
    a.from = 2;
    a.term = 3;
    a.log_term = 4;
    a.index = 5;
    a.set_entries(RepeatedField::from_vec(vec![mock_entry(), mock_entry()]));
    a.commit = 6;
    a.set_snapshot(mock_snapshot());
    a.reject = false;
    a.reject_hint = 9;
    a.context = vec![2, 3, 4, 4, 5];
    a
}

fn encode_message(test: &EraMessage) {
    let mut buf = Vec::with_capacity(1024);
    test.write_to_vec(&mut buf).unwrap();
}

fn decode_message(test: &Vec<u8>) {
    let _: EraMessage = parse_from_bytes(test.as_slice()).unwrap();
}

fn encode_raft_cmd_req(test: &RaftCmdRequest) {
    let mut buf = Vec::with_capacity(2048);
    test.write_to_vec(&mut buf).unwrap();
}

fn decode_raft_cmd_req(test: &Vec<u8>) {
    let _: RaftCmdRequest = parse_from_bytes(test.as_slice()).unwrap();
}

fn criterion_bench(c: &mut Criterion) {
    c.bench_function("rp ser eraftpb::message", |b| {
        let message = mock_message();
        b.iter(|| encode_message(&message))
    });
    c.bench_function("rp de  eraftpb::message", |b| {
        let message_bytes = mock_message().write_to_bytes().unwrap();
        b.iter(|| decode_message(&message_bytes))
    });
    c.bench_function("rp ser raft_cmdpb::req", |b| {
        let raft_cmd_request = mock_raft_cmd_req();
        b.iter(|| encode_raft_cmd_req(&raft_cmd_request))
    });
    c.bench_function("rp de  raft_cmdpb::req", |b| {
        let raft_cmd_request_bytes = mock_raft_cmd_req().write_to_bytes().unwrap();
        b.iter(|| decode_raft_cmd_req(&raft_cmd_request_bytes))
    });
}

criterion_group!(benches, criterion_bench);
criterion_main!(benches);
