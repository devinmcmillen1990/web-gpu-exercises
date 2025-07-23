# Exercise 1 - List GPUs
## Dependencies
- wgpu
## Objective
Provide list of available GPUs discoverable by the wgpu library.
## Sample Output
Found Adapters:
  AdapterInfo { name: "NVIDIA GeForce GTX 1650 with Max-Q Design", vendor: 4318, device: 8081, device_type: DiscreteGpu, driver: "NVIDIA", driver_info: "535.98", backend: Vulkan }

  AdapterInfo { name: "Intel(R) UHD Graphics 630", vendor: 32902, device: 16027, device_type: IntegratedGpu, driver: "Intel Corporation", driver_info: "Intel driver", backend: Vulkan }

  AdapterInfo { name: "Intel(R) UHD Graphics 630", vendor: 32902, device: 16027, device_type: IntegratedGpu, driver: "26.20.100.7584", driver_info: "", backend: Dx12 }

  AdapterInfo { name: "NVIDIA GeForce GTX 1650 with Max-Q Design", vendor: 4318, device: 8081, device_type: DiscreteGpu, driver: "31.0.15.3598", driver_info: "", backend: Dx12 }

  AdapterInfo { name: "Microsoft Basic Render Driver", vendor: 5140, device: 140, device_type: Cpu, driver: "10.0.26100.4484", driver_info: "", backend: Dx12 }
  
  AdapterInfo { name: "Intel(R) UHD Graphics 630", vendor: 32902, device: 0, device_type: IntegratedGpu, driver: "" driver_info: "4.6.0 - Build 26.20.100.7584", backend: Gl }