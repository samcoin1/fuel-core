use fuel_client::client::FuelClient;
use fuel_core::service::{configure, run_in_background};
use fuel_vm::consts::*;
use fuel_vm::prelude::*;
use std::convert::TryInto;

#[tokio::test]
async fn start_session() {
    let srv = run_in_background(configure(Default::default())).await;
    let client = FuelClient::from(srv);

    let session = client.start_session().await.unwrap();
    let session_p = client.start_session().await.unwrap();

    let id = session.as_str();
    let id_p = session_p.as_str();

    assert_ne!(id, id_p);
}

#[tokio::test]
async fn end_session() {
    let srv = run_in_background(configure(Default::default())).await;
    let client = FuelClient::from(srv);

    let session = client.start_session().await.unwrap();
    let id = session.as_str();

    assert!(client.end_session(id).await.unwrap());
    assert!(!client.end_session(id).await.unwrap());
}

#[tokio::test]
async fn reset() {
    let srv = run_in_background(configure(Default::default())).await;
    let client = FuelClient::from(srv);

    let session = client.start_session().await.unwrap();
    let id = session.as_str();

    let register = client.register(id, 0x10).await.unwrap();
    assert_eq!(0x00, register);

    let result = client
        .execute(id, &Opcode::ADDI(0x10, 0x10, 0xfa))
        .await
        .unwrap();
    assert!(result);

    let register = client.register(id, 0x10).await.unwrap();
    assert_eq!(0xfa, register);

    let result = client
        .execute(id, &Opcode::ADDI(0x11, 0x11, 0x08))
        .await
        .unwrap();
    assert!(result);

    let result = client.execute(id, &Opcode::ALOC(0x11)).await.unwrap();
    assert!(result);

    let result = client
        .execute(id, &Opcode::ADDI(0x11, REG_HP, 1))
        .await
        .unwrap();
    assert!(result);

    let result = client
        .execute(id, &Opcode::SW(0x11, 0x10, 0))
        .await
        .unwrap();
    assert!(result);

    let memory = client.register(id, 0x11).await.unwrap();
    let memory = client.memory(id, memory as RegisterId, 8).await.unwrap();
    let memory = Word::from_be_bytes(memory.as_slice().try_into().unwrap());
    assert_eq!(0xfa, memory);

    let result = client.reset(id).await.unwrap();
    assert!(result);

    let register = client.register(id, 0x10).await.unwrap();
    assert_eq!(0x00, register);

    let result = client.end_session(id).await.unwrap();
    assert!(result);
}