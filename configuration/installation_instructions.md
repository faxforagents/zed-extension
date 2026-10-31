Create an API key at https://app.faxforagents.com and paste it as `api_key`.

A test key (`ffa_test_...`) reaches the sandbox only: no carrier, no cost, and the destination number picks the outcome (`+15550000001` is delivered). A live key (`ffa_live_...`) reaches the carrier and spends the prepaid wallet.

The extension installs the `faxforagents-mcp` npm package and runs it with Zed's Node. The key is sent to api.faxforagents.com only. Docs: https://faxforagents.com/mcp/
