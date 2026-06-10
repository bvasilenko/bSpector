use crate::{
    ArtefactShape, BspectorError, RiskScoreBand, ScanState, Strictness, VulnerabilityPattern,
    error::process_exit_code, substrate_input::SubstrateInput,
};
use std::{fmt, path::Path};

const BINARY_NAME: &str = "bspector";
const PLACEHOLDER_PATTERN: VulnerabilityPattern = VulnerabilityPattern::PromptInjection;
const PLACEHOLDER_SCAN_STATE: ScanState = ScanState::Unsafe;
const PLACEHOLDER_ARTEFACT_SHAPE: ArtefactShape = ArtefactShape::SkillMd;
const PLACEHOLDER_STRICTNESS: Strictness = Strictness::Default;

pub fn run(args: crate::cli::ScanArgs) -> Result<std::process::ExitCode, BspectorError> {
    validate_reason(args.reason.as_deref())?;

    let input = SubstrateInput::new(
        args.artefact,
        args.artefact_shape,
        args.strictness,
        args.llm_stage,
        args.osv_online,
        args.manifest,
        args.reason,
    )?;

    println!("{}", PlaceholderDirective::new(input));

    Ok(process_exit_code(PLACEHOLDER_SCAN_STATE.into()))
}

fn validate_reason(reason: Option<&str>) -> Result<(), BspectorError> {
    if matches!(reason, Some(r) if r.trim().is_empty()) {
        return Err(BspectorError::Usage("reason must not be empty".to_owned()));
    }
    Ok(())
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct PlaceholderDirective {
    input: SubstrateInput,
    pattern: VulnerabilityPattern,
    scan_state: ScanState,
}

impl PlaceholderDirective {
    fn new(input: SubstrateInput) -> Self {
        Self {
            input,
            pattern: PLACEHOLDER_PATTERN,
            scan_state: PLACEHOLDER_SCAN_STATE,
        }
    }

    fn resolved_artefact_shape(&self) -> ArtefactShape {
        self.input
            .artefact_shape
            .as_deref()
            .and_then(|s| s.parse().ok())
            .unwrap_or(PLACEHOLDER_ARTEFACT_SHAPE)
    }

    fn resolved_strictness(&self) -> Strictness {
        self.input
            .strictness
            .as_deref()
            .and_then(|s| s.parse().ok())
            .unwrap_or(PLACEHOLDER_STRICTNESS)
    }
}

impl fmt::Display for PlaceholderDirective {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            formatter,
            "[{BINARY_NAME} placeholder directive - pre-corpus output]"
        )?;
        writeln!(
            formatter,
            "Parsed input: artefact={}, artefact-shape={}, strictness={}, llm-stage={}, osv-online={}, manifest={}, reason={}. Routing key: VulnerabilityPattern::{} placeholder route. Invocation surface: cli. Scan-state: {}.",
            self.input.artefact,
            self.resolved_artefact_shape(),
            self.resolved_strictness(),
            self.input.llm_stage,
            self.input.osv_online,
            format_optional_path(self.input.manifest.as_deref()),
            format_optional_reason(self.input.reason.as_deref()),
            format_variant_name(self.pattern),
            format_variant_name(self.scan_state),
        )?;
        writeln!(
            formatter,
            "ACTION: This invocation reached {BINARY_NAME} at the pre-corpus phase. A real evolved directive would name the specific vulnerability pattern {} matched in the artefact manifest and steer the calling LLM toward either refusing the install, downgrading strictness with explicit operator opt-in, or extending the scan ruleset.",
            self.pattern,
        )?;
        write!(
            formatter,
            "Re-invoke after the corpus-backed release lands. Do not treat this placeholder as ground truth. Exit code carries the verdict-class signal."
        )
    }
}

fn format_optional_path(path: Option<&Path>) -> String {
    path.map(|p| p.display().to_string())
        .unwrap_or_else(|| "<none>".to_owned())
}

fn format_optional_reason(reason: Option<&str>) -> String {
    reason
        .map(|r| r.to_owned())
        .unwrap_or_else(|| "<none>".to_owned())
}

fn format_variant_name<T: fmt::Debug>(value: T) -> String {
    format!("{value:?}")
}

pub fn risk_threshold() -> RiskScoreBand {
    RiskScoreBand::DEFAULT_THRESHOLD
}
