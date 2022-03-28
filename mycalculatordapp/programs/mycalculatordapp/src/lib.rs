use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod mycalculatordapp {
  use super::*;

  // 第一個參數總是Context(上下文), context包含需要從中檢索的Solana帳戶
  // Solana程序中無法存儲數據, 要存儲數據實際上必須創建一個像文件一樣的帳戶, 可以在其中存儲數據, 並且可以放在Solana區塊鏈上
  pub fn create(ctx: Context<Create>, init_message: String) -> Result<()> {
    // 從Context中獲取帳戶, &mut: calculator擁有讀寫權限
    // Note: 1. let a = &b (a只擁有b資源的讀權限)
    //       2. let a = &mut b (a擁有b資源的讀寫權限, 但a不可修改指向)
    //       3. let mut a = &mut b (a擁有b資源的讀寫權限, 並且可以修改指向)
    let calculator = &mut ctx.accounts.calculator;
    calculator.greeting = init_message;
    Ok(())
  }
  // addition
  pub fn add(ctx: Context<Addition>, num1: i64, num2: i64) ->  Result<()> {
    let calculator = &mut ctx.accounts.calculator;
    calculator.result = num1 + num2;
    Ok(())
  }

  // subtraction function
  pub fn sub(ctx: Context<Subtraction>, num1: i64, num2: i64) -> Result<()> {
    let calculator = &mut ctx.accounts.calculator;
    calculator.result = num1 - num2;
    Ok(())
  }
  // multiplication function
  pub fn multiplication(ctx: Context<Multiplication>, num1: i64, num2: i64) -> Result<()> {
    let calculator = &mut ctx.accounts.calculator;
    calculator.result = num1 * num2;
    Ok(())
  }

  // division function
  pub fn division(ctx: Context<Division>, num1: i64, num2: i64) -> Result<()> {
    let calculator = &mut ctx.accounts.calculator;
    calculator.result = num1 / num2;
    Ok(())
  }
}

#[derive(Accounts)] // 定義宏 - Accounts
pub struct Create<'info> {
  // 指定一個新的帳戶
  #[account(init, payer=user, space=264)] // 帳戶, 計算的花費由用戶支付, 分配空間(可調整大小)
  pub calculator: Account<'info, Calculator>,
  // 標記用戶為不可變
  #[account(mut)]
  // 將用戶指定為簽名者
  pub user: Signer<'info>,
  // 對於區塊鏈, 需要做一個系統說明
  pub system_program: Program<'info, System>
}

// Addition
#[derive(Accounts)]
pub struct Addition<'info> {
  #[account(mut)]
  pub calculator: Account<'info, Calculator>
}

// Subtraction
#[derive(Accounts)]
pub struct Subtraction<'info> {
  #[account(mut)]
  pub calculator: Account<'info, Calculator>
}

// Multipilcation
#[derive(Accounts)]
pub struct Multiplication<'info> {
  #[account(mut)]
  pub calculator: Account<'info, Calculator>
}

// Division
#[derive(Accounts)]
pub struct Division<'info> {
  #[account(mut)]
  pub calculator: Account<'info, Calculator>
}

// 定義account的信息
#[account]
pub struct Calculator {
  pub greeting: String,
  pub result: i64, // i64允許包含符號
  pub remainder: i64
} 