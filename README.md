# bspector

For agents that install tools mid-session. Before your AI coding agent runs `npm install some-cli` or registers an MCP server, bspector reads the tool's capability manifest (what files it can touch, what network it calls, what permissions it requests) and decides SAFE, UNSAFE, or MALFORMED. Excessive-agency declarations (e.g. write-everywhere, full network egress) get refused with a directive that tells the agent why and what to try instead. The capability taxonomy is closed (12 vulnerability patterns at v0.1); the prompt library that powers each verdict evolves continuously via empirical-lift evaluation, so the same `bspector scan ./skill.md` invocation gets more accurate at flagging risk as the corpus matures.


Prompt lookup tool. Agent names a vulnerability pattern from a fixed list; bspector returns the prompt for that vulnerability pattern. The prompt tells the agent how to check the skill manifest for that vulnerability.

Built for agentic loops. Scans a skill manifest or extension descriptor, matches against a closed 12-pattern vulnerability taxonomy, writes a verdict directive on stdout, exits with a discriminating code so the calling agent can branch.

```
bspector scan <artefact>    scan a manifest against the vulnerability-pattern taxonomy; exit 0 / 1 / 2 / 64
bspector vuln-patterns      list the 12 supported vulnerability-pattern identifiers
bspector update             self-update to the latest published version
```

Exit code contract: `0` safe (no vulnerability matched), `1` unsafe (vulnerability matched), `2` internal error, `64` malformed input.

## Install

```sh
cargo install --git https://github.com/bvasilenko/bSpector
```

## Use

```sh
bspector scan ./skill.md --artefact-shape skill-md
# stdout: [bspector placeholder directive - pre-corpus output] ...
# exit: 1

bspector scan https://example.com/skill.md --artefact-shape skill-md --strictness strict
# exit: 1

bspector vuln-patterns
# stdout: prompt-injection
#         data-exfiltration
#         ...
```

Optional flags for `scan`: `--artefact-shape <shape>`, `--strictness <level>`, `--llm-stage`, `--osv-online`, `--manifest <path>`, `--json`, `--quiet`, `--reason <text>`.

## Vulnerability pattern taxonomy

Closed 12-variant `VulnerabilityPattern` enum. The taxonomy is fixed at this version; widening lands in a later version.

| Category | Variants |
|---|---|
| Prompt integrity | `prompt-injection`, `system-prompt-leakage`, `memory-poisoning`, `trigger-abuse` |
| Data and access | `data-exfiltration`, `privilege-escalation`, `least-privilege-violation` |
| Supply and tools | `supply-chain`, `tool-misuse`, `excessive-agency` |
| Agent behavior | `rogue-agent`, `output-handling` |

`bspector vuln-patterns` prints the full list.

## License

MIT.
