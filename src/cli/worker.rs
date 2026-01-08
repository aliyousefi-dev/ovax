use crate::ovagrpc::ova_service_client::OvaServiceClient;
use crate::ovagrpc::HeartbeatRequest;
use tonic::Request;
use std::time::Duration;
use tokio::time;

/// Dedicated function to handle background heartbeat pulses
async fn start_heartbeat(server_addr: String, worker_id: String) {
    // Establish a dedicated connection for the heartbeat loop
    if let Ok(mut client) = OvaServiceClient::connect(server_addr).await {
        let mut interval = time::interval(Duration::from_secs(5));
        
        loop {
            // Wait for the next 5-second tick
            interval.tick().await;

            let hb_req = Request::new(HeartbeatRequest {
                worker_id: worker_id.clone(),
                cpu_usage: 12.0, // Placeholder: replace with actual sysinfo metrics later
                ram_usage: 25.0,
                active_jobs: 0,
            });

            // Send the health status to the Go server
            match client.heartbeat(hb_req).await {
                Ok(_) => println!("[Status] Heartbeat sent: Online"),
                Err(e) => eprintln!("[Status] Heartbeat Error: {}", e),
            }
        }
    }
}

pub async fn execute_connect(server_addr: String) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Verify initial connection to the OVA server
    let _client = OvaServiceClient::connect(server_addr.clone()).await?;
    println!("Successfully connected to OVA Server!");

    // 2. Spawn the heartbeat loop in a background thread (Tokio task)
    // This allows the heartbeat to run independently of other processing tasks
    let worker_id = "ovax-rust-01".to_string();
    tokio::spawn(start_heartbeat(server_addr, worker_id));

    println!("Worker is now running. Press Ctrl+C to stop.");

    // 3. Keep the main function alive
    // Without this, the program would exit and kill the background heartbeat task
    tokio::signal::ctrl_c().await?;
    
    println!("\nShutdown signal received. Closing worker...");
    Ok(())
}