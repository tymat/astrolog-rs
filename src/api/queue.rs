use std::collections::BinaryHeap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::sync::Semaphore;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueConfig {
    pub max_queue_size: usize,
    pub max_wait_time: Duration,
    pub priority_levels: usize,
}

impl Default for QueueConfig {
    fn default() -> Self {
        Self {
            max_queue_size: 10000,
            max_wait_time: Duration::from_secs(30),
            priority_levels: 3,
        }
    }
}

#[derive(Debug, Clone)]
pub struct QueuedRequest {
    pub priority: u8,
    pub timestamp: Instant,
    pub request_type: String,
}

impl PartialEq for QueuedRequest {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority && self.timestamp == other.timestamp
    }
}

impl Eq for QueuedRequest {}

impl PartialOrd for QueuedRequest {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for QueuedRequest {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Higher priority first, then FIFO
        match other.priority.cmp(&self.priority) {
            std::cmp::Ordering::Equal => self.timestamp.cmp(&other.timestamp),
            other => other,
        }
    }
}

pub struct RequestQueue {
    queue: Arc<Mutex<BinaryHeap<QueuedRequest>>>,
    semaphore: Arc<Semaphore>,
    config: QueueConfig,
}

impl RequestQueue {
    pub fn new(config: QueueConfig, max_concurrent: usize) -> Self {
        Self {
            queue: Arc::new(Mutex::new(BinaryHeap::new())),
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
            config,
        }
    }

    pub fn max_queue_size(&self) -> usize {
        self.config.max_queue_size
    }

    pub fn max_wait_time(&self) -> Duration {
        self.config.max_wait_time
    }

    pub fn priority_levels(&self) -> usize {
        self.config.priority_levels
    }

    pub async fn enqueue(&self, priority: u8, request_type: String) -> Result<(), String> {
        let mut queue = self.queue.lock().map_err(|_| "Failed to lock queue")?;
        
        if queue.len() >= self.config.max_queue_size {
            return Err("Queue is full".to_string());
        }

        queue.push(QueuedRequest {
            priority,
            timestamp: Instant::now(),
            request_type,
        });

        Ok(())
    }

    pub async fn acquire(&self) -> Result<(), String> {
        // Try to acquire the semaphore with a timeout
        match tokio::time::timeout(
            self.config.max_wait_time,
            self.semaphore.acquire()
        ).await {
            Ok(Ok(_)) => Ok(()),
            Ok(Err(_)) => Err("Failed to acquire semaphore".to_string()),
            Err(_) => Err("Timeout waiting for request processing".to_string()),
        }
    }

    pub fn release(&self) {
        self.semaphore.add_permits(1);
    }

    pub fn queue_size(&self) -> usize {
        self.queue.lock().map(|q| q.len()).unwrap_or(0)
    }

    pub fn is_full(&self) -> bool {
        self.queue_size() >= self.config.max_queue_size
    }
}

// Helper function to determine request priority
pub fn get_request_priority(request_type: &str) -> u8 {
    match request_type {
        "natal" => 2,    // Highest priority
        "transit" => 1,  // Medium priority
        "synastry" => 0, // Lowest priority
        _ => 1,          // Default to medium priority
    }
} 