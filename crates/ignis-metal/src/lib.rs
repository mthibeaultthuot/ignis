use ignis_runtime::Runtime;
use objc2::rc::{Id, Retained};
use objc2::runtime::ProtocolObject;
use objc2_foundation::{NSError, NSString};
use objc2_metal::MTLCommandEncoder;
use objc2_metal::MTLComputePipelineState;
use objc2_metal::MTLFunction;
use objc2_metal::MTLSize;

use objc2_metal::{
    MTLBuffer, MTLCommandBuffer, MTLCommandQueue, MTLCompileOptions, MTLComputeCommandEncoder,
    MTLCreateSystemDefaultDevice, MTLDevice, MTLLibrary, MTLResourceOptions,
};
use std::collections::HashMap;
use std::ffi::c_void;
use std::hash::Hash;
use std::hash::Hasher;
use std::ptr::NonNull;

fn hash_key(src: &str, func: &str) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    let mut h = DefaultHasher::new();
    src.hash(&mut h);
    func.hash(&mut h);
    h.finish()
}

fn bytes_of<T: Copy>(x: T) -> Vec<u8> {
    let size = std::mem::size_of::<T>();
    let mut v = vec![0u8; size];
    unsafe {
        std::ptr::copy_nonoverlapping(&x as *const T as *const u8, v.as_mut_ptr(), size);
    }
    v
}

pub enum KernelArg {
    Buffer {
        buf: Retained<ProtocolObject<dyn MTLBuffer>>,
        offset: usize,
    },
    Bytes(Vec<u8>),
}

impl KernelArg {
    pub fn u32(v: u32) -> Self {
        KernelArg::Bytes(bytes_of(v))
    }
    pub fn f32(v: f32) -> Self {
        KernelArg::Bytes(bytes_of(v))
    }
}

pub struct MetalRuntime {
    device: Retained<ProtocolObject<dyn MTLDevice>>,
    queue: Retained<ProtocolObject<dyn MTLCommandQueue>>,
    pipelines: HashMap<u64, Retained<ProtocolObject<dyn MTLComputePipelineState>>>,
}

impl MetalRuntime {
    pub fn new() -> Self {
        let device = MTLCreateSystemDefaultDevice().expect("no Metal device");
        let queue = device.newCommandQueue().expect("no command queue");
        Self {
            device,
            queue,
            pipelines: HashMap::new(),
        }
    }

    pub fn buffer_from_slice<T: Copy>(
        &self,
        data: &[T],
    ) -> Retained<ProtocolObject<dyn MTLBuffer>> {
        let byte_len = data.len() * std::mem::size_of::<T>();
        let buf = self
            .device
            .newBufferWithLength_options(byte_len, MTLResourceOptions::StorageModeShared)
            .expect("buffer alloc");
        unsafe {
            let dst = buf.contents().as_ptr() as *mut T;
            std::ptr::copy_nonoverlapping(data.as_ptr(), dst, data.len());
        }
        buf
    }

    pub fn empty_buffer<T>(&self, len: usize) -> Retained<ProtocolObject<dyn MTLBuffer>> {
        let byte_len = len * std::mem::size_of::<T>();
        self.device
            .newBufferWithLength_options(byte_len, MTLResourceOptions::StorageModeShared)
            .expect("buffer alloc")
    }

    pub fn read_buffer<T: Copy>(&self, buf: &ProtocolObject<dyn MTLBuffer>, len: usize) -> Vec<T> {
        let mut out = vec![unsafe { std::mem::zeroed() }; len];
        unsafe {
            let src = buf.contents().as_ptr() as *const T;
            std::ptr::copy_nonoverlapping(src, out.as_mut_ptr(), len);
        }
        out
    }

    fn pipeline_for(
        &mut self,
        src: &str,
        func_name: &str,
    ) -> Retained<ProtocolObject<dyn MTLComputePipelineState>> {
        let key = hash_key(src, func_name);
        if let Some(p) = self.pipelines.get(&key) {
            return p.clone();
        }
        let ns_src = NSString::from_str(src);
        let opts = MTLCompileOptions::new();
        let lib: Retained<ProtocolObject<dyn MTLLibrary>> = self
            .device
            .newLibraryWithSource_options_error(&ns_src, Some(&opts))
            .expect("compile MSL failed");
        let func: Retained<ProtocolObject<dyn MTLFunction>> = lib
            .newFunctionWithName(&NSString::from_str(func_name))
            .expect("function not found");
        let pipe = self
            .device
            .newComputePipelineStateWithFunction_error(&func)
            .expect("pipeline creation failed");
        self.pipelines.insert(key, pipe.clone());
        pipe
    }

    fn bind_args(
        &self,
        enc: &ProtocolObject<dyn MTLComputeCommandEncoder>,
        args: &mut [KernelArg],
    ) {
        for (i, arg) in args.iter_mut().enumerate() {
            match arg {
                KernelArg::Buffer { buf, offset } => unsafe {
                    enc.setBuffer_offset_atIndex(Some(buf.as_ref()), *offset, i);
                },
                KernelArg::Bytes(bytes) => unsafe {
                    let ptr = NonNull::new(bytes.as_mut_ptr() as *mut c_void).unwrap();
                    enc.setBytes_length_atIndex(ptr, bytes.len(), i);
                },
            }
        }
    }

    pub fn launch(
        &mut self,
        src: &str,
        func_name: &str,
        args: &mut [KernelArg],
        grid: MTLSize,
        threads_per_tg: MTLSize,
    ) {
        let pipe = self.pipeline_for(src, func_name);

        let cmd = self.queue.commandBuffer().expect("cmd buffer");
        let enc = cmd.computeCommandEncoder().expect("compute encoder");
        enc.setComputePipelineState(&pipe);

        self.bind_args(&enc, args);
        enc.dispatchThreads_threadsPerThreadgroup(grid, threads_per_tg);

        enc.endEncoding();
        cmd.commit();
        unsafe {
            cmd.waitUntilCompleted();
        }
    }

    pub fn launch_1d(&mut self, src: &str, func_name: &str, args: &mut [KernelArg], n: usize) {
        let pipe = self.pipeline_for(src, func_name);
        let w = pipe.threadExecutionWidth();
        let threads_per_tg = MTLSize {
            width: w,
            height: 1,
            depth: 1,
        };
        let grid = MTLSize {
            width: n,
            height: 1,
            depth: 1,
        };

        let cmd = self.queue.commandBuffer().expect("cmd buffer");
        let enc = cmd.computeCommandEncoder().expect("compute encoder");
        enc.setComputePipelineState(&pipe);

        self.bind_args(&enc, args);
        enc.dispatchThreads_threadsPerThreadgroup(grid, threads_per_tg);

        enc.endEncoding();
        cmd.commit();
        unsafe {
            cmd.waitUntilCompleted();
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::MetalRuntime;

    #[test]
    fn matmul() {
        const MATMUL_MSL: &str = r#"
        #include <metal_stdlib>
        using namespace metal;

        kernel void matmul(device const float* A [[ buffer(0) ]],
                           device const float* B [[ buffer(1) ]],
                           device float*       C [[ buffer(2) ]],
                           constant uint& M     [[ buffer(3) ]],
                           constant uint& N     [[ buffer(4) ]],
                           constant uint& K     [[ buffer(5) ]],
                           uint2 gid [[thread_position_in_grid]]) {
            uint row = gid.y;
            uint col = gid.x;

            if (row < M && col < N) {
                float acc = 0.0;
                for (uint k = 0; k < K; k++) {
                    acc += A[row * K + k] * B[k * N + col];
                }
                C[row * N + col] = acc;
            }
        }
        "#;
        let m = 2;
        let k = 3;
        let n = 2;

        let a = vec![1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0];
        let b = vec![7.0f32, 8.0, 9.0, 10.0, 11.0, 12.0];

        let mut rt = MetalRuntime::new();

        let buf_a = rt.buffer_from_slice(&a);
        let buf_b = rt.buffer_from_slice(&b);
        let buf_c = rt.empty_buffer::<f32>(m * n);

        let mut args = [
            KernelArg::Buffer {
                buf: buf_a.clone(),
                offset: 0,
            },
            KernelArg::Buffer {
                buf: buf_b.clone(),
                offset: 0,
            },
            KernelArg::Buffer {
                buf: buf_c.clone(),
                offset: 0,
            },
            KernelArg::u32(m as u32),
            KernelArg::u32(n as u32),
            KernelArg::u32(k as u32),
        ];

        let grid = objc2_metal::MTLSize {
            width: n,
            height: m,
            depth: 1,
        };

        let threads_per_tg = objc2_metal::MTLSize {
            width: 8,
            height: 8,
            depth: 1,
        };

        rt.launch(MATMUL_MSL, "matmul", &mut args, grid, threads_per_tg);

        let c: Vec<f32> = rt.read_buffer(buf_c.as_ref(), m * n);

        println!("Result C: {:?}", c);
        assert_eq!(c, vec![58.0, 64.0, 139.0, 154.0]);
    }
}
