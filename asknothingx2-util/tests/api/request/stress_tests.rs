use asknothingx2_util::api::request::RequestBody;
use bytes::Bytes;
use tokio::task::JoinSet;

#[tokio::test]
async fn test_concurrent_body_operations() {
    let mut tasks = JoinSet::new();

    for i in 0..100 {
        tasks.spawn(async move {
            let data = format!("concurrent test data {i}");
            let body = RequestBody::from_string(data.clone());

            assert!(!body.is_empty());
            assert_eq!(body.content_length(), Some(data.len() as u64));

            let _display = body.to_string();
            let _debug = format!("{body:?}");

            i
        });
    }

    let mut completed = 0;
    while let Some(result) = tasks.join_next().await {
        let _task_id = result.unwrap();
        completed += 1;
    }

    assert_eq!(completed, 100);
}

#[test]
fn test_memory_stress() {
    for i in 0..1000 {
        let data = format!("stress test iteration {i}").repeat(100);
        let body = RequestBody::from_string(data);
        assert!(!body.is_empty());
    }
}

#[cfg(feature = "stream")]
#[test]
fn test_large_bytes_iterator_stress() {
    let mut chunks = Vec::new();
    for i in 0..10000 {
        chunks.push(Bytes::from(format!("chunk{i}")));
    }

    let body = RequestBody::from_bytes_iter(chunks.clone());
    let expected_length: usize = chunks.iter().map(|b| b.len()).sum();
    assert_eq!(body.content_length(), Some(expected_length as u64));
}
