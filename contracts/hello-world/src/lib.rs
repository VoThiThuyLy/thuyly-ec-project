#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, Address, Env,
};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum FriendLoanError {
    InvalidAmount = 1,
    SelfLoan = 2,
    InvalidDueDate = 3,
    LoanNotFound = 4,
    NotBorrower = 5,
    NotLender = 6,
    AlreadyConfirmed = 7,
    NotConfirmed = 8,
    AlreadyRepaid = 9,
    AmountOverflow = 10,
    CounterOverflow = 11,
    InvalidOwedBalance = 12,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LoanRecord {
    pub id: u64,
    pub lender: Address,
    pub borrower: Address,
    pub amount: i128,
    pub due_date: u64,
    pub confirmed: bool,
    pub repaid: bool,
}

#[contracttype]
pub enum DataKey {
    LoanCounter,
    Loan(u64),
    Owed(Address),
}

#[contract]
pub struct FriendLoan;

#[contractimpl]
impl FriendLoan {
    // Người cho vay tạo khoản vay mới và lưu thông tin vào blockchain.
    pub fn create_loan(
        env: Env,
        lender: Address,
        borrower: Address,
        amount: i128,
        due_date: u64,
    ) -> Result<u64, FriendLoanError> {
        lender.require_auth();

        if amount <= 0 {
            return Err(FriendLoanError::InvalidAmount);
        }

        if lender == borrower {
            return Err(FriendLoanError::SelfLoan);
        }

        if due_date <= env.ledger().timestamp() {
            return Err(FriendLoanError::InvalidDueDate);
        }

        let storage = env.storage().persistent();

        let current_id = match storage.get::<_, u64>(&DataKey::LoanCounter) {
            Some(value) => value,
            None => 0,
        };

        let loan_id = current_id
            .checked_add(1)
            .ok_or(FriendLoanError::CounterOverflow)?;

        let loan = LoanRecord {
            id: loan_id,
            lender,
            borrower,
            amount,
            due_date,
            confirmed: false,
            repaid: false,
        };

        storage.set(&DataKey::Loan(loan_id), &loan);
        storage.set(&DataKey::LoanCounter, &loan_id);

        Ok(loan_id)
    }

    // Người vay xác nhận khoản vay và cập nhật số tiền đang nợ.
    pub fn confirm_loan(
        env: Env,
        borrower: Address,
        loan_id: u64,
    ) -> Result<(), FriendLoanError> {
        borrower.require_auth();

        let storage = env.storage().persistent();

        let mut loan = storage
            .get::<_, LoanRecord>(&DataKey::Loan(loan_id))
            .ok_or(FriendLoanError::LoanNotFound)?;

        if loan.borrower != borrower {
            return Err(FriendLoanError::NotBorrower);
        }

        if loan.confirmed {
            return Err(FriendLoanError::AlreadyConfirmed);
        }

        let owed_key = DataKey::Owed(borrower.clone());

        let current_owed = match storage.get::<_, i128>(&owed_key) {
            Some(value) => value,
            None => 0,
        };

        let new_owed = current_owed
            .checked_add(loan.amount)
            .ok_or(FriendLoanError::AmountOverflow)?;

        loan.confirmed = true;

        storage.set(&DataKey::Loan(loan_id), &loan);
        storage.set(&owed_key, &new_owed);

        Ok(())
    }

    // Người cho vay đánh dấu khoản vay đã được hoàn trả.
    pub fn mark_repaid(
        env: Env,
        lender: Address,
        loan_id: u64,
    ) -> Result<(), FriendLoanError> {
        lender.require_auth();

        let storage = env.storage().persistent();

        let mut loan = storage
            .get::<_, LoanRecord>(&DataKey::Loan(loan_id))
            .ok_or(FriendLoanError::LoanNotFound)?;

        if loan.lender != lender {
            return Err(FriendLoanError::NotLender);
        }

        if !loan.confirmed {
            return Err(FriendLoanError::NotConfirmed);
        }

        if loan.repaid {
            return Err(FriendLoanError::AlreadyRepaid);
        }

        let owed_key = DataKey::Owed(loan.borrower.clone());

        let current_owed = match storage.get::<_, i128>(&owed_key) {
            Some(value) => value,
            None => 0,
        };

        let new_owed = current_owed
            .checked_sub(loan.amount)
            .ok_or(FriendLoanError::InvalidOwedBalance)?;

        if new_owed < 0 {
            return Err(FriendLoanError::InvalidOwedBalance);
        }

        loan.repaid = true;

        storage.set(&DataKey::Loan(loan_id), &loan);
        storage.set(&owed_key, &new_owed);

        Ok(())
    }

    // Mọi người có thể đọc thông tin một khoản vay theo ID.
    pub fn get_loan(
        env: Env,
        loan_id: u64,
    ) -> Result<LoanRecord, FriendLoanError> {
        env.storage()
            .persistent()
            .get::<_, LoanRecord>(&DataKey::Loan(loan_id))
            .ok_or(FriendLoanError::LoanNotFound)
    }

    // Mọi người có thể đọc tổng số tiền một người đang nợ.
    pub fn get_amount_owed(
        env: Env,
        borrower: Address,
    ) -> Result<i128, FriendLoanError> {
        let owed_key = DataKey::Owed(borrower);

        let amount = match env.storage().persistent().get::<_, i128>(&owed_key) {
            Some(value) => value,
            None => 0,
        };

        Ok(amount)
    }
}