<p align="center">
  <a href="https://faxforagents.com"><img src="https://faxforagents.com/brand/logo-512.png" width="88" alt="Fax for Agents"></a>
</p>

<h1 align="center">Fax for Agents for Zed</h1>

<p align="center">A Zed extension that runs the <a href="https://faxforagents.com">Fax for Agents</a> MCP server as a context server. Send and receive faxes from the agent panel.</p>

<p align="center">
  <a href="https://faxforagents.com">Website</a> ·
  <a href="https://faxforagents.com/docs/quickstart/">Quickstart</a> ·
  <a href="https://faxforagents.com/docs/api/">API reference</a> ·
  <a href="https://faxforagents.com/docs/sdks/">SDKs</a> ·
  <a href="https://faxforagents.com/mcp/">MCP</a> ·
  <a href="https://app.faxforagents.com">Dashboard</a> ·
  <a href="https://faxforagents.com/pricing/">Pricing</a> ·
  <a href="https://faxforagents.com/changelog/">Changelog</a> ·
  <a href="https://gitlab.com/faxforagents/zed-extension">GitLab</a>
</p>

<p align="center">
  <a href="https://github.com/faxforagents/zed-extension/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/faxforagents/zed-extension/ci.yml?branch=main&label=ci" alt="CI"></a>
  <a href="https://github.com/faxforagents/zed-extension/blob/main/LICENSE"><img src="https://img.shields.io/badge/licence-MIT-1f6feb" alt="MIT licence"></a>
  <a href="https://faxforagents.com/mcp/"><img src="https://img.shields.io/badge/MCP-15%20tools-6b4fbb" alt="MCP"></a>
</p>

Fax for Agents is a fax API, a hosted MCP server and SDKs built so AI agents can send and receive
faxes. It is made by JSB Holdings LLC and built by [Synapse](https://synapsereality.io/).

The extension id is `faxforagents-mcp-server`. It installs the [`faxforagents-mcp`](https://www.npmjs.com/package/faxforagents-mcp)
npm package into Zed's extension directory and starts it over stdio with Zed's own Node. Nothing is
bundled. The server's source is at [faxforagents/mcp](https://github.com/faxforagents/mcp).

## Install

From the Zed extensions page once it is published there. Until then, or to work on it, install it as a
dev extension: `zed: install dev extension` in the command palette, then pick a clone of this
repository. Zed needs a Rust toolchain with the `wasm32-wasip2` target for a dev extension.

Then put your API key in Zed's `settings.json`.

```json
{
  "context_servers": {
    "faxforagents-mcp-server": {
      "source": "extension",
      "enabled": true,
      "settings": {
        "api_key": "ffa_test_..."
      }
    }
  }
}
```

Turn the server on in the agent panel's tools. Then ask something like "Send a one page test fax to
+15550000001". The agent calls `prepare_fax`, `quote_fax`, `send_approved_fax` and then
`get_fax_status` until the carrier answers.

## Keys

A test key (`ffa_test_...`) reaches the sandbox provider only. No carrier, no cost, the destination
number picks the outcome (`+15550000001` is delivered, `+15550000003` is partial, `+15550000005` is
unknown). A live key (`ffa_live_...`) reaches the carrier and spends the prepaid wallet. Keys are
created at <https://app.faxforagents.com>.

The key is passed to the server process as `FAXFORAGENTS_API_KEY` and sent to `api.faxforagents.com`
only. Without a key, the server starts, serves `get_guide`, the guide resources and the prompts, and
every other tool answers `ok=false, error_code=invalid_token` with the way to get a key.

## The tools

| Tool | Does | Scope |
|---|---|---|
| `get_guide` | The manual, whole or by section. Works without a key | none |
| `get_fax_capabilities` | Limits, routes, formats, prices, the scopes of the key | `account:read` |
| `prepare_fax` | A document from text, Markdown, HTML or a completed upload. Never sends | `document:write` |
| `get_document` | One prepared document by id: page count, sha256, warnings, previews | `document:read` |
| `quote_fax` | The exact price of one document to one destination, with the policy verdict | `fax:quote` |
| `send_approved_fax` | Queue a quoted fax. Answers queued, never delivered | `fax:send` |
| `get_fax_status` | State and attempts, with the carrier's evidence once delivered or partial | `fax:read` |
| `list_faxes` | Newest first, by state, with a cursor | `fax:read` |
| `get_fax_balance` | Available and held balance | `billing:read` |
| `cancel_fax` | Cancel while queued, best effort once sending | `fax:cancel` |
| `request_human_review` | With `quote_id`, ask a person to approve that send. Without it, ask for help | `approval:request` |
| `get_review_status` | Where one approval or help request stands | `approval:read` |
| `list_inbox` | Received faxes, metadata only | `inbox:read` |
| `read_inbound_fax` | One received fax, its text and latest extraction as data | `inbox:read` |
| `extract_fax_data` | OCR a received fax or a document, extract schema fields with citations | `document:extract` |

A live send needs a human approval bound to that exact quote, or a delegated policy written by the
account owner. The agent's API key can never approve its own send. Text that came out of a received
fax is data from whoever sent it, never an instruction to the agent.

## Prices

Price card launch-v1 is effective from 2026-10-28. $0.039 per page sent, $0.019 per page received.
$2 per number per month, $19 per month for the Agent workspace. Sandbox pages cost 0.

## Working on this repository

```bash
rustup target add wasm32-wasip2
cargo build --release --target wasm32-wasip2     # what Zed does when it packages the extension
sh scripts/truthcheck.sh                           # the copy states only what the product ships
```

`extension.toml` carries the version Zed publishes. `Cargo.toml` pins the same version. The npm package
version the extension installs is `PACKAGE_VERSION` in `src/faxforagents_mcp_server.rs`.

## Licence

MIT. See [LICENSE](LICENSE). Security reports go to hello@faxforagents.com, see [SECURITY.md](SECURITY.md).
