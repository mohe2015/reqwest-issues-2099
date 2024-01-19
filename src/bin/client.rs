use std::time::Instant;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let num_tasks = 1020;

    let client = reqwest::ClientBuilder::new().build().unwrap();

    let (tx, mut rx) = tokio::sync::mpsc::channel(2048);

    (0..num_tasks).for_each(|_| {
        let tx = tx.clone();
        let client = client.clone();
        tokio::spawn(async move {
            let data = client
                .get("http://0.0.0.0:3000/")
                .send()
                .await
                .unwrap()
                .text()
                .await;
            tx.send(data).await.unwrap();
        });
    });

    let start = Instant::now();
    let mut outputs = Vec::new();
    let mut count = 0;
    for _ in 0..num_tasks {
        outputs.push(rx.recv().await);

        if count > 1010 {
            println!(
                " - request: {: >4}, elapsed: {:.4}",
                count,
                start.elapsed().as_secs_f32()
            );
        }
        count += 1;
    }
}