#[macro_use]
extern crate criterion;
extern crate test_proto_bench;

use criterion::Criterion;
use prost::Message;

use test_proto_bench::pr::raft_cmdpb::{RaftCmdRequest, Request, RaftRequestHeader, GetRequest, PutRequest, AdminRequest};
use test_proto_bench::pr::eraftpb::Message as EraMessage;
use test_proto_bench::pr::eraftpb::{Entry, Snapshot};

pub fn mock_vec() -> Vec<u8> {
    let mut data = Vec::with_capacity(1000);
    for i in 0..data.len() {
        data.push(i as u8);
    }
    data
}

fn mock_req_header() -> RaftRequestHeader {
    RaftRequestHeader {
        region_id: 2,
        peer: None,
        read_quorum: true,
        uuid: vec![1, 2, 3, 4],
        region_epoch: None,
        term: 2,
        sync_log: false,
    }
}

fn mock_get_req() -> GetRequest {
    GetRequest {
        cf: String::from("Jojo! This is the last of my hamon!"),
        key: mock_vec(),
    }
}

fn mock_put_req() -> PutRequest {
    let data = mock_vec();
    PutRequest {
        cf: String::from("I reject my humanity, Jojo!"),
        key: data.clone(),
        value: data,
    }
}

fn mock_req() -> Request {
    Request {
        cmd_type: 3,
        get: Some(mock_get_req()),
        put: Some(mock_put_req()),
        delete: None,
        snap: None,
        prewrite: None,
        delete_range: None,
        ingest_sst: None,
    }
}

fn mock_admin_req() -> AdminRequest {
    AdminRequest {
        cmd_type: 6,
        change_peer: None,
        split: None,
        compact_log: None,
        transfer_leader: None,
        verify_hash: None,
        prepare_merge: None,
        commit_merge: None,
        rollback_merge: None,
        splits: None,
    }
}

fn mock_raft_cmd_req() -> RaftCmdRequest {
    RaftCmdRequest {
        header: Some(mock_req_header()),
        requests: vec![mock_req(), mock_req()],
        admin_request: Some(mock_admin_req()),
        status_request: None,
    }
}

fn mock_entry() -> Entry {
    let data = mock_vec();
    Entry {
        entry_type: 1,
        term: 7,
        index: 8,
        data: data.clone(),
        context: data,
        sync_log: true,
    }
}

fn mock_snapshot() -> Snapshot {
    Snapshot {
        data: mock_vec(),
        metadata: None,
    }
}

fn mock_message() -> EraMessage {
    EraMessage {
        msg_type: 8,
        to: 1,
        from: 2,
        term: 3,
        log_term: 4,
        index: 5,
        entries: vec![mock_entry(), mock_entry()],
        commit: 6,
        snapshot: Some(mock_snapshot()),
        reject: false,
        reject_hint: 9,
        context: mock_vec(),
    }
}

fn encode_message(test: &EraMessage) {
    let mut buf = Vec::with_capacity(1024);
    test.encode(&mut buf).unwrap();
}

fn decode_message(test: &Vec<u8>) {
    EraMessage::decode(test).unwrap();
}

fn encode_raft_cmd_req(test: &RaftCmdRequest) {
    let mut buf = Vec::with_capacity(2048);
    test.encode(&mut buf).unwrap();
}

fn decode_raft_cmd_req(test: &Vec<u8>) {
    RaftCmdRequest::decode(test).unwrap();
}

fn criterion_bench(c: &mut Criterion) {
    c.bench_function("pr ser eraftpb::message", |b|
        {
            let message = mock_message();
            b.iter(|| encode_message(&message))
        });
    c.bench_function("pr de  eraftpb::message", |b| {
        let message = mock_message();
        let mut message_buf = Vec::with_capacity(1024);
        message.encode(&mut message_buf).unwrap();
        b.iter(|| decode_message(&message_buf))
    });
    c.bench_function("pr ser raft_cmdpb::req", |b| {
        let raft_cmd_request = mock_raft_cmd_req();
        b.iter(|| encode_raft_cmd_req(&raft_cmd_request))
    });
    c.bench_function("pr de  raft_cmdpb::req", |b| {
        let raft_cmd_request = mock_raft_cmd_req();
        let mut raft_cmd_request_buf = Vec::with_capacity(2048);
        raft_cmd_request.encode(&mut raft_cmd_request_buf).unwrap();
        b.iter(|| decode_raft_cmd_req(&raft_cmd_request_buf))
    });
    c.bench_function("pr set repeated field", |b| {
        let mut raft_cmd_request = mock_raft_cmd_req();
        b.iter(|| raft_cmd_request.requests = vec![])
    });
}

criterion_group!(benches, criterion_bench);
criterion_main!(benches);
