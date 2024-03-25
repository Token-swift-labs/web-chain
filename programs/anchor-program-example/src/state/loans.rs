use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone,Copy,InitSpace,PartialEq)]
pub enum LoanState {
    Acitve,
    Pending,
    Closed,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone,Copy,InitSpace)]
pub struct Loan {
    pub loan_id: u32,
    pub nft_id: u32,
    pub req_amount: u64,
    pub interest: u64,
    pub period: u64,
    pub paid_amount: u64,
    pub lender: Pubkey,
    pub borrower: Pubkey,
    pub state: LoanState,
}   

#[account]
#[derive(InitSpace)] 
pub struct LoanPDA {
    pub bump: u8,
    pub loans: [Option<Loan>; 10],
    pub loan_count: u8,

}

impl LoanPDA{
    pub fn add_loan(&mut self,loan:Loan)->&str{
        if let Some(index) = self.loans.iter().position(|&x| x.is_none()) {
            self.loans[index] = Some(loan);
            return "success";
        }else{
            return "no-space"
        }

        

    }
    pub fn destroy_loan(&mut self,loan_id:u32){
        if let Some(index) = self.loans.iter().position(|&x| x.is_some() && x.unwrap().loan_id == loan_id) {
            if self.loans[index].unwrap().state != LoanState::Pending {
                return;
            }
            self.loans[index] = None;
        }
    }
}