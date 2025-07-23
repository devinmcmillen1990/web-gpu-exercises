pub fn list_gpus() {
    let instance = wgpu::Instance::default();
    
    println!("Found Adapters:");

    for adapter in instance.enumerate_adapters(wgpu::Backends::all()) {
        println!("  {:?}", adapter.get_info());
    }
}