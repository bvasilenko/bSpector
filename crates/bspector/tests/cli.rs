use assert_cmd::Command;
use bspector::{ArtefactShape, Strictness, VulnerabilityPattern};
use predicates::prelude::*;

const PLACEHOLDER_DIRECTIVE_HEADER: &str = "[bspector placeholder directive - pre-corpus output]";
const ACTION_PREFIX: &str = "ACTION: This invocation reached bspector";
const EXIT_CODE_FOOTER: &str = "Exit code carries the verdict-class signal.";
const PUBLIC_INVOCATION_SURFACE_LINE: &str = "Invocation surface: cli.";
const SCAN_FINDING_EXIT_CODE: i32 = 1;

fn bspector() -> Command {
    Command::cargo_bin("bspector").expect("binary exists")
}

fn stdout_with_code(args: &[&str], code: i32) -> String {
    let output = bspector()
        .args(args)
        .assert()
        .code(code)
        .get_output()
        .clone();
    String::from_utf8(output.stdout).expect("stdout is utf8")
}

fn scan_stdout(args: &[&str]) -> String {
    stdout_with_code(args, SCAN_FINDING_EXIT_CODE)
}

fn success_stdout(args: &[&str]) -> String {
    stdout_with_code(args, 0)
}

fn assert_scan_directive(stdout: &str) {
    assert!(
        stdout.contains(PLACEHOLDER_DIRECTIVE_HEADER),
        "missing header"
    );
    assert!(
        stdout.contains("Parsed input: artefact="),
        "missing artefact field"
    );
    assert!(
        stdout.contains("artefact-shape="),
        "missing artefact-shape field"
    );
    assert!(stdout.contains("strictness="), "missing strictness field");
    assert!(stdout.contains("llm-stage="), "missing llm-stage field");
    assert!(stdout.contains("osv-online="), "missing osv-online field");
    assert!(stdout.contains("manifest="), "missing manifest field");
    assert!(stdout.contains("reason="), "missing reason field");
    assert!(
        stdout.contains("Routing key: VulnerabilityPattern::"),
        "missing routing key"
    );
    assert!(
        stdout.contains(" placeholder route."),
        "missing placeholder route"
    );
    assert!(
        stdout.contains(PUBLIC_INVOCATION_SURFACE_LINE),
        "missing invocation surface"
    );
    assert!(stdout.contains("Scan-state: Unsafe."), "missing scan-state");
    assert!(stdout.contains(ACTION_PREFIX), "missing action prefix");
    assert!(
        stdout.contains("vulnerability pattern "),
        "missing vulnerability pattern text"
    );
    assert!(
        stdout.contains("refusing the install"),
        "missing directive action text"
    );
    assert!(
        stdout.contains(EXIT_CODE_FOOTER),
        "missing exit code footer"
    );
}

#[test]
fn help_exits_successfully_and_mentions_tool_purpose() {
    bspector()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Prompt lookup tool"));
}

#[test]
fn vuln_patterns_exits_successfully_and_prints_all_12_patterns() {
    let stdout = success_stdout(&["vuln-patterns"]);
    let actual: Vec<&str> = stdout.lines().collect();
    let expected: Vec<String> = VulnerabilityPattern::ALL
        .iter()
        .map(ToString::to_string)
        .collect();

    assert_eq!(expected, actual);
}

#[test]
fn scan_emits_placeholder_directive_and_finding_exit_code() {
    let stdout = scan_stdout(&["scan", "./README.md", "--artefact-shape", "skill-md"]);
    assert_scan_directive(&stdout);
}

#[test]
fn scan_directive_echoes_artefact_path() {
    let stdout = scan_stdout(&["scan", "./README.md"]);
    assert!(
        stdout.contains("artefact=./README.md"),
        "artefact not echoed: {stdout}"
    );
}

#[test]
fn scan_directive_echoes_artefact_shape_when_provided() {
    let stdout = scan_stdout(&["scan", "./skill.md", "--artefact-shape", "skill-md"]);
    assert!(
        stdout.contains("artefact-shape=skill-md"),
        "artefact-shape not echoed: {stdout}"
    );
}

#[test]
fn scan_directive_echoes_strictness_when_provided() {
    let stdout = scan_stdout(&["scan", "./skill.md", "--strictness", "strict"]);
    assert!(
        stdout.contains("strictness=strict"),
        "strictness not echoed: {stdout}"
    );
}

#[test]
fn scan_directive_echoes_llm_stage_flag() {
    let stdout_with_flag = scan_stdout(&["scan", "./skill.md", "--llm-stage"]);
    assert!(
        stdout_with_flag.contains("llm-stage=true"),
        "llm-stage=true not echoed"
    );

    let stdout_without_flag = scan_stdout(&["scan", "./skill.md"]);
    assert!(
        stdout_without_flag.contains("llm-stage=false"),
        "llm-stage=false not echoed"
    );
}

#[test]
fn scan_directive_echoes_osv_online_flag() {
    let stdout_with_flag = scan_stdout(&["scan", "./skill.md", "--osv-online"]);
    assert!(
        stdout_with_flag.contains("osv-online=true"),
        "osv-online=true not echoed"
    );

    let stdout_without_flag = scan_stdout(&["scan", "./skill.md"]);
    assert!(
        stdout_without_flag.contains("osv-online=false"),
        "osv-online=false not echoed"
    );
}

#[test]
fn scan_directive_echoes_manifest_when_provided() {
    let stdout = scan_stdout(&["scan", "./skill.md", "--manifest", "override.toml"]);
    assert!(
        stdout.contains("manifest=override.toml"),
        "manifest not echoed: {stdout}"
    );
}

#[test]
fn scan_directive_echoes_none_for_absent_manifest() {
    let stdout = scan_stdout(&["scan", "./skill.md"]);
    assert!(
        stdout.contains("manifest=<none>"),
        "absent manifest not shown: {stdout}"
    );
}

#[test]
fn scan_directive_echoes_reason_when_provided() {
    let stdout = scan_stdout(&["scan", "./skill.md", "--reason", "review requested"]);
    assert!(
        stdout.contains("reason=review requested"),
        "reason not echoed: {stdout}"
    );
}

#[test]
fn scan_directive_echoes_none_for_absent_reason() {
    let stdout = scan_stdout(&["scan", "./skill.md"]);
    assert!(
        stdout.contains("reason=<none>"),
        "absent reason not shown: {stdout}"
    );
}

#[test]
fn scan_exit_code_contract_is_finding_for_placeholder_state() {
    bspector()
        .args(["scan", "./README.md"])
        .assert()
        .code(SCAN_FINDING_EXIT_CODE);
}

#[test]
fn scan_accepts_all_valid_artefact_shape_values() {
    for shape in ArtefactShape::ALL {
        let stdout = scan_stdout(&[
            "scan",
            "./skill.md",
            "--artefact-shape",
            shape.stable_name(),
        ]);
        assert!(
            stdout.contains(&format!("artefact-shape={shape}")),
            "artefact-shape {shape} not echoed in directive"
        );
    }
}

#[test]
fn scan_accepts_all_valid_strictness_values() {
    for level in Strictness::ALL {
        let stdout = scan_stdout(&["scan", "./skill.md", "--strictness", level.stable_name()]);
        assert!(
            stdout.contains(&format!("strictness={level}")),
            "strictness {level} not echoed in directive"
        );
    }
}

#[test]
fn scan_directive_shows_default_artefact_shape_when_not_specified() {
    let stdout = scan_stdout(&["scan", "./skill.md"]);
    assert!(
        stdout.contains("artefact-shape=skill-md"),
        "default artefact-shape not shown: {stdout}"
    );
}

#[test]
fn scan_directive_shows_default_strictness_when_not_specified() {
    let stdout = scan_stdout(&["scan", "./skill.md"]);
    assert!(
        stdout.contains("strictness=default"),
        "default strictness not shown: {stdout}"
    );
}

#[test]
fn scan_with_unknown_artefact_shape_falls_back_to_placeholder_shape() {
    let stdout = scan_stdout(&[
        "scan",
        "./skill.md",
        "--artefact-shape",
        "completely-unknown",
    ]);
    assert!(
        stdout.contains("artefact-shape=skill-md"),
        "unknown artefact-shape must fall back to placeholder: {stdout}"
    );
}

#[test]
fn scan_with_unknown_strictness_falls_back_to_placeholder_strictness() {
    let stdout = scan_stdout(&["scan", "./skill.md", "--strictness", "completely-unknown"]);
    assert!(
        stdout.contains("strictness=default"),
        "unknown strictness must fall back to placeholder: {stdout}"
    );
}

#[test]
fn scan_rejects_blank_reason() {
    for blank in ["", " ", "   ", "\t"] {
        bspector()
            .args(["scan", "./skill.md", "--reason", blank])
            .assert()
            .code(64)
            .stderr(predicate::str::contains("reason must not be empty"));
    }
}

#[test]
fn scan_accepts_every_non_blank_reason_shape() {
    for reason in [
        "review requested",
        " review requested ",
        "review\trequested",
    ] {
        let stdout = scan_stdout(&["scan", "./skill.md", "--reason", reason]);
        assert_scan_directive(&stdout);
    }
}

#[test]
fn scan_quiet_and_json_flags_keep_directive_stdout() {
    for extra_flags in [vec!["--quiet"], vec!["--json"], vec!["--quiet", "--json"]] {
        let mut args = vec!["scan", "./skill.md"];
        args.extend(extra_flags);
        let stdout = scan_stdout(&args);
        assert_scan_directive(&stdout);
    }
}

#[test]
fn boolean_flag_with_explicit_value_is_usage_error() {
    bspector()
        .args(["scan", "./skill.md", "--json=false"])
        .assert()
        .code(64)
        .stderr(predicate::str::contains("unexpected value"));
}

#[test]
fn update_command_is_deferred_and_exits_successfully() {
    let stdout = success_stdout(&["update"]);
    assert_eq!(
        "bspector update placeholder: behavior is deferred.\n",
        stdout
    );
}

#[test]
fn unknown_command_exits_with_usage_failure() {
    bspector()
        .arg("unknown")
        .assert()
        .code(64)
        .stderr(predicate::str::contains("unrecognized subcommand"));
}

#[test]
fn malformed_flag_shapes_exit_with_usage_failure() {
    for (args, stderr_fragment) in [
        (&["scan", "--reason"][..], "a value is required"),
        (&["scan", "--unknown"][..], "unexpected argument"),
    ] {
        bspector()
            .args(args)
            .assert()
            .code(64)
            .stderr(predicate::str::contains(stderr_fragment));
    }
}

#[test]
fn scan_missing_artefact_exits_with_usage_failure() {
    bspector().arg("scan").assert().code(64);
}

#[test]
fn scan_help_exits_successfully() {
    bspector().args(["scan", "--help"]).assert().success();
}
