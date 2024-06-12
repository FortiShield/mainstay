// This file is autogenerated with https://github.com/acheroncrypto/native-to-mainstay

use mainstay_lang::prelude::*;

declare_id!("11111111111111111111111111111111");

#[program]
pub mod spl_token_swap {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, fees: Fees, swap_curve: SwapCurve) -> Result<()> {
        Ok(())
    }

    pub fn swap(ctx: Context<Swap>, amount_in: u64, minimum_amount_out: u64) -> Result<()> {
        Ok(())
    }

    pub fn deposit_all_token_types(
        ctx: Context<DepositAllTokenTypes>,
        pool_token_amount: u64,
        maximum_token_a_amount: u64,
        maximum_token_b_amount: u64,
    ) -> Result<()> {
        Ok(())
    }

    pub fn withdraw_all_token_types(
        ctx: Context<WithdrawAllTokenTypes>,
        pool_token_amount: u64,
        minimum_token_a_amount: u64,
        minimum_token_b_amount: u64,
    ) -> Result<()> {
        Ok(())
    }

    pub fn deposit_single_token_type_exact_amount_in(
        ctx: Context<DepositSingleTokenTypeExactAmountIn>,
        source_token_amount: u64,
        minimum_pool_token_amount: u64,
    ) -> Result<()> {
        Ok(())
    }

    pub fn withdraw_single_token_type_exact_amount_out(
        ctx: Context<WithdrawSingleTokenTypeExactAmountOut>,
        destination_token_amount: u64,
        maximum_pool_token_amount: u64,
    ) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    swap: Signer<'info>,
    authority: AccountInfo<'info>,
    token_a: AccountInfo<'info>,
    token_b: AccountInfo<'info>,
    #[account(mut)]
    pool: AccountInfo<'info>,
    fee: AccountInfo<'info>,
    #[account(mut)]
    destination: AccountInfo<'info>,
    token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Swap<'info> {
    swap: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    user_transfer_authority: Signer<'info>,
    #[account(mut)]
    source: AccountInfo<'info>,
    #[account(mut)]
    swap_source: AccountInfo<'info>,
    #[account(mut)]
    swap_destination: AccountInfo<'info>,
    #[account(mut)]
    destination: AccountInfo<'info>,
    #[account(mut)]
    pool_mint: AccountInfo<'info>,
    #[account(mut)]
    pool_fee: AccountInfo<'info>,
    token_program: Program<'info, Token>,
    // #[account(mut)]
    // optional_host_fee: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct DepositAllTokenTypes<'info> {
    swap: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    user_transfer_authority: Signer<'info>,
    #[account(mut)]
    deposit_token_a: AccountInfo<'info>,
    #[account(mut)]
    deposit_token_b: AccountInfo<'info>,
    #[account(mut)]
    swap_token_a: AccountInfo<'info>,
    #[account(mut)]
    swap_token_b: AccountInfo<'info>,
    #[account(mut)]
    pool_mint: AccountInfo<'info>,
    #[account(mut)]
    destination: AccountInfo<'info>,
    token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct WithdrawAllTokenTypes<'info> {
    swap: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    user_transfer_authority: Signer<'info>,
    #[account(mut)]
    pool_mint: AccountInfo<'info>,
    #[account(mut)]
    source: AccountInfo<'info>,
    #[account(mut)]
    swap_token_a: AccountInfo<'info>,
    #[account(mut)]
    swap_token_b: AccountInfo<'info>,
    #[account(mut)]
    destination_token_a: AccountInfo<'info>,
    #[account(mut)]
    destination_token_b: AccountInfo<'info>,
    #[account(mut)]
    fee_account: AccountInfo<'info>,
    token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct DepositSingleTokenTypeExactAmountIn<'info> {
    swap: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    user_transfer_authority: Signer<'info>,
    #[account(mut)]
    source_token: AccountInfo<'info>,
    #[account(mut)]
    swap_token_a: AccountInfo<'info>,
    #[account(mut)]
    swap_token_b: AccountInfo<'info>,
    #[account(mut)]
    pool_mint: AccountInfo<'info>,
    #[account(mut)]
    destination: AccountInfo<'info>,
    token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct WithdrawSingleTokenTypeExactAmountOut<'info> {
    swap: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    user_transfer_authority: Signer<'info>,
    #[account(mut)]
    pool_mint: AccountInfo<'info>,
    #[account(mut)]
    pool_token_source: AccountInfo<'info>,
    #[account(mut)]
    swap_token_a: AccountInfo<'info>,
    #[account(mut)]
    swap_token_b: AccountInfo<'info>,
    #[account(mut)]
    destination: AccountInfo<'info>,
    #[account(mut)]
    fee_account: AccountInfo<'info>,
    token_program: Program<'info, Token>,
}

#[account]
pub struct Swap {
    // Swap version
    pub version: u8,

    /// Initialized state.
    pub is_initialized: bool,
    /// Bump seed used in program address.
    /// The program address is created deterministically with the bump seed,
    /// swap program id, and swap account pubkey.  This program address has
    /// authority over the swap's token A account, token B account, and pool
    /// token mint.
    pub bump_seed: u8,

    /// Program ID of the tokens being exchanged.
    pub token_program_id: Pubkey,

    /// Token A
    pub token_a: Pubkey,
    /// Token B
    pub token_b: Pubkey,

    /// Pool tokens are issued when A or B tokens are deposited.
    /// Pool tokens can be withdrawn back to the original A or B token.
    pub pool_mint: Pubkey,

    /// Mint information for token A
    pub token_a_mint: Pubkey,
    /// Mint information for token B
    pub token_b_mint: Pubkey,

    /// Pool token account to receive trading and / or withdrawal fees
    pub pool_fee_account: Pubkey,

    /// All fee information
    pub fees: Fees,

    /// Swap curve parameters, to be unpacked and used by the SwapCurve, which
    /// calculates swaps, deposits, and withdrawals
    pub swap_curve: SwapCurve,
}

#[derive(MainstaySerialize, MainstayDeserialize)]
pub struct Fees {
    /// Trade fees are extra token amounts that are held inside the token
    /// accounts during a trade, making the value of liquidity tokens rise.
    /// Trade fee numerator
    pub trade_fee_numerator: u64,
    /// Trade fee denominator
    pub trade_fee_denominator: u64,

    /// Owner trading fees are extra token amounts that are held inside the token
    /// accounts during a trade, with the equivalent in pool tokens minted to
    /// the owner of the program.
    /// Owner trade fee numerator
    pub owner_trade_fee_numerator: u64,
    /// Owner trade fee denominator
    pub owner_trade_fee_denominator: u64,

    /// Owner withdraw fees are extra liquidity pool token amounts that are
    /// sent to the owner on every withdrawal.
    /// Owner withdraw fee numerator
    pub owner_withdraw_fee_numerator: u64,
    /// Owner withdraw fee denominator
    pub owner_withdraw_fee_denominator: u64,

    /// Host fees are a proportion of the owner trading fees, sent to an
    /// extra account provided during the trade.
    /// Host trading fee numerator
    pub host_fee_numerator: u64,
    /// Host trading fee denominator
    pub host_fee_denominator: u64,
}

#[derive(MainstaySerialize, MainstayDeserialize)]
pub struct SwapCurve {
    /// The type of curve contained in the calculator, helpful for outside
    /// queries
    pub curve_type: CurveType,
    /// The actual calculator, represented as a trait object to allow for many
    /// different types of curves
    // pub calculator: Arc<dyn CurveCalculator + Sync + Send>,
    pub calculator: [u8; 32],
}

#[derive(MainstaySerialize, MainstayDeserialize)]
pub struct StableCurve {
    /// Amplifier constant
    pub amp: u64,
}

#[derive(MainstaySerialize, MainstayDeserialize)]
pub struct OffsetCurve {
    /// Amount to offset the token B liquidity account
    pub token_b_offset: u64,
}

#[derive(MainstaySerialize, MainstayDeserialize)]
pub struct ConstantProductCurve {}

#[derive(MainstaySerialize, MainstayDeserialize)]
pub struct ConstantPriceCurve {
    /// Amount of token A required to get 1 token B
    pub token_b_price: u64,
}

#[derive(MainstaySerialize, MainstayDeserialize)]
pub enum CurveType {
    /// Uniswap-style constant product curve, invariant = token_a_amount * token_b_amount
    ConstantProduct,
    /// Flat line, always providing 1:1 from one token to another
    ConstantPrice,
    /// Stable, like uniswap, but with wide zone of 1:1 instead of one point
    Stable,
    /// Offset curve, like Uniswap, but the token B side has a faked offset
    Offset,
}

#[error_code]
pub enum SwapError {
    // 0.
    /// The account cannot be initialized because it is already being used.
    #[msg("Swap account already in use")]
    AlreadyInUse,
    /// The program address provided doesn't match the value generated by the program.
    #[msg("Invalid program address generated from bump seed and key")]
    InvalidProgramAddress,
    /// The owner of the input isn't set to the program address generated by the program.
    #[msg("Input account owner is not the program address")]
    InvalidOwner,
    /// The owner of the pool token output is set to the program address generated by the program.
    #[msg("Output pool account owner cannot be the program address")]
    InvalidOutputOwner,
    /// The deserialization of the account returned something besides State::Mint.
    #[msg("Deserialized account is not an SPL Token mint")]
    ExpectedMint,

    // 5.
    /// The deserialization of the account returned something besides State::Account.
    #[msg("Deserialized account is not an SPL Token account")]
    ExpectedAccount,
    /// The input token account is empty.
    #[msg("Input token account empty")]
    EmptySupply,
    /// The pool token mint has a non-zero supply.
    #[msg("Pool token mint has a non-zero supply")]
    InvalidSupply,
    /// The provided token account has a delegate.
    #[msg("Token account has a delegate")]
    InvalidDelegate,
    /// The input token is invalid for swap.
    #[msg("InvalidInput")]
    InvalidInput,

    // 10.
    /// Address of the provided swap token account is incorrect.
    #[msg("Address of the provided swap token account is incorrect")]
    IncorrectSwapAccount,
    /// Address of the provided pool token mint is incorrect
    #[msg("Address of the provided pool token mint is incorrect")]
    IncorrectPoolMint,
    /// The output token is invalid for swap.
    #[msg("InvalidOutput")]
    InvalidOutput,
    /// General calculation failure due to overflow or underflow
    #[msg("General calculation failure due to overflow or underflow")]
    CalculationFailure,
    /// Invalid instruction number passed in.
    #[msg("Invalid instruction")]
    InvalidInstruction,

    // 15.
    /// Swap input token accounts have the same mint
    #[msg("Swap input token accounts have the same mint")]
    RepeatedMint,
    /// Swap instruction exceeds desired slippage limit
    #[msg("Swap instruction exceeds desired slippage limit")]
    ExceededSlippage,
    /// The provided token account has a close authority.
    #[msg("Token account has a close authority")]
    InvalidCloseAuthority,
    /// The pool token mint has a freeze authority.
    #[msg("Pool token mint has a freeze authority")]
    InvalidFreezeAuthority,
    /// The pool fee token account is incorrect
    #[msg("Pool fee token account incorrect")]
    IncorrectFeeAccount,

    // 20.
    /// Given pool token amount results in zero trading tokens
    #[msg("Given pool token amount results in zero trading tokens")]
    ZeroTradingTokens,
    /// The fee calculation failed due to overflow, underflow, or unexpected 0
    #[msg("Fee calculation failed due to overflow, underflow, or unexpected 0")]
    FeeCalculationFailure,
    /// ConversionFailure
    #[msg("Conversion to u64 failed with an overflow or underflow")]
    ConversionFailure,
    /// The provided fee does not match the program owner's constraints
    #[msg("The provided fee does not match the program owner's constraints")]
    InvalidFee,
    /// The provided token program does not match the token program expected by the swap
    #[msg("The provided token program does not match the token program expected by the swap")]
    IncorrectTokenProgramId,

    // 25.
    /// The provided curve type is not supported by the program owner
    #[msg("The provided curve type is not supported by the program owner")]
    UnsupportedCurveType,
    /// The provided curve parameters are invalid
    #[msg("The provided curve parameters are invalid")]
    InvalidCurve,
    /// The operation cannot be performed on the given curve
    #[msg("The operation cannot be performed on the given curve")]
    UnsupportedCurveOperation,
}
