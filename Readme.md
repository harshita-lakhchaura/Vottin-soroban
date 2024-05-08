# Simple Voting Smart Contract

## Overview

The SimpleVoting contract enables a basic voting system on the Soroban blockchain platform. It allows for the initialization of a voting process and enables users to cast their votes within a specified period.

## Features

- **Initialization of Voting**: Initialize the voting process with parameters such as voting period, target approval rate, and total voters.
- **Proposal Creation**: Create a proposal with voting end time, target approval rate, total voters count, and an empty list of voters upon initialization.
- **Voting**: Users can cast their votes during the voting period, and each vote increments the vote count of the proposal.
- **Vote Validation**: Ensure that users can only vote once and within the specified voting period.
- **Approval Rate Calculation**: Calculate the approval rate based on the number of votes received compared to the total voters.
- **Approval Status**: Determine whether the proposal is approved based on its approval rate.

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

## Implementation Details
The contract is implemented using the Soroban SDK and utilizes persistent storage for maintaining voting state. Event publishing is used for notifying interested parties about voting actions.
