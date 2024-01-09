## Taiko Indexer

This repository show cases consumption of an Inscription Protocol and saving it to a database using Diesel.

## Features

Common features:
1. Data Indexing and Storage: The blockchain indexer is capable of parsing and
indexing blockchain data, storing it in a structured format for efficient retrieval. It
indexes transactions, blocks, addresses, and other relevant information, establishing
relationships to support organized and indexed data.
2. Fast Search and Query: The indexer provides powerful search and query
capabilities, allowing users to quickly search blockchain data based on various
criteria and parameters. This includes querying by transaction IDs, block heights,
timestamps, addresses, contracts, etc., to retrieve specific blockchain information.
3. Real-time Synchronization and Updates: The indexer stays in sync with the
blockchain network in real-time, capturing new transactions and blocks and updating
the indexed data accordingly. This ensures that the indexed data reflects the latest
changes happening in the blockchain.
4. High Performance and Scalability: The indexer is designed and optimized to handle
large volumes of blockchain data. It employs efficient data storage and indexing
algorithms to enable fast data retrieval and query response times. Additionally, it is
scalable to accommodate the growing size of the blockchain.
5. Developer Tools and APIs: The indexer provides developer-friendly tools and APIs
for easy integration of blockchain index data into applications. This includes
well-documented APIs, sample code, SDKs, and other auxiliary tools and libraries to
simplify the development process.
Inscription features:
In order to not conflict with the current indexers, we followed Unisat's open-source indexer whenever necessary, also we followed all of the rules at the official BRC-20 gitbook. In addition to these rules, we extensively checked BRC-20 tokens / token movements and cross checked it with other indexers to not have any indexing conflicts.
Here are the detailed indexing rules that we follow:
l Inscription must have a MIME Type of "text/plain" or "application/json". To check this, split the mime type from ";" and check the first part without strip/trim etc.
l Inscription must be a valid JSON (not JSON5)
l JSON must have "p", "op", "tick" fields where "p"="brc-20", "op" in ["deploy", "mint", "transfer"]
l If op is deploy, JSON must have a "max" field. "lim" and "dec" fields are optional. If "dec" is not set, it will be counted as 18. If "lim" is not set it will be equal to "max".
l If op is mint or transfer, JSON must have an "amt" field.
l ALL NECESSARY JSON FIELDS MUST BE STRINGS. Numbers at max, lim, amt, dec etc. are not accepted. Extra fields which haven't been discussed here can be of any type.
l Numeric fields are not stripped/trimmed. "dec" field must have only digits, other numeric fields may have a single dot(".") for decimal representation (+,- etc. are not accepted). Decimal fields cannot start or end with dot (e.g. ".99" and "99." are invalid).
l Empty string for numeric field is invalid.
l 0 for numeric fields is invalid except for "dec" field.
l If any decimal representation have more decimal digits than "dec" of ticker, the inscription will be counted as invalid (even if the extra digits are 0)
l Max value of "dec" is 18.
l Max value of any numeric field is uint64_max.
l "tick" must be 4bytes wide (UTF-8 is accepted). "tick" is case insensitive, we use lowercase letters to track tickers (convert tick to lowercase before processing).
l If a deploy, mint or transfer is sent as fee to miner while inscribing, it must be ignored
l If a transfer is sent as fee in its first transfer, its amount must be returned to sender
l If a mint has been deployed with more amt than lim, it will be ignored.
l If a transfer has been deployed with more amt than available balance of that wallet, it will be ignored.
l All balances are followed using scriptPubKey since some wallets may not have an address attached on bitcoin.
How the protocol works:
l First a deploy inscription is inscribed. This will set the rules for this brc-20 ticker. If the same ticker (case insensitive) has already been deployed, the second deployment will be invalid.
l Then anyone can inscribe mint inscriptions with the limits set in deploy inscription until the minted balance reaches to "max" set in deploy inscription.
l The last mint inscription will mint the remaining tokens. For example, if the ticker only has 100 tokens left but a mint inscription wants to mint 500, it will mint the remaining 100 tokens.
l When a wallet mints a brc-20 token (inscribes a mint inscription to its address), its overall balance and available balance will increase.
l Wallets can inscribe transfer inscriptions with amount up to their available balance.
l If a user inscribe a transfer inscription but does not transfer it, its overall balance will stay the same but its available balance will decrease.
l When this transfer inscription is transferred (not sent as fee to miner) the original wallet's overall balance will decrease but its available balance will stay the same. The receiver's overall balance and available balance will increase.
l If you transfer the transfer inscription to your own wallet, your available balance will increase and the transfer inscription will become used.
l A transfer inscription will become invalid/used after its first transfer.
l Buying, transferring mint and deploy inscriptions will not change anyone's balance.
l Some possible pitfalls:

## Getting Started

Clone the repository:

```bash
git clone https://github.com/alexLizhenyu/Taiko_indexer.git
cd Taiko_indexer
```

The first startup requires creating a docker network:

```bash
docker network create SIP
```

Run the docker container:

```bash
docker-compose -f ./docker-compose-postgres.yml up -d
START_BLOCK=<u64> ENDPOINT_URL=<str> docker-compose up -d
```
