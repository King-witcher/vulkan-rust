use std::sync::Arc;

use smallvec::SmallVec;
use vulkano::format::Format;
use vulkano::image::Image;
use vulkano::swapchain::{
    ColorSpace, PresentMode, SurfaceCapabilities, SurfaceTransform, Swapchain, SwapchainCreateInfo,
};
use vulkano::{
    image::{
        ImageAspects, ImageSubresourceRange,
        sampler::{ComponentMapping, ComponentSwizzle},
        view::{ImageView, ImageViewCreateInfo, ImageViewType},
    },
    swapchain::SwapchainCreateFlags,
    sync::Sharing,
};

use crate::vw_engine::vw_device::VwDevice;

pub struct VwSwapchain {
    swapchain: Arc<Swapchain>,
    images: Vec<Arc<Image>>,
    surface_format: Format,
    extent: [u32; 2],
    image_views: Vec<Arc<ImageView>>,
}

impl VwSwapchain {
    pub fn new(device: &VwDevice) -> anyhow::Result<Self> {
        let swapchain_support = device.swap_chain_support()?;
        let (surface_format, color_space) =
            choose_surface_format(&swapchain_support.surface_formats);
        let present_mode = choose_present_mode(&swapchain_support.present_modes);
        let extent = choose_extent(&swapchain_support.surface_capabilities, 1920, 1080);

        let image_count = 3.clamp(
            swapchain_support.surface_capabilities.min_image_count,
            swapchain_support
                .surface_capabilities
                .max_image_count
                .unwrap_or(u32::MAX),
        );

        let mut create_info: SwapchainCreateInfo = SwapchainCreateInfo {
            flags: SwapchainCreateFlags::empty(),
            min_image_count: swapchain_support.surface_capabilities.min_image_count,
            image_format: surface_format,
            image_color_space: color_space,
            image_extent: extent,
            image_array_layers: 1,
            image_usage: vulkano::image::ImageUsage::COLOR_ATTACHMENT,
            image_sharing: Sharing::Exclusive,
            pre_transform: SurfaceTransform::Identity, // No transformation
            // pre_transform: swapchain_support.surface_capabilities.current_transform, // No transformation
            composite_alpha: vulkano::swapchain::CompositeAlpha::Opaque, // Window is opaque
            present_mode,
            clipped: true, // We don't care about pixels that are obscured by other window or outside the screen
            ..Default::default()
        };

        // If the queues are different, we need to set the sharing mode to concurrent
        if device.graphics_queue().queue_index() != device.present_queue().queue_index() {
            create_info.image_sharing = Sharing::Concurrent(SmallVec::from_slice(&[
                device.graphics_queue().queue_index(),
                device.present_queue().queue_index(),
            ]));
        }

        let (swapchain, images) = vulkano::swapchain::Swapchain::new(
            device.logical_device(),
            device.surface(),
            create_info,
        )?;

        let image_views = create_image_views(surface_format, images.clone());

        Ok(VwSwapchain {
            swapchain,
            images,
            surface_format,
            image_views,
            extent,
        })
    }
}

fn choose_surface_format(available_formats: &[(Format, ColorSpace)]) -> (Format, ColorSpace) {
    for (format, color_space) in available_formats.iter() {
        if *format == Format::B8G8R8A8_SRGB && *color_space == ColorSpace::SrgbNonLinear {
            return (*format, *color_space);
        }
    }
    available_formats[0]
}

fn choose_present_mode(present_modes: &[PresentMode]) -> PresentMode {
    for present_mode in present_modes.iter() {
        if *present_mode == PresentMode::Mailbox {
            return *present_mode;
        }
    }
    PresentMode::Fifo
}

fn choose_extent(capabilities: &SurfaceCapabilities, width: u32, height: u32) -> [u32; 2] {
    if let Some([width, height]) = capabilities.current_extent {
        [width, height]
    } else {
        let [min_width, min_height] = capabilities.min_image_extent;
        let [max_width, max_height] = capabilities.max_image_extent;

        let actual_extent = [
            width.clamp(min_width, max_width),
            height.clamp(min_height, max_height),
        ];

        actual_extent
    }
}

fn create_image_views(surface_format: Format, images: Vec<Arc<Image>>) -> Vec<Arc<ImageView>> {
    images
        .iter()
        .map(|image| {
            let create_info = ImageViewCreateInfo {
                format: surface_format,
                view_type: ImageViewType::Dim2d,
                component_mapping: ComponentMapping {
                    r: ComponentSwizzle::Identity,
                    g: ComponentSwizzle::Identity,
                    b: ComponentSwizzle::Identity,
                    a: ComponentSwizzle::Identity,
                },
                subresource_range: ImageSubresourceRange {
                    aspects: ImageAspects::COLOR,
                    array_layers: 0..1, // ??
                    mip_levels: 0..1,
                },
                ..Default::default()
            };
            ImageView::new(image.clone(), create_info).expect("Failed to create image view")
        })
        .collect::<Vec<_>>()
}
