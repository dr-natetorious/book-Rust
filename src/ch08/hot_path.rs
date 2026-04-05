use crate::ch02::feed::{scan_messages, FeedError, MessageView};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProfileSample {
    pub parsed_messages: usize,
    pub add_messages: usize,
    pub modeled_allocations: usize,
}

fn profile_stream(stream: &[u8], allocate_each_message: bool) -> Result<ProfileSample, FeedError> {
    let mut parsed_messages = 0_usize;
    let mut add_messages = 0_usize;
    let mut modeled_allocations = 0_usize;
    let mut scratch = Vec::new();

    scan_messages(stream, |message| {
        parsed_messages += 1;
        if allocate_each_message {
            match message {
                MessageView::Add(add) => {
                    let cloned_payload = add.stock.to_vec();
                    add_messages += 1;
                    modeled_allocations += 1;
                    drop(cloned_payload);
                }
                MessageView::Execute(_) | MessageView::Cancel(_) => {}
            }
        } else if let MessageView::Add(add) = message {
            let before = scratch.capacity();
            scratch.clear();
            scratch.extend_from_slice(add.stock);
            if scratch.capacity() > before {
                modeled_allocations += 1;
            }
            add_messages += 1;
        }

        Ok(())
    })?;

    Ok(ProfileSample {
        parsed_messages,
        add_messages,
        modeled_allocations,
    })
}

// tag::baseline_profile[]
pub fn baseline_profile(stream: &[u8]) -> Result<ProfileSample, FeedError> {
    profile_stream(stream, true)
}
// end::baseline_profile[]

// tag::optimized_profile[]
pub fn optimized_profile(stream: &[u8]) -> Result<ProfileSample, FeedError> {
    profile_stream(stream, false)
}
// end::optimized_profile[]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// tag::workload_model[]
pub enum WorkloadModel {
    Threads,
    Async,
}
// end::workload_model[]

// tag::choose_model[]
pub fn choose_concurrency_model(consumers: usize, blocks_on_io: bool) -> WorkloadModel {
    if consumers <= 2 && !blocks_on_io {
        WorkloadModel::Threads
    } else {
        WorkloadModel::Async
    }
}
// end::choose_model[]
