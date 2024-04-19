# Simple Voting Smart Contract

This smart contract implements a simple voting system where users can vote on a proposal within a specified period.

## Features

- Initialize voting period and parameters.
- Allow voters to cast their votes.
- Track voting progress and calculate approval rate.

## Error Handling

This contract utilizes error codes to handle various scenarios, including:

- Already initialized contract.
- Conversion errors.
- Expected storage keys.
- Already voted.
- Overflow conditions.
- Closed voting periods.

## Usage

1. *Initialize Voting*: Call init_voting to set up the voting period and parameters.

2. *Vote*: Voters can cast their votes using the vote function.

3. *Track Voting*: Monitor voting progress and approval rate using the contract's state.

## Data Structures

### Proposal

- voting_end_time: Timestamp indicating the end of the voting period.
- votes: Total number of votes received.
- target_approval_rate_bps: Target approval rate in basis points.
- total_voters: Total number of eligible voters.
- voters: Map