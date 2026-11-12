use std::{cell::RefCell, error::Error as StdError, io, rc::Rc};

use proc_macro2::TokenStream;
use quote::ToTokens;

use xsd_parser::{
    config::Schema,
    exec_generator, exec_interpreter, exec_optimizer, exec_parser, exec_render, generate,
    models::{code::Module, TypeIdent},
    pipeline::renderer::{Context, MetaData, RenderStep, RenderStepPhase, RenderStepType},
    Config, Error, IdentType, Renderer, RendererError,
};

#[derive(Clone, Debug, Eq, PartialEq)]
struct Event {
    step: &'static str,
    phase: RenderStepPhase,
    ident: Option<TypeIdent>,
}

type Events = Rc<RefCell<Vec<Event>>>;

#[derive(Clone, Debug)]
struct ObservedStep {
    name: &'static str,
    fail: Option<RenderStepPhase>,
    events: Events,
}

impl ObservedStep {
    fn record(
        &self,
        phase: RenderStepPhase,
        ident: Option<&TypeIdent>,
    ) -> Result<(), RendererError> {
        self.events.borrow_mut().push(Event {
            step: self.name,
            phase,
            ident: ident.cloned(),
        });
        if self.fail == Some(phase) {
            return Err(RendererError::custom(io::Error::other("callback failed")));
        }
        Ok(())
    }
}

impl RenderStep for ObservedStep {
    fn render_step_type(&self) -> RenderStepType {
        RenderStepType::Undefined
    }

    fn render_step_name(&self) -> &'static str {
        self.name
    }

    fn initialize(&mut self, _: &mut MetaData<'_>) -> Result<(), RendererError> {
        self.record(RenderStepPhase::Initialize, None)
    }

    fn render_type(&mut self, ctx: &mut Context<'_, '_>) -> Result<(), RendererError> {
        // Even after a callback has modified the output, an error must return
        // no module and stop all later callbacks.
        ctx.current_module().append(quote::quote!(
            const _: () = ();
        ));
        self.record(RenderStepPhase::RenderType, Some(ctx.ident))
    }

    fn finish(&mut self, _: &MetaData<'_>, module: &mut Module) -> Result<(), RendererError> {
        module.append(quote::quote!(
            const _: () = ();
        ));
        self.record(RenderStepPhase::Finish, None)
    }
}

fn config() -> Config {
    Config::default()
        .with_schema(Schema::File(
            "tests/feature/fallible_render_steps/schema.xsd".into(),
        ))
        .with_generate([
            (IdentType::Type, "tns:AlphaType"),
            (IdentType::Type, "tns:BetaType"),
        ])
}

fn observed_config(events: &Events, fail: Option<RenderStepPhase>) -> Config {
    let mut config = config();
    // The builder replaces repeated step types. Register these instances
    // explicitly so the test can observe callbacks before and after a failure.
    for name in ["before", "failing", "after"] {
        config.renderer.steps.push(Box::new(ObservedStep {
            name,
            fail: if name == "failing" { fail } else { None },
            events: Rc::clone(events),
        }));
    }
    config
}

#[derive(Clone, Copy)]
enum EntryPoint {
    Renderer,
    ExecRender,
    Generate,
}

fn render(config: Config, entry: EntryPoint) -> Result<TokenStream, Error> {
    if matches!(entry, EntryPoint::Generate) {
        return generate(config);
    }
    let schemas = exec_parser(config.parser)?;
    let meta = exec_interpreter(config.interpreter, &schemas)?;
    let meta = exec_optimizer(config.optimizer, meta)?;
    let data = exec_generator(config.generator, &schemas, &meta)?;
    if matches!(entry, EntryPoint::ExecRender) {
        return Ok(exec_render(config.renderer, &data)?.into_token_stream());
    }
    let mut renderer = Renderer::new(&data);
    for step in config.renderer.steps {
        renderer = renderer.with_step_boxed(step.into_render_step());
    }
    Ok(renderer.finish()?.into_token_stream())
}

fn assert_callback_failures(entry: EntryPoint) {
    let baseline = Events::default();
    render(observed_config(&baseline, None), entry).unwrap();
    let baseline = baseline.borrow();
    assert!(
        baseline
            .iter()
            .filter(|event| event.step == "failing" && event.phase == RenderStepPhase::RenderType)
            .count()
            > 1,
        "the fixture must exercise more than one type"
    );

    for phase in [
        RenderStepPhase::Initialize,
        RenderStepPhase::RenderType,
        RenderStepPhase::Finish,
    ] {
        let events = Events::default();
        let error = render(observed_config(&events, Some(phase)), entry).unwrap_err();
        let Error::RendererError(error) = error else {
            panic!("unexpected pipeline error: {error:?}");
        };
        let stop = baseline
            .iter()
            .position(|event| event.step == "failing" && event.phase == phase)
            .unwrap();
        assert_eq!(&*events.borrow(), &baseline[..=stop]);

        let RendererError::RenderStep {
            step,
            phase: actual_phase,
            ident,
            source,
        } = &error
        else {
            panic!("missing callback context: {error:?}");
        };
        assert_eq!(*step, "failing");
        assert_eq!(*actual_phase, phase);
        assert_eq!(ident, &baseline[stop].ident);
        assert!(error.to_string().contains(&phase.to_string()));
        assert!(!error.to_string().contains("callback failed"));
        if let Some(ident) = ident {
            assert!(error.to_string().contains(&ident.to_string()));
        }

        // Match the typed renderer error directly; its source retains the
        // original application error and supports downcasting.
        let context_source = StdError::source(&error).unwrap();
        assert!(matches!(source.as_ref(), RendererError::Custom(_)));
        let original = context_source
            .source()
            .unwrap()
            .downcast_ref::<io::Error>()
            .unwrap();
        assert_eq!(original.kind(), io::ErrorKind::Other);
        assert_eq!(original.to_string(), "callback failed");
    }
}

#[test]
fn renderer_stops_at_the_first_callback_error() {
    assert_callback_failures(EntryPoint::Renderer);
}

#[test]
fn exec_render_propagates_callback_errors() {
    assert_callback_failures(EntryPoint::ExecRender);
}

#[test]
fn generate_propagates_callback_errors() {
    assert_callback_failures(EntryPoint::Generate);
}

#[test]
fn successful_callbacks_keep_their_order() {
    let events = Events::default();
    generate(observed_config(&events, None)).unwrap();
    let events = events.borrow();
    let (initialize, rest) = events.split_at(3);
    let (render, finish) = rest.split_at(rest.len() - 3);
    for (events, phase) in [
        (initialize, RenderStepPhase::Initialize),
        (finish, RenderStepPhase::Finish),
    ] {
        for (event, name) in events.iter().zip(["before", "failing", "after"]) {
            assert_eq!(event.step, name);
            assert_eq!(event.phase, phase);
            assert!(event.ident.is_none());
        }
    }
    let chunks = render.chunks_exact(3);
    assert!(chunks.remainder().is_empty());
    for events in chunks {
        for (event, name) in events.iter().zip(["before", "failing", "after"]) {
            assert_eq!(event.step, name);
            assert_eq!(event.phase, RenderStepPhase::RenderType);
            assert!(event.ident.is_some());
            assert_eq!(event.ident, events[0].ident);
        }
    }
}

#[derive(Clone, Debug)]
struct NoopStep;

impl RenderStep for NoopStep {
    fn render_step_type(&self) -> RenderStepType {
        RenderStepType::Undefined
    }
}

#[test]
fn default_callbacks_preserve_generated_code() {
    assert_eq!(
        NoopStep.render_step_name(),
        std::any::type_name::<NoopStep>()
    );
    let expected = generate(config()).unwrap().to_string();
    for entry in [
        EntryPoint::Renderer,
        EntryPoint::ExecRender,
        EntryPoint::Generate,
    ] {
        let module = render(config().with_render_step(NoopStep), entry).unwrap();
        assert_eq!(module.to_string(), expected);
    }
}

#[test]
fn custom_errors_accept_boxed_error_reports() {
    let report = anyhow::anyhow!("external error report");
    let error = RendererError::custom(report);
    assert_eq!(error.to_string(), "Custom renderer error");
    assert_eq!(error.source().unwrap().to_string(), "external error report");
    let report = anyhow::Error::new(error);
    assert_eq!(
        format!("{report:#}"),
        "Custom renderer error: external error report"
    );
}

#[test]
fn formatted_callback_errors_report_the_cause_once() {
    let events = Events::default();
    let error = render(
        observed_config(&events, Some(RenderStepPhase::Initialize)),
        EntryPoint::Generate,
    )
    .unwrap_err();
    let report = anyhow::Error::new(error);
    for message in [format!("{report:#}"), format!("{report:?}")] {
        assert_eq!(message.matches("callback failed").count(), 1, "{message}");
    }
}
