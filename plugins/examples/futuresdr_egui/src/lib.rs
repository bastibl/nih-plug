use futuresdr::runtime::scheduler::SmolScheduler;
use futuresdr::runtime::Flowgraph;
use futuresdr::runtime::FlowgraphHandle;
use futuresdr::runtime::Runtime;
use futuresdr::async_io::block_on;
use nih_plug::prelude::*;
use nih_plug_egui::{create_egui_editor, egui, widgets, EguiState};
use std::sync::Arc;

pub struct Echo<'a> {
    params: Arc<EchoParams>,
    runtime: Option<Runtime<'a, SmolScheduler>>,
    flowgraph_handle: Option<FlowgraphHandle>,
}

#[derive(Params)]
pub struct EchoParams {
    #[persist = "editor-state"]
    editor_state: Arc<EguiState>,

    #[id = "space"]
    pub space: IntParam,
}

impl<'a> Default for Echo<'a> {
    fn default() -> Self {
        Self {
            params: Arc::new(EchoParams::default()),
            runtime: None,
            flowgraph_handle: None,
        }
    }
}

impl Default for EchoParams {
    fn default() -> Self {
        Self {
            editor_state: EguiState::from_size(300, 180),
            space: IntParam::new("Space", 0, IntRange::Linear { min: 0, max: 30 }),
        }
    }
}

impl Plugin for Echo<'static> {
    const NAME: &'static str = "FutureSDR Echo (egui)";
    const VENDOR: &'static str = "Bastian Bloessl";
    const URL: &'static str = "https://www.bastibl.net/";
    const EMAIL: &'static str = "mail@bastibl.net";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(1),
            main_output_channels: NonZeroU32::new(1),
            ..AudioIOLayout::const_default()
        },
    ];

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        let params = self.params.clone();
        create_egui_editor(
            self.params.editor_state.clone(),
            (),
            |_, _| {},
            move |egui_ctx, setter, _state| {
                egui::CentralPanel::default().show(egui_ctx, |ui| {
                    ui.label("Delay (in Samples)");
                    ui.add(widgets::ParamSlider::for_param(&params.space, setter));
                });
            },
        )
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        let rt = Runtime::new();
        let fg = Flowgraph::new();
        let (_task, handle) = block_on(rt.start(fg));
        self.runtime = Some(rt);
        self.flowgraph_handle = Some(handle);

        true
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        for channel_samples in buffer.iter_samples() {
            let gain = 0.5;
            for sample in channel_samples {
                *sample *= gain;
            }

            // if self.params.editor_state.is_open() {
            // }
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for Echo<'static> {
    const CLAP_ID: &'static str = "net.bastibl.futuresdr-echo";
    const CLAP_DESCRIPTION: Option<&'static str> =
        Some("A simple FutureSDR echo plugin with an egui GUI");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::AudioEffect,
        ClapFeature::Stereo,
        ClapFeature::Mono,
        ClapFeature::Utility,
    ];
}

impl Vst3Plugin for Echo<'static> {
    const VST3_CLASS_ID: [u8; 16] = *b"FutureSDREcho\0\0\0";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Tools];
}

nih_export_clap!(Echo<'static>);
nih_export_vst3!(Echo<'static>);
