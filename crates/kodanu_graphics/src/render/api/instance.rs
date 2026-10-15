use crate::{InstanceDescriptor, InstanceError};

use wgpu::{
    Adapter as WgpuAdapter, ColorTargetState as WgpuColorTargetState,
    CommandEncoder as WgpuCommandEncoder, CommandEncoderDescriptor as WgpuCommandEncoderDescriptor,
    CurrentSurfaceTexture as WgpuCurrentSurfaceTexture, Device as WgpuDevice,
    DeviceDescriptor as WgpuDeviceDescriptor, ExperimentalFeatures as WgpuExperimentalFeatures,
    Features as WgpuFeatures, Instance as WgpuInstance,
    InstanceDescriptor as WgpuInstanceDescriptor, Limits as WgpuLimits,
    MemoryHints as WgpuMemoryHints, PowerPreference as WgpuPowerReference,
    PresentMode as WgpuPresentMode, Queue as WgpuQueue,
    RequestAdapterOptions as WgpuRequestAdapterOptions, Surface as WgpuSurface,
    SurfaceColorSpace as WgpuSurfaceColorSpace, SurfaceConfiguration as WgpuSurfaceConfiguration,
    SurfaceTargetUnsafe as WgpuSurfaceTargetUnsafe, SurfaceTexture as WgpuSurfaceTexture,
    TextureFormat as WgpuTextureFormat, TextureUsages as WgpuTextureUsages, Trace as WgpuTrace,
};

use {
    kodanu_ecs::{EventReader, ResMut, Resource},
    kodanu_math::SurfaceSize,
    std::iter::once,
};

#[derive(Resource)]
pub struct Instance {
    surface: WgpuSurface<'static>,
    config: WgpuSurfaceConfiguration,
    _instance: WgpuInstance,
    _adapter: WgpuAdapter,
    device: WgpuDevice,
    queue: WgpuQueue,
}

impl Instance {
    pub async fn new(descriptor: InstanceDescriptor) -> Self {
        let size = descriptor.size();
        let target = descriptor.surface_target();

        let instance = WgpuInstance::new(WgpuInstanceDescriptor::from(descriptor));

        let surface = unsafe {
            instance
                .create_surface_unsafe(WgpuSurfaceTargetUnsafe::RawHandle {
                    raw_display_handle: Some(target.raw_display()),
                    raw_window_handle: target.raw_window(),
                })
                .unwrap_or_else(|_| panic!("{}", InstanceError::SurfaceCreate))
        };

        let adapter = instance
            .request_adapter(&WgpuRequestAdapterOptions {
                power_preference: WgpuPowerReference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
                apply_limit_buckets: false,
            })
            .await
            .unwrap_or_else(|_| panic!("{}", InstanceError::AdapterRequest));

        let (device, queue) = adapter
            .request_device(&WgpuDeviceDescriptor {
                label: Some("Device"),
                memory_hints: WgpuMemoryHints::Performance,
                required_features: WgpuFeatures::empty(),
                experimental_features: WgpuExperimentalFeatures::disabled(),
                required_limits: WgpuLimits::defaults(),
                trace: WgpuTrace::Off,
            })
            .await
            .unwrap_or_else(|_| panic!("{}", InstanceError::DeviceCreate));

        let capabilities = surface.get_capabilities(&adapter);

        let format = capabilities
            .formats
            .iter()
            .copied()
            .find(WgpuTextureFormat::is_srgb)
            .unwrap_or(capabilities.formats[0]);

        let config = WgpuSurfaceConfiguration {
            usage: WgpuTextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width(),
            height: size.height(),
            present_mode: WgpuPresentMode::Fifo,
            alpha_mode: capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
            color_space: WgpuSurfaceColorSpace::Srgb,
        };

        surface.configure(&device, &config);

        Self {
            config,
            _instance: instance,
            surface,
            _adapter: adapter,
            device,
            queue,
        }
    }
}

impl Instance {
    pub(crate) fn instance_update(event: EventReader<SurfaceSize>, mut instance: ResMut<Instance>) {
        for event in event.iter() {
            instance.resize(*event);
        }
    }
}

impl Instance {
    pub const fn queue(&self) -> &WgpuQueue {
        &self.queue
    }

    pub const fn device(&self) -> &WgpuDevice {
        &self.device
    }

    pub fn format(&self) -> WgpuTextureFormat {
        WgpuColorTargetState::from(self.config.format).format
    }

    pub fn size(&self) -> SurfaceSize {
        SurfaceSize::clamped(self.config.width, self.config.height)
    }

    pub(crate) fn present(&self, encoder: WgpuCommandEncoder, frame: WgpuSurfaceTexture) {
        self.queue.submit(once(encoder.finish()));
        self.queue.present(frame);
    }

    pub(crate) fn create_encoder(&self) -> WgpuCommandEncoder {
        self.device
            .create_command_encoder(&WgpuCommandEncoderDescriptor::default())
    }

    pub(crate) fn acquire_frame(&self) -> WgpuCurrentSurfaceTexture {
        self.surface.get_current_texture()
    }

    pub(crate) fn resize(&mut self, size: SurfaceSize) {
        self.config.width = size.width();
        self.config.height = size.height();

        self.surface.configure(&self.device, &self.config);
    }
}
