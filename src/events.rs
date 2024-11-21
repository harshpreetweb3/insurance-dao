use scrypto::prelude::*;

#[allow(non_camel_case_types)]
#[derive(ScryptoSbor, ScryptoEvent)]
pub enum EventType {
    DEPLOYMENT,

    TOKEN_BOUGHT,

    TOKEN_SELL,

    PRAPOSAL,

    VOTE,

    EXECUTE_PROPOSAL,

    TREASURY_CONTRIBUTION,

    ZERO_COUPON_BOND_CREATION,

    ANN_TOKEN_CREATION,

    QUORUM_NOT_MET,

    QUORUM_MET,

    ANNUAL_PAYOUT_CLAIMED,

    ANNUAL_PAYOUT_COULD_NOT_BE_CLAIMED
}

#[derive(ScryptoSbor, ScryptoEvent)]
pub enum DaoType {
    Investment,
    Insurance
}

#[derive(ScryptoSbor, ScryptoEvent)]
pub struct TokenWightedDeployment {
    pub component_address: ComponentAddress,

    pub token_address: ResourceAddress,

    pub owner_token_address: ResourceAddress,

    pub community_name: String,

    pub community_image: String,

    pub token_price: Decimal,

    pub token_buy_back_price: Decimal,

    pub description: String,

    pub total_token: i32,

    pub token_image: String,

    pub tags: Vec<String>,

    pub purpose: String,

    pub proposal_creation_right: ProposalCreationRight,
}

#[derive(ScryptoSbor, ScryptoEvent)]
pub struct TokenWeightBuyToken {
    pub amount: Decimal,

    pub resource_address: ResourceAddress,

    pub amount_paid: Decimal,

    pub current_component_share: Decimal,
}

#[derive(ScryptoSbor, ScryptoEvent)]
pub struct PraposalMetadata {
    // a simple string representing current praposal
    pub title: String,
    pub description: String,
    // represent the minimum amount of quorm requires for this praposal to pass
    pub minimum_quorum: Decimal,
    pub end_time_ts: i64,
    pub start_time_ts: i64,
    pub owner_token_address: ResourceAddress,
    pub component_address: ComponentAddress, // votes:HashMap<Address,Decimal>
    pub address_issued_bonds_to_sell: Option<ComponentAddress>,
    pub target_xrd_amount: Option<Decimal>,
    pub proposal_creator_address: Option<ComponentAddress>,
    pub amount_of_tokens_should_be_minted: Option<usize>,
    pub proposal_id: usize,
    pub governance_token_or_owner_token_address: ResourceAddress,
    pub token_type: VotingType,
}

#[derive(ScryptoSbor, ScryptoEvent)]
pub enum DaoEvent {
    ProposalExecute(PraposalExecute),

    TokenWeightedDEployment(TokenWightedDeployment),

    TokenWeightedTokenPurchase(TokenWeightBuyToken),

    PraposalDeployment(PraposalMetadata),

    PraposalVote(ProposalVote),

    TreasuryContribution(TreasuryContribution),

    ZeroCouponBondCreation(ZeroCouponBondCreation),

    AnnTokenCreation(AnnuityTokenCreation),

    ProposalQuorumNotMet(ProposalQuorumNotMet), // New event type

    ProposalQuorumMet(ProposalQuorumMet), // ProposalCreationRightEveryone,

                                          // ProposalCreationRightTokenHolderThreshold(Decimal),

                                          // ProposalCreationRightAdmin

    ClaimAnnualPayout(ClaimAnnualPayout)
}

// #[derive(ScryptoSbor, ScryptoEvent)]
// pub enum ProposalRightEvent {

//     ProposalCreationRightEveryone,

//     ProposalCreationRightTokenHolderThreshold(Decimal),

//     ProposalCreationRightAdmin

// }

#[derive(ScryptoSbor, ScryptoEvent)]
pub struct PraposalExecute {
    pub praposal_address: ComponentAddress,
    pub proposal_id: usize, // pub purchased_bond_address : Option<ResourceAddress>,
                            // pub purchased_amount : Decimal
}

#[derive(ScryptoSbor, ScryptoEvent)]
pub struct ProposalVote {
    pub praposal_address: ComponentAddress,
    pub voting_amount: Decimal,
    pub againts: bool,
    pub voter_address: ComponentAddress,
    pub proposal_id: usize,
}

// create an event for community_creation
#[derive(ScryptoSbor, ScryptoEvent)]
pub struct PandaoEvent {
    pub event_type: EventType,

    pub dao_type: DaoType,

    pub component_address: ComponentAddress,

    pub meta_data: DaoEvent,
}

// #[derive(ScryptoSbor, ScryptoEvent)]
// pub struct PandaoAdditionalEvent {

//     pub meta_data: ProposalRightEvent
// }

// create an event for community_creation
#[derive(ScryptoSbor, ScryptoEvent)]
pub struct BoughtToken {
    pub component_address: ComponentAddress,
    pub user_address: ResourceAddress,
    pub amount: Decimal,
}

#[derive(ScryptoSbor, Debug)]
pub struct TreasuryContribution {
    pub contributor: ComponentAddress,
    pub amount: Decimal,
    pub timestamp: u64,
}

#[allow(non_camel_case_types)]
#[derive(ScryptoSbor, Clone, Debug, PartialEq, Eq)]
pub enum ProposalCreationRight {
    EVERYONE,
    TOKEN_HOLDER_THRESHOLD(Decimal),
    ADMIN,
}

#[allow(non_camel_case_types)]
#[derive(ScryptoSbor, Clone, Debug, PartialEq, Eq)]
pub enum VotingType {
    ResourceHold,
    Equality,
}

#[derive(ScryptoSbor, ScryptoEvent)]
pub struct ZeroCouponBondCreation {
    pub component_address: ComponentAddress,
    pub contract_type: String,
    pub contract_role: String,
    pub contract_identifier: String,
    pub nominal_interest_rate: Decimal,
    pub currency: String,
    pub initial_exchange_date: u64,
    pub maturity_date: u64,
    pub notional_principal: Decimal,
    pub discount: u64,
    pub bond_position: String,
    pub price: Decimal,
    pub number_of_bonds: Decimal,
    pub creator_address: ComponentAddress,
}

// ANN TOKEN CREATION
#[derive(ScryptoSbor, ScryptoEvent)]
pub struct AnnuityTokenCreation {
    pub component_address: ComponentAddress,
    pub contract_type: String,
    pub contract_role: String,
    pub contract_identifier: String,
    pub nominal_interest_rate: Decimal,
    pub currency: String,
    pub initial_exchange_date: u64,
    pub maturity_date: u64,
    pub notional_principal: Decimal,
    pub annuity_position: String,
    pub price: Decimal,
    pub number_of_annuities_to_mint: Decimal,
    pub your_address: ComponentAddress,
}

#[derive(ScryptoSbor, ScryptoEvent)]
pub struct ProposalQuorumNotMet {
    pub proposal_id: usize,
    pub minimum_quorum: Decimal,
    pub number_of_voters: usize,
}

#[derive(ScryptoSbor, ScryptoEvent)]
pub struct ProposalQuorumMet {
    pub proposal_id: usize,
    pub minimum_quorum: Decimal,
    pub number_of_voters: usize,
}

#[derive(ScryptoSbor, ScryptoEvent)]
pub struct ClaimAnnualPayout {
    pub message : String
}
