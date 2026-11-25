# Project Title

FriendLoan

## Project Description

FriendLoan is a simple loan-recording smart contract built on Stellar Soroban. It allows users to create and manage loan records between a lender and a borrower.

The contract stores information such as the lender, borrower, loan amount, due date, and repayment status. The borrower can confirm a loan, and the lender can mark it as repaid after repayment has been handled.

FriendLoan records loan information on the blockchain but does not transfer XLM or other tokens.

## Project Vision

FriendLoan aims to provide a simple and transparent way for people to record informal loans between friends.

By storing loan details and status on the blockchain, the project helps participants review their loan agreements and track whether a loan has been confirmed or marked as repaid.

## Key Features

- Create a loan record with a lender, borrower, amount, and due date.
- Require the lender's authorization when creating a loan.
- Require the borrower's authorization to confirm a loan.
- Allow the lender to mark a confirmed loan as repaid.
- Retrieve loan details using a loan ID.
- Retrieve a borrower's total recorded outstanding amount.
- Validate that the loan amount is greater than zero.
- Prevent a user from lending to themselves.
- Validate that the loan due date is in the future.
- Prevent a loan from being confirmed more than once.
- Prevent a loan from being marked as repaid more than once.
- Store loan records and outstanding balances in persistent Soroban storage.

## Usage Instructions

The contract can be interacted with using the Stellar CLI on Stellar Testnet.

Before invoking the contract, replace `my-contract` with the deployed contract ID or configured alias. Replace wallet aliases with the corresponding Stellar addresses if required by your CLI.

### 1. Create a Loan

The lender creates a loan record.

```bash
stellar contract invoke --id my-contract --source-account student --network testnet -- create_loan --lender student --borrower borrower --amount 1000000000 --due_date 2000000000
```

The contract returns a loan ID. The example assumes the returned loan ID is `1`.

### 2. Confirm the Loan

The borrower confirms the loan using their own wallet.

```bash
stellar contract invoke --id my-contract --source-account borrower --network testnet -- confirm_loan --borrower borrower --loan_id 1
```

### 3. Get Loan Details

Retrieve the stored information for a loan.

```bash
stellar contract invoke --id my-contract --source-account student --network testnet -- get_loan --loan_id 1
```

### 4. Mark the Loan as Repaid

After repayment has been handled outside the contract, the lender can mark the loan as repaid.

```bash
stellar contract invoke --id my-contract --source-account student --network testnet -- mark_repaid --lender student --loan_id 1
```

### 5. Check the Outstanding Amount

Retrieve the borrower's total recorded outstanding amount.

```bash
stellar contract invoke --id my-contract --source-account student --network testnet -- get_amount_owed --borrower borrower
```

The loan amount is stored as an `i128` integer. If the amount is expressed in XLM, convert it to stroops before calling the contract: 1 XLM = 10,000,000 stroops.

## Future Scope

The following features are not implemented in the current contract and may be considered for future development:

- Actual XLM or token transfers for lending and repayment.
- On-chain repayment verification or payment evidence.
- Partial loan repayments.
- Loan cancellation or modification.
- Notifications for upcoming or overdue due dates.
- A user interface for creating and managing loan records.
- Additional loan history and filtering features.

## Technology Stack

- Rust
- `#![no_std]`
- Stellar Soroban smart contracts
- `soroban-sdk` version 29
- Stellar Testnet
- Stellar CLI
- Persistent Soroban contract storage

## Contract Detail

- Contract ID: <CCB6J6FFW7GEWYMU6YPEG6NXVBE3WCLKAAZHLB7U43V7QI3QBACPQ6D7>
- Deployer: <GDX7TSIETRNDHTXENQ5MXQB6BVC6UXEKYTYIH6K7ZQVEQSOVVJBGPE4A>
![Contract on Stellar Expert](image.png)

