// blue print for
mod events;
use crate::events::*;
mod proposal;
use scrypto::prelude::*;

mod ann;

#[blueprint]
#[events(PandaoEvent, DaoEvent, TokenWightedDeployment, DaoType, EventType)]
mod radixdao {

    use std::collections::HashMap;

    use super::*;
    use proposal::pandao_praposal::TokenWeightProposal;
    use scrypto::address;
    // use scrypto_test::prelude::drop_fungible_bucket;
    use ann::annuity::Annuity;

    pub struct TokenWeigtedDao {
        current_praposals: HashMap<ComponentAddress, HashMap<usize, Global<TokenWeightProposal>>>,

        dao_token_resource_manager: ResourceManager,

        dao_token: Vault,

        organization_name: String,

        shares: Vault,

        ann_tokens: HashMap<ResourceAddress, Vault>,

        dao_token_address: ResourceAddress,

        owner_token_addresss: ResourceAddress,

        token_price: Decimal,

        buy_back_price: Decimal,

        ann_token: HashMap<ComponentAddress, Vec<Global<Annuity>>>,

        contributors: HashMap<ComponentAddress, Decimal>,

        proposal_creation_right: ProposalCreationRight,

        liquidated_collateral: Vault,
    }

    impl TokenWeigtedDao {
        pub fn initiate(
            organization_name: String,

            token_supply: i32,

            divisibility: u8,

            token_price: Decimal,

            token_buy_back_price: Decimal,

            org_ico_url: String,

            power_token_url: String,

            description: String,

            tags: Vec<String>,

            purpose: String,

            proposal_creation_right: ProposalCreationRight,

            token_name: String,
        ) -> (Global<TokenWeigtedDao>, Bucket) {
            // reserve an address for the DAO component
            let (address_reservation, _) =
                Runtime::allocate_component_address(TokenWeigtedDao::blueprint_id());

            let owner_badge_description = format!("{}'s owner badge", &organization_name);

            let owner_badge: Bucket = ResourceBuilder::new_fungible(OwnerRole::None)
                .divisibility(0)
                .metadata(metadata!(
                    init{
                        "name"=>owner_badge_description,locked;
                        "icon_url" => Url::of(&org_ico_url), locked;
                    }
                ))
                .mint_initial_supply(1)
                .into();

            // create nft to be sold for voting purpose
            let dao_token_description = format!("{} voting share", &token_name);

            let voting_power_tokens: Bucket = ResourceBuilder::new_fungible(OwnerRole::None)
                .divisibility(divisibility)
                .metadata(metadata!(init{
                    "name"=>dao_token_description,locked ;
                    "organization name" => organization_name.as_str() , locked ;
                    "icon_url" => Url::of(&power_token_url), locked;
                }))
                .mint_initial_supply(token_supply)
                .into();

            let dao_token_address = voting_power_tokens.resource_address();

            let owner_token_addresss = owner_badge.resource_address();

            let component: Global<TokenWeigtedDao>;

            match proposal_creation_right {
                ProposalCreationRight::EVERYONE => {
                    component = Self {
                        token_price: token_price.clone(),

                        organization_name: organization_name.clone(),

                        dao_token_address: dao_token_address.clone(),

                        owner_token_addresss: owner_token_addresss.clone(),

                        current_praposals: HashMap::new(),

                        dao_token_resource_manager: voting_power_tokens.resource_manager(),

                        dao_token: Vault::with_bucket(voting_power_tokens),

                        buy_back_price: token_buy_back_price.clone(),

                        shares: Vault::new(XRD),

                        ann_tokens: HashMap::new(),

                        // Initialize zero_coupon_bond as None
                        ann_token: HashMap::new(),

                        contributors: HashMap::new(),

                        proposal_creation_right: ProposalCreationRight::EVERYONE,

                        liquidated_collateral: Vault::new(XRD),
                    }
                    .instantiate()
                    .prepare_to_globalize(OwnerRole::Fixed(rule!(require(
                        owner_token_addresss.clone()
                    ))))
                    .with_address(address_reservation.clone())
                    .globalize();

                    let component_address = component.address();

                    let event_metadata = TokenWightedDeployment {
                        component_address,

                        token_address: dao_token_address,

                        owner_token_address: owner_token_addresss,

                        community_name: organization_name,

                        community_image: org_ico_url,

                        token_price,

                        token_buy_back_price,

                        description,

                        total_token: token_supply,

                        token_image: power_token_url,

                        tags: tags.clone(),

                        purpose: purpose.clone(),

                        proposal_creation_right: ProposalCreationRight::EVERYONE,
                    };

                    Runtime::emit_event(PandaoEvent {
                        // event_type: EventType::PROPOSAL_CREATION_RIGHT,
                        event_type: EventType::DEPLOYMENT,
                        dao_type: DaoType::Insurance,
                        component_address,
                        meta_data: DaoEvent::TokenWeightedDEployment(event_metadata),
                    });
                }
                ProposalCreationRight::TOKEN_HOLDER_THRESHOLD(threshold) => {
                    component = Self {
                        token_price: token_price.clone(),

                        organization_name: organization_name.clone(),

                        dao_token_address: dao_token_address.clone(),

                        owner_token_addresss: owner_token_addresss.clone(),

                        current_praposals: HashMap::new(),

                        dao_token_resource_manager: voting_power_tokens.resource_manager(),

                        dao_token: Vault::with_bucket(voting_power_tokens),

                        buy_back_price: token_buy_back_price.clone(),

                        shares: Vault::new(XRD),

                        ann_tokens: HashMap::new(),

                        // Initialize zero_coupon_bond as None
                        ann_token: HashMap::new(),

                        contributors: HashMap::new(),

                        proposal_creation_right: ProposalCreationRight::TOKEN_HOLDER_THRESHOLD(
                            threshold,
                        ),

                        liquidated_collateral: Vault::new(XRD),
                    }
                    .instantiate()
                    .prepare_to_globalize(OwnerRole::Fixed(rule!(require(
                        owner_token_addresss.clone()
                    ))))
                    .with_address(address_reservation.clone())
                    .globalize();

                    let component_address = component.address();

                    let event_metadata = TokenWightedDeployment {
                        component_address,

                        token_address: dao_token_address,

                        owner_token_address: owner_token_addresss,

                        community_name: organization_name,

                        community_image: org_ico_url,

                        token_price,

                        token_buy_back_price,

                        description,

                        total_token: token_supply,

                        token_image: power_token_url,

                        tags: tags.clone(),

                        purpose: purpose.clone(),

                        proposal_creation_right: ProposalCreationRight::TOKEN_HOLDER_THRESHOLD(
                            threshold,
                        ),
                    };

                    Runtime::emit_event(PandaoEvent {
                        // event_type: EventType::PROPOSAL_CREATION_RIGHT,
                        event_type: EventType::DEPLOYMENT,
                        dao_type: DaoType::Insurance,
                        component_address,
                        meta_data: DaoEvent::TokenWeightedDEployment(event_metadata),
                    });
                }
                ProposalCreationRight::ADMIN => {
                    component = Self {
                        token_price: token_price.clone(),

                        organization_name: organization_name.clone(),

                        dao_token_address: dao_token_address.clone(),

                        owner_token_addresss: owner_token_addresss.clone(),

                        current_praposals: HashMap::new(),

                        dao_token_resource_manager: voting_power_tokens.resource_manager(),

                        dao_token: Vault::with_bucket(voting_power_tokens),

                        buy_back_price: token_buy_back_price.clone(),

                        shares: Vault::new(XRD),

                        ann_tokens: HashMap::new(),

                        // Initialize zero_coupon_bond as None
                        ann_token: HashMap::new(),

                        contributors: HashMap::new(),

                        proposal_creation_right: ProposalCreationRight::ADMIN,

                        liquidated_collateral: Vault::new(XRD),
                    }
                    .instantiate()
                    .prepare_to_globalize(OwnerRole::Fixed(rule!(require(
                        owner_token_addresss.clone()
                    ))))
                    .with_address(address_reservation.clone())
                    .globalize();

                    let component_address = component.address();

                    let event_metadata = TokenWightedDeployment {
                        component_address,

                        token_address: dao_token_address,

                        owner_token_address: owner_token_addresss,

                        community_name: organization_name,

                        community_image: org_ico_url,

                        token_price,

                        token_buy_back_price,

                        description,

                        total_token: token_supply,

                        token_image: power_token_url,

                        tags: tags.clone(),

                        purpose: purpose.clone(),

                        proposal_creation_right: ProposalCreationRight::ADMIN,
                    };

                    Runtime::emit_event(PandaoEvent {
                        // event_type: EventType::PROPOSAL_CREATION_RIGHT,
                        event_type: EventType::DEPLOYMENT,
                        dao_type: DaoType::Insurance,
                        component_address,
                        meta_data: DaoEvent::TokenWeightedDEployment(event_metadata),
                    });
                }
            }

            (component, owner_badge)
        }

        pub fn obtain_community_token(
            &mut self,
            mut xrd: Bucket,
            token_amount: Decimal,
            // minter_address: Option<String>,
        ) -> (Bucket, Bucket) {
            assert!(
                (self.token_price * token_amount) <= xrd.amount(),
                "you are paying an insufficient amount"
            );

            let collected_xrd = xrd.take(self.token_price * token_amount);

            let power_share = self.dao_token.take(token_amount);

            let amount_paid = self.token_price * token_amount;

            self.shares.put(collected_xrd);

            //emit event

            let event_metadata = TokenWeightBuyToken {
                amount: token_amount,

                resource_address: self.dao_token_address,

                amount_paid,

                current_component_share: self.shares.amount(), // current component's collected xrd
            };

            let component_address = Runtime::global_address();

            Runtime::emit_event(PandaoEvent {
                event_type: EventType::TOKEN_BOUGHT,

                dao_type: DaoType::Insurance,

                component_address,

                meta_data: DaoEvent::TokenWeightedTokenPurchase(event_metadata),
            });

            (xrd, power_share)
        }

        pub fn withdraw_power(&mut self, voting_power: Bucket) -> Bucket {
            // put the voting power back
            assert!(
                !self.current_praposals.is_empty(),
                "token can not be sold when there are active praposals or incomplete proposals"
            );

            let power_amount = voting_power.amount();

            self.dao_token.put(voting_power);

            let event_metadata = TokenWeightBuyToken {
                amount: power_amount,

                resource_address: self.dao_token_address.clone(),

                amount_paid: power_amount * self.buy_back_price,

                current_component_share: self.shares.amount(),
            };

            let component_address = Runtime::global_address();

            Runtime::emit_event(PandaoEvent {
                event_type: EventType::TOKEN_SELL,
                dao_type: DaoType::Insurance,
                component_address,
                meta_data: DaoEvent::TokenWeightedTokenPurchase(event_metadata),
            });

            self.shares.take(power_amount * self.buy_back_price)
        }

        pub fn get_usd_price() -> Decimal {
            let usd_price = Runtime::get_usd_price();
            usd_price
        }

        //get_unique_random_number
        pub fn get_proposal_id() -> u64 {
            let current_epoch = Runtime::current_epoch();
            let unique_number: u64 = current_epoch.number();
            unique_number
        }

        pub fn create_praposal(
            &mut self,
            title: String,
            description: String,
            minimun_quorum: u8,
            start_time: scrypto::time::UtcDateTime,
            end_time: scrypto::time::UtcDateTime,
            address_issued_bonds_to_sell: Option<ComponentAddress>, // Address issued ANN token to sell
            target_xrd_amount: Option<Decimal>,
            proposal_creator_address: Option<ComponentAddress>,
            governance_token_or_owner_token_address: Bucket,
            voting_type: VotingType,
        ) -> (
            Global<crate::proposal::pandao_praposal::TokenWeightProposal>,
            String,
            Bucket,
        ) {
            //implement proposal creation rights
            match self.proposal_creation_right {
                ProposalCreationRight::EVERYONE => {
                    assert_eq!(
                        governance_token_or_owner_token_address.resource_address(),
                        self.dao_token_address,
                        "wrong voting token supplied! please make sure that you supply DAO Governance Token"
                    );

                    assert!(
                        governance_token_or_owner_token_address.amount() >= Decimal::one(),
                        "Proposal creator must have at least one governance token to create a proposal"
                    );

                    //allow proposal creation
                }
                ProposalCreationRight::TOKEN_HOLDER_THRESHOLD(threshold) => {
                    assert_eq!(
                        governance_token_or_owner_token_address.resource_address(),
                        self.dao_token_address,
                        "wrong voting token supplied! please make sure that you supply DAO Governance Token"
                    );

                    assert!(
                        governance_token_or_owner_token_address.amount() >= threshold,
                        "Proposal creator does not have enough tokens to meet the threshold"
                    );
                }
                ProposalCreationRight::ADMIN => {
                    assert_eq!(
                        governance_token_or_owner_token_address.resource_address(),
                        self.owner_token_addresss,
                        "Only the admin can create a proposal and If you are an Admin please make sure you pass OWNER TOKEN ADDRESS"
                    );

                    assert!(
                        governance_token_or_owner_token_address.amount() >= Decimal::one(),
                        "ADMIN must pass his/her OWNER TOKEN to create proposal"
                    );
                }
            }

            use crate::proposal::pandao_praposal::TokenWeightProposal;

            if let Some(address_selling_ann_tokens) = address_issued_bonds_to_sell {
                assert!(
                    self.ann_token.contains_key(&address_selling_ann_tokens),
                    "The Address you have specified has not created any ANN Token"
                );
            }

            let amount_of_tokens_should_be_minted = None;

            let global_proposal_component: Global<TokenWeightProposal>;

            match voting_type {
                VotingType::ResourceHold => {
                    (global_proposal_component, _) = TokenWeightProposal::new(
                        title.clone(),
                        description.clone(),
                        minimun_quorum,
                        start_time,
                        end_time,
                        self.owner_token_addresss.clone(),
                        self.dao_token_address.clone(),
                        address_issued_bonds_to_sell.clone(),
                        target_xrd_amount.clone(),
                        proposal_creator_address,
                        amount_of_tokens_should_be_minted,
                        VotingType::ResourceHold,
                    );
                }
                VotingType::Equality => {
                    (global_proposal_component, _) = TokenWeightProposal::new(
                        title.clone(),
                        description.clone(),
                        minimun_quorum,
                        start_time,
                        end_time,
                        self.owner_token_addresss.clone(),
                        self.dao_token_address.clone(),
                        address_issued_bonds_to_sell.clone(),
                        target_xrd_amount.clone(),
                        proposal_creator_address,
                        amount_of_tokens_should_be_minted,
                        VotingType::Equality,
                    );
                }
            }

            let start_time_ts: i64 = start_time.to_instant().seconds_since_unix_epoch;
            let end_time_ts: i64 = end_time.to_instant().seconds_since_unix_epoch;

            //unique-id-generation
            let proposal_id: usize = Self::get_proposal_id()
                .try_into()
                .expect("couldn't get called successfully");
            //populate HashMap with newly created proposal

            let inner_map = self
                .current_praposals
                .entry(proposal_creator_address.unwrap())
                .or_insert_with(HashMap::new);

            inner_map.insert(proposal_id, global_proposal_component);

            match voting_type {
                VotingType::ResourceHold => {
                    let praposal_metadata = PraposalMetadata {
                        title,
                        description,
                        minimum_quorum: minimun_quorum.into(),
                        end_time_ts,
                        start_time_ts,
                        owner_token_address: self.owner_token_addresss.clone(),
                        component_address: global_proposal_component.address(),
                        address_issued_bonds_to_sell,
                        target_xrd_amount,
                        proposal_creator_address,
                        amount_of_tokens_should_be_minted,
                        proposal_id,
                        governance_token_or_owner_token_address:
                            governance_token_or_owner_token_address.resource_address(),
                        token_type: VotingType::ResourceHold,
                    };

                    let component_address = Runtime::global_address();

                    Runtime::emit_event(PandaoEvent {
                        event_type: EventType::PRAPOSAL,
                        dao_type: DaoType::Insurance,
                        meta_data: DaoEvent::PraposalDeployment(praposal_metadata),
                        component_address,
                    });
                }
                VotingType::Equality => {
                    let praposal_metadata = PraposalMetadata {
                        title,
                        description,
                        minimum_quorum: minimun_quorum.into(),
                        end_time_ts,
                        start_time_ts,
                        owner_token_address: self.owner_token_addresss.clone(),
                        component_address: global_proposal_component.address(),
                        address_issued_bonds_to_sell,
                        target_xrd_amount,
                        proposal_creator_address,
                        amount_of_tokens_should_be_minted,
                        proposal_id,
                        governance_token_or_owner_token_address:
                            governance_token_or_owner_token_address.resource_address(),
                        token_type: VotingType::Equality,
                    };

                    let component_address = Runtime::global_address();

                    Runtime::emit_event(PandaoEvent {
                        event_type: EventType::PRAPOSAL,
                        dao_type: DaoType::Insurance,
                        meta_data: DaoEvent::PraposalDeployment(praposal_metadata),
                        component_address,
                    });
                }
            }

            let mut message = String::new();
            message = format!("Proposal created with id : {}", proposal_id);

            (
                global_proposal_component,
                message,
                governance_token_or_owner_token_address,
            )
        }

        pub fn execute_proposal(&mut self, proposal_id: usize) {
            // First, find the proposal
            let mut proposal_option = None;
            let mut bond_creator_address_option = None;
            let mut target_xrd_amount_option = None;

            for (_, inner_map) in &self.current_praposals {
                if let Some(proposal) = inner_map.get(&proposal_id) {
                    proposal_option = Some(proposal.clone());
                    bond_creator_address_option = Some(proposal.get_address_issued_bonds());
                    target_xrd_amount_option = Some(proposal.get_target_xrd_amount());
                    break;
                }
            }

            // If the proposal is found, execute it
            if let (Some(proposal), Some(bond_creator_address), Some(target_xrd_amount)) = (
                proposal_option,
                bond_creator_address_option,
                target_xrd_amount_option,
            ) {
                // let current_epoch = Runtime::current_epoch();
                // let current_time_seconds = current_epoch.number() as i64;

                let now: Instant = Clock::current_time_rounded_to_seconds();

                let current_time_seconds: i64 = now.seconds_since_unix_epoch;

                let last_time = proposal.get_last_time();

                let end_time_seconds = last_time.to_instant().seconds_since_unix_epoch;

                // Debug statements to verify the times
                println!("Current time (epoch seconds): {}", current_time_seconds);
                println!("Proposal end time (epoch seconds): {}", end_time_seconds);

                assert!(
                    current_time_seconds > end_time_seconds,
                    "Proposal can only be executed after the specified end time"
                );

                // Check if the minimum quorum is met
                let number_of_voters = proposal.get_number_of_voters();
                let minimum_quorum = proposal.get_minimum_quorum();
                // let minimum_quorum = min_quo.0.to_u64().expect("Invalid minimum quorum value") as usize;

                if number_of_voters < minimum_quorum {
                    // Emit an event indicating that the proposal cannot be executed due to insufficient participation
                    let event_metadata = ProposalQuorumNotMet {
                        proposal_id,
                        minimum_quorum: proposal.get_minimum_quorum(),
                        number_of_voters,
                    };

                    let component_address = Runtime::global_address();

                    Runtime::emit_event(PandaoEvent {
                        event_type: EventType::QUORUM_NOT_MET,
                        dao_type: DaoType::Insurance,
                        component_address,
                        meta_data: DaoEvent::ProposalQuorumNotMet(event_metadata),
                    });

                    panic!("The proposal cannot be executed due to insufficient participation");
                }

                // Check if the treasury has enough XRD
                let treasury_balance = self.shares.amount();

                assert!(
                    treasury_balance >= target_xrd_amount,
                    "Insufficient funds in the treasury to execute the proposal."
                );

                // Create a bucket with the exact XRD amount needed for the purchase
                let payment = self.shares.take(target_xrd_amount);

                // Call the purchase_bond function
                // let remaining = self.purchase_bond(bond_creator_address, payment);
                let remaining = self.purchase_ann_token(bond_creator_address, payment);

                // Handle remaining funds and received bond NFT
                self.shares.put(remaining);

                // let praposal_metadata = PraposalExecute {
                //     praposal_address: proposal.address(),
                //     proposal_id, // purchased_bond_address,
                //                  // purchased_amount: purchased_amt
                // };

                // let component_address = Runtime::global_address();

                // Runtime::emit_event(PandaoEvent {
                //     event_type: EventType::EXECUTE_PROPOSAL,
                //     dao_type: DaoType::Insurance,
                //     meta_data: DaoEvent::ProposalExecute(praposal_metadata),
                //     component_address,
                // });

                let event_metadata = ProposalQuorumMet {
                    proposal_id,
                    minimum_quorum: proposal.get_minimum_quorum(),
                    number_of_voters,
                };

                let component_address = Runtime::global_address();

                Runtime::emit_event(PandaoEvent {
                    event_type: EventType::QUORUM_MET,
                    dao_type: DaoType::Insurance,
                    component_address,
                    meta_data: DaoEvent::ProposalQuorumMet(event_metadata),
                });

                // Optionally, you can remove the executed proposal from the current proposals
                for (_, inner_map) in &mut self.current_praposals {
                    inner_map.remove(&proposal_id);
                }
            } else {
                panic!("there is no current active proposal with the given ID");
            }
        }

        pub fn vote(
            &mut self,
            token: Bucket,
            againsts: bool,
            account: Global<Account>,
            proposal_id: usize,
        ) -> Bucket {
            let owner_role_of_voter = account.get_owner_role();
            Runtime::assert_access_rule(owner_role_of_voter.rule);

            // Find the proposal by proposal_id
            let mut proposal_option = None;
            for (_, inner_map) in &self.current_praposals {
                if let Some(proposal) = inner_map.get(&proposal_id) {
                    proposal_option = Some(proposal.clone());
                    break;
                }
            }

            if let Some(proposal) = proposal_option {
                assert_eq!(
                    token.resource_address(),
                    self.dao_token_address,
                    "wrong voting token supplied"
                );

                // Get the voter address from the account
                let voter_address = account.address();

                let mut vote_caster_addresses = proposal.get_vote_caster_addresses();

                // Check if the voter has already voted
                assert!(
                    !vote_caster_addresses.contains(&voter_address),
                    "You have already voted on this proposal."
                );

                let amount = token.amount();

                let event_metadata = ProposalVote {
                    praposal_address: proposal.address(),
                    voting_amount: amount,
                    againts: againsts,
                    voter_address,
                    proposal_id,
                };

                Runtime::emit_event(PandaoEvent {
                    event_type: EventType::VOTE,
                    dao_type: DaoType::Insurance,
                    component_address: Runtime::global_address(),
                    meta_data: DaoEvent::PraposalVote(event_metadata),
                });

                let result = proposal.vote(token, againsts);

                // Mark this voter as having voted
                proposal.set_vote_caster_address(voter_address);

                result
            } else {
                assert!(false, "no active proposal with the given ID");
                panic!();
            }
        }

        // ISSUE ANN TOKEN
        pub fn issue_ann_token(
            &mut self,
            contract_type: String,
            contract_role: String,
            contract_identifier: String,
            nominal_interest_rate: Decimal,
            currency: String,
            initial_exchange_date: u64,
            maturity_date: u64,
            notional_principal: Decimal,
            annuity_position: String,
            price: Decimal,
            number_of_annuities_to_mint: Decimal,
            your_address: ComponentAddress,
            nft_as_a_collateral: Bucket,
        ) -> Global<Annuity> {
            // assert!(
            //    !self.ann_token.contains_key(&your_address),
            //     "This address has already created an ANN token and you cannot create another."
            // );

            let ann_token_component = Annuity::instantiate_annuity(
                contract_type.clone(),
                contract_role.clone(),
                contract_identifier.clone(),
                nominal_interest_rate,
                currency.clone(),
                initial_exchange_date.clone(),
                maturity_date.clone(),
                notional_principal.clone(),
                annuity_position.clone(),
                price.clone(),
                number_of_annuities_to_mint.clone(),
                nft_as_a_collateral,
            );

            self.ann_token
                .entry(your_address)
                .or_insert_with(Vec::new)
                .push(ann_token_component);

            let event_metadata = AnnuityTokenCreation {
                component_address: ann_token_component.address(),
                contract_type,
                contract_role,
                contract_identifier,
                nominal_interest_rate,
                currency,
                initial_exchange_date,
                maturity_date,
                notional_principal,
                annuity_position,
                price,
                number_of_annuities_to_mint,
                your_address,
            };

            Runtime::emit_event(PandaoEvent {
                event_type: EventType::ANN_TOKEN_CREATION,
                dao_type: DaoType::Insurance,
                component_address: Runtime::global_address(),
                meta_data: DaoEvent::AnnTokenCreation(event_metadata),
            });

            ann_token_component
        }

        pub fn update_ann_vault_and_store(&mut self, desired_ann_token: Bucket) {
            let desired_resource_address: ResourceAddress = desired_ann_token.resource_address();

            if !self.ann_tokens.contains_key(&desired_resource_address) {
                self.ann_tokens.insert(
                    desired_resource_address,
                    Vault::new(desired_resource_address),
                );
            }
            let vault = self.ann_tokens.get_mut(&desired_resource_address).unwrap();
            vault.put(desired_ann_token);
        }

        // PURCHASE ANN TOKEN
        pub fn purchase_ann_token(
            &mut self,
            ann_token_creator_address: ComponentAddress,
            payment: Bucket,
        ) -> Bucket {
            assert!(
                self.ann_token.contains_key(&ann_token_creator_address),
                "No ANN Token created by the specified address."
            );
            let ann_token_components = self.ann_token.get_mut(&ann_token_creator_address).unwrap();
            let latest_ann_component = ann_token_components
                .last_mut()
                .expect("No bond component found");
            let (purchased_ann_token, payment) = latest_ann_component.purchase_annuity(payment);
            self.update_ann_vault_and_store(purchased_ann_token);
            payment
        }

        // Function to retrieve bond creators and their bond component addresses
        pub fn get_bond_creators(&self) -> HashMap<ComponentAddress, Vec<Global<Annuity>>> {
            self.ann_token.clone() // Return the HashMap of bond creators and their bonds
        }

        // New function to get all bond creator addresses
        pub fn get_bond_creator_addresses(&self) -> Vec<ComponentAddress> {
            self.ann_token.keys().cloned().collect() // Return a list of bond creator addresses
        }

        pub fn send_money_to_dao_treasury(
            &mut self,
            payment: Bucket,
            account: Global<Account>,
        ) -> Bucket {
            // Ensure the payment is in XRD
            assert_eq!(
                payment.resource_address(),
                XRD,
                "Only XRD tokens are accepted for treasury contributions"
            );

            // Get the amount being sent
            let amount = payment.amount();

            // Get the sender's address
            let sender_address = account.address();

            // Store the payment in the dao_token vault
            self.dao_token.put(payment);

            // Update the contributor's record
            self.update_contributor_record(sender_address, amount);

            // Emit an event for the contribution
            self.emit_contribution_event(sender_address, amount);

            // Return an empty bucket
            Bucket::new(XRD)
        }

        // Helper method to update the contributor's record
        fn update_contributor_record(&mut self, address: ComponentAddress, amount: Decimal) {
            *self.contributors.entry(address).or_insert(Decimal::zero()) += amount;
        }

        // Helper method to emit a contribution event
        fn emit_contribution_event(&self, address: ComponentAddress, amount: Decimal) {
            let event_metadata = TreasuryContribution {
                contributor: address,
                amount: amount,
                timestamp: Runtime::current_epoch().number(),
            };

            Runtime::emit_event(PandaoEvent {
                event_type: EventType::TREASURY_CONTRIBUTION,
                dao_type: DaoType::Insurance,
                component_address: Runtime::global_address(),
                meta_data: DaoEvent::TreasuryContribution(event_metadata),
            });
        }

        // Method to get all contributors and their total contributions
        pub fn get_all_contributors(&self) -> HashMap<ComponentAddress, Decimal> {
            self.contributors.clone()
        }

        // pub fn execute_proposal_for_pandao(&mut self){
        //     match self.current_praposal{
        //         Some(current_proposal) =>{
        //             //earlier execute proposal was not taking any action but it was made to take action in financial dao case
        //             println!("your proposal is executed successfully");
        //             self.current_praposal = None;
        //         },
        //         // None => println!("there is not any proposal created")
        //         None => assert!(false, "there is no any created proposal")
        //     }
        // }

        // pub fn create_proposal_to_mint_more_dao_tokens(
        //     &mut self,
        //     title: String,
        //     description: String,
        //     minimun_quorum: u8,
        //     amount_of_tokens_should_be_minted: Option<usize>,
        //     start_time: scrypto::time::UtcDateTime,
        //     end_time: scrypto::time::UtcDateTime,
        //     proposal_creator_address: Option<ComponentAddress>,
        //     governance_token_or_owner_token_address: Bucket,
        //     voting_type: VotingType,
        // ) -> (
        //     Global<crate::proposal::pandao_praposal::TokenWeightProposal>,
        //     String,
        //     Bucket,
        // ) {
        //     match self.proposal_creation_right {
        //         ProposalCreationRight::EVERYONE => {
        //             assert_eq!(
        //                 governance_token_or_owner_token_address.resource_address(),
        //                 self.dao_token_address,
        //                 "wrong voting token supplied! please make sure that you supply DAO Governance Token"
        //             );

        //             assert!(
        //                 governance_token_or_owner_token_address.amount() >= Decimal::one(),
        //                 "Proposal creator must have at least one governance token to create a proposal"
        //             );

        //             //allow proposal creation
        //         }
        //         ProposalCreationRight::TOKEN_HOLDER_THRESHOLD(threshold) => {
        //             assert_eq!(
        //                 governance_token_or_owner_token_address.resource_address(),
        //                 self.dao_token_address,
        //                 "wrong voting token supplied! please make sure that you supply DAO Governance Token"
        //             );

        //             assert!(
        //                 governance_token_or_owner_token_address.amount() >= threshold,
        //                 "Proposal creator does not have enough tokens to meet the threshold"
        //             );
        //         }
        //         ProposalCreationRight::ADMIN => {
        //             assert_eq!(
        //                 governance_token_or_owner_token_address.resource_address(),
        //                 self.owner_token_addresss,
        //                 "Only the admin can create a proposal and If you are an Admin please make sure you pass OWNER TOKEN ADDRESS"
        //             );

        //             assert!(
        //                 governance_token_or_owner_token_address.amount() >= Decimal::one(),
        //                 "ADMIN must pass his/her OWNER TOKEN to create proposal"
        //             );
        //         }
        //     }

        //     let address_issued_bonds_to_sell = None;
        //     let target_xrd_amount = None;

        //     let (global_proposal_component, _) = TokenWeightProposal::new(
        //         title.clone(),
        //         description.clone(),
        //         minimun_quorum,
        //         start_time,
        //         end_time,
        //         self.owner_token_addresss.clone(),
        //         self.dao_token_address.clone(),
        //         address_issued_bonds_to_sell.clone(),
        //         target_xrd_amount.clone(),
        //         proposal_creator_address,
        //         amount_of_tokens_should_be_minted,
        //         voting_type,
        //     );

        //     let proposal_id: usize = Self::get_proposal_id()
        //         .try_into()
        //         .expect("couldn't get called successfully");

        //     let inner_map = self
        //         .current_praposals
        //         .entry(proposal_creator_address.unwrap())
        //         .or_insert_with(HashMap::new);

        //     inner_map.insert(proposal_id, global_proposal_component);

        //     let start_time_s = start_time.to_instant().seconds_since_unix_epoch;
        //     let end_time_s = end_time.to_instant().seconds_since_unix_epoch;

        //     let proposal_metadata = PraposalMetadata {
        //         title,
        //         description,
        //         minimum_quorum: minimun_quorum.into(),
        //         start_time_ts: start_time_s,
        //         end_time_ts: end_time_s,
        //         owner_token_address: self.owner_token_addresss.clone(),
        //         component_address: global_proposal_component.address(),
        //         address_issued_bonds_to_sell,
        //         target_xrd_amount,
        //         proposal_creator_address,
        //         amount_of_tokens_should_be_minted,
        //         proposal_id,
        //         governance_token_or_owner_token_address: governance_token_or_owner_token_address
        //             .resource_address(),
        //     };

        //     let component_address = Runtime::global_address();

        //     Runtime::emit_event(PandaoEvent {
        //         event_type: EventType::PRAPOSAL,
        //         dao_type: DaoType::Insurance,
        //         meta_data: DaoEvent::PraposalDeployment(proposal_metadata),
        //         component_address,
        //     });

        //     let mut message = String::new();
        //     message = format!("Proposal created with id : {}", proposal_id);

        //     (
        //         global_proposal_component,
        //         message,
        //         governance_token_or_owner_token_address,
        //     )
        // }

        pub fn mint_more_tokens(&mut self, token_number_to_mint: usize) {
            self.dao_token
                .put(self.dao_token_resource_manager.mint(token_number_to_mint));
        }

        pub fn execute_proposal_to_mint_more_tokens(
            &mut self,
            proposal_id: usize,
        ) -> Result<String, String> {
            for (_, inner_map) in &self.current_praposals {
                let proposal = inner_map.get(&proposal_id);

                let now: Instant = Clock::current_time_rounded_to_seconds();
                let current_time_seconds: i64 = now.seconds_since_unix_epoch;

                let last_time = proposal.unwrap().get_last_time();
                let end_time_seconds = last_time.to_instant().seconds_since_unix_epoch;

                // Debug statements to verify the times
                println!("Current time (epoch seconds): {}", current_time_seconds);
                println!("Proposal end time (epoch seconds): {}", end_time_seconds);

                assert!(
                    current_time_seconds > end_time_seconds,
                    "Proposal can only be executed after the specified end time"
                );

                match proposal {
                    Some(proposal) => {
                        if let Some(how_much_amount) = proposal.get_token_mint_amount() {
                            &self.mint_more_tokens(how_much_amount);

                            let message = "proposal executed successfully".to_string();

                            return Ok(message);
                        } else {
                            return Err(format!("token_mint_amount is not present in a proposal with id : {proposal_id} and it seems not to be a proposal to mint more tokens"));
                        }
                    }
                    None => return Err(format!("proposal with id : {proposal_id} not found")),
                }
            }
            return Err(format!("proposal with id : {proposal_id} not found"));
        }

        pub fn check_time_until_next_payout(
            &self,
            ann_token_creator_address: ComponentAddress,
        ) -> Result<i64, String> {
            if let Some(ann_token_components) = self.ann_token.get(&ann_token_creator_address) {
                let latest_ann_component = ann_token_components
                    .last()
                    .ok_or_else(|| "No ANN token component found".to_string())?;

                Ok(latest_ann_component.check_time_until_next_payout())
            } else {
                Err("No ANN Token created by the specified address.".to_string())
            }
        }

        // pub fn claim_annual_payout(
        //     &mut self,
        //     ann_token_creator_address: ComponentAddress,
        //     annuity_token: Bucket,
        // ) -> Result<(Bucket, Bucket), String> {
        //     if let Some(ann_token_components) = self.ann_token.get_mut(&ann_token_creator_address) {
        //         let latest_ann_component = ann_token_components
        //             .last_mut()
        //             .ok_or_else(|| "No ANN token component found".to_string())?;

        //         let (annuity_token, payout) =
        //             latest_ann_component.claim_annual_payout(annuity_token);

        //         // self.update_ann_vault_and_store(annuity_token);

        //         Ok((annuity_token, payout))
        //     } else {
        //         Err("No ANN Token created by the specified address.".to_string())
        //     }
        // }

        
        pub fn claim_the_payout(
            &mut self,
            ann_token_creator_address: ComponentAddress
        ) -> Result<(), String> {
            assert!(
                self.ann_token.contains_key(&ann_token_creator_address),
                "No insurance tokens are created by the specified address"
            );

            if let Some(ann_token_components) = self.ann_token.get_mut(&ann_token_creator_address) {
                let latest_ann_component = ann_token_components
                    .last_mut()
                    .ok_or_else(|| "No ANN token component found".to_string())?;

                //how much XRDs are required by the community as an annual payout?
                //this is being taken care inside a function

                //make sure to have ANN token creation per address; removed
                //let payment = self.shares.take(target_xrd_amount);

                //token resource_address
                let ann_resource_address = latest_ann_component.get_annuity_address();

                let mut vault = self.ann_tokens.get_mut(&ann_resource_address).unwrap(); //was mutable earlier

                let annuity_token_to_showcase = vault.take(1);

                let (ann_token_in_return, payout_or_collateral, liquidated, premature_claim) =
                    latest_ann_component.claim_annual_payout(annuity_token_to_showcase);

                // let mut vault_to_give_back_ann = self.ann_tokens.get_mut(&r_a).unwrap();
                vault.put(ann_token_in_return);

                if liquidated == true {
                    self.liquidated_collateral =
                        Vault::new(payout_or_collateral.resource_address());
                    self.liquidated_collateral.put(payout_or_collateral);
                } else {
                    if premature_claim == false {
                        self.shares.put(payout_or_collateral);
                    } else {
                        println!("there is a empty bucket to add");
                    }
                }
                Ok(())
            } else {
                Err("No ANN Token created by the specified address.".to_string())
            }
        }

        //FOR BOND ISSUER TO TAKE OUT COMMUNITY INVESTMENT
        pub fn take_out_the_invested_XRDs_by_the_community(
            &mut self,
            ann_creator_address: ComponentAddress,
        ) -> Bucket {
            assert!(
                self.ann_token.contains_key(&ann_creator_address),
                "No bonds created by the specified address."
            );

            // Retrieve the most recent bond component created by the bond creator
            let ann_components = self.ann_token.get_mut(&ann_creator_address).unwrap();

            let latest_ann_component = ann_components.last_mut().expect("No bond component found");

            // let bond_creator_money_taken_status = latest_bond_component.bond_creator_money_status();

            let taken_out_invested_amount =
                latest_ann_component.take_out_the_invested_xrds_by_community();

            let event_metadata = TakenOutInvestedXRD {
                ann_creator_address,
                taken_out_amount: taken_out_invested_amount.amount(),
            };

            Runtime::emit_event(PandaoEvent {
                event_type: EventType::TAKEN_OUT_INVESTED_XRDS,
                dao_type: DaoType::Investment,
                component_address: Runtime::global_address(),
                meta_data: DaoEvent::TakenOutInvestedXRD(event_metadata),
            });

            taken_out_invested_amount
        }

        pub fn put_in_money_plus_interest_for_the_community_to_redeem(
            &mut self,
            ann_creator_address: ComponentAddress,
            borrowed_xrd_with_interest: Bucket,
        ) -> (Bucket, Bucket) {
            assert!(
                self.ann_token.contains_key(&ann_creator_address),
                "No ANN created by the specified address"
            );

            // Retrieve the most recent bond component created by the bond creator
            let bond_components = self.ann_token.get_mut(&ann_creator_address).unwrap();

            let latest_bond_component = bond_components.last_mut().expect("No ANN component found");

            let amount_getting_deposited = borrowed_xrd_with_interest.amount();

            //GET REQUIRED AMOUNT
            //CASE 1: DEPOSIT CAN BE NOTIONAL_PRINCIPAL + INTEREST
            let required_total_amount = latest_bond_component.total_amount_to_payback();
            //CASE 2: DEPOSIT CAN BE ANNUAL + ANNUAL_INTEREST
            let required_annual_amount = latest_bond_component.annual_amount_to_payback();

            let extra_money_if_any = latest_bond_component
                .put_in_money_plus_interest_for_the_community_to_redeem(borrowed_xrd_with_interest);

            let balance_of_ann_component =
                latest_bond_component.check_the_balance_of_ann_contract();

            //Jdo sara paisa with interest wapis ho gya odo baad appa collateral wapis krna hai

            let remaining_amount_to_be_deposited =
                latest_bond_component.remaining_amount_to_be_deposited();

            if balance_of_ann_component >= required_total_amount {
                //GIVE BACK THE COLLATERAL
                let collateral_being_taken_back = latest_bond_component.get_back_the_collateral();
                // let extra_money_amount = extra_money_if_any.amount();

                let event_metadata = PutInMoneyPlusInterestEvent {
                    ann_creator_address,
                    amount_getting_deposited,
                    yearly_amount_required_by_the_community: required_annual_amount,
                    total_amount_required_by_the_community: required_total_amount,
                    amount_sent_for_a_community_to_redeem: amount_getting_deposited,
                    extra_amount_given_back_to_the_sender: extra_money_if_any.amount(),
                    more_xrd_amount_required_by_the_community: remaining_amount_to_be_deposited,
                    collateral_given_back: true,
                };

                Runtime::emit_event(PandaoEvent {
                    event_type: EventType::PUT_IN_MONEY_PLUS_INTEREST,
                    dao_type: DaoType::Insurance,
                    component_address: Runtime::global_address(),
                    meta_data: DaoEvent::PutInMoneyPlusInterest(event_metadata),
                });
                
                return (extra_money_if_any, collateral_being_taken_back);
            } else if balance_of_ann_component >= required_annual_amount {
                //DO NOT GIVE BACK THE COLLATERAL
                //DO NOT GIVE BACK EXTRA AMOUNT (managed from ANN contract)
                let empty_collateral = Bucket::new(XRD);

                let event_metadata = PutInMoneyPlusInterestEvent {
                    ann_creator_address,
                    amount_getting_deposited,
                    yearly_amount_required_by_the_community: required_annual_amount,
                    total_amount_required_by_the_community: required_total_amount,
                    amount_sent_for_a_community_to_redeem: amount_getting_deposited,
                    extra_amount_given_back_to_the_sender: extra_money_if_any.amount(),
                    more_xrd_amount_required_by_the_community: remaining_amount_to_be_deposited,
                    collateral_given_back: false,
                };

                Runtime::emit_event(PandaoEvent {
                    event_type: EventType::PUT_IN_MONEY_PLUS_INTEREST,
                    dao_type: DaoType::Insurance,
                    component_address: Runtime::global_address(),
                    meta_data: DaoEvent::PutInMoneyPlusInterest(event_metadata),
                });

                return (extra_money_if_any, empty_collateral); //empty_buckets returned
            } else {
                let empty_collateral = Bucket::new(XRD);

                let event_metadata = PutInMoneyPlusInterestEvent {
                    ann_creator_address,
                    amount_getting_deposited,
                    yearly_amount_required_by_the_community: required_annual_amount,
                    total_amount_required_by_the_community: required_total_amount,
                    amount_sent_for_a_community_to_redeem: amount_getting_deposited,
                    extra_amount_given_back_to_the_sender: extra_money_if_any.amount(),
                    more_xrd_amount_required_by_the_community: remaining_amount_to_be_deposited,
                    collateral_given_back: false,
                };

                Runtime::emit_event(PandaoEvent {
                    event_type: EventType::PUT_IN_MONEY_PLUS_INTEREST,
                    dao_type: DaoType::Insurance,
                    component_address: Runtime::global_address(),
                    meta_data: DaoEvent::PutInMoneyPlusInterest(event_metadata),
                });
                
                return (extra_money_if_any, empty_collateral); //empty_buckets returned
            }

            // if balance_of_ann_component >= required_annual_amount {
            //     //DO NOT GIVE BACK THE COLLATERAL
            //     //DO NOT GIVE BACK EXTRA AMOUNT
            //     let collateral_being_taken_back = latest_bond_component.get_back_the_collateral();

            //     let extra_money_amount = extra_money_if_any.amount();

            //     // let required_now = latest_bond_component.balance_required_by_the_community();

            //     let event_metadata_if = PutInMoneyPlusInterestEvent {
            //         bond_creator_address,
            //         amount_getting_deposited,
            //         amount_required_by_the_community: required_amount,
            //         amount_taken_by_the_community: required_amount,
            //         extra_amount_given_back_to_the_sender: extra_money_amount,
            //         more_xrd_amount_required_by_the_community: Decimal::zero(),
            //         collateral_given_back: true,
            //     };

            //     Runtime::emit_event(PandaoEvent {
            //         event_type: EventType::PUT_IN_MONEY_PLUS_INTEREST,
            //         dao_type: DaoType::Investment,
            //         component_address: Runtime::global_address(),
            //         meta_data: DaoEvent::PutInMoneyPlusInterest(event_metadata_if),
            //     });

            //     (extra_money_if_any, collateral_being_taken_back)
            // } else {
            //     let extra_money_amount = extra_money.amount();

            //     //event emission

            //     if amount_getting_deposited >= required_amount {
            //         let event_metadata_if = PutInMoneyPlusInterestEvent {
            //             bond_creator_address,
            //             amount_getting_deposited,
            //             amount_required_by_the_community: required_amount,
            //             amount_taken_by_the_community: required_amount,
            //             extra_amount_given_back_to_the_sender: extra_money_amount,
            //             more_xrd_amount_required_by_the_community: Decimal::zero(),
            //             collateral_given_back: false,
            //         };

            //         Runtime::emit_event(PandaoEvent {
            //             event_type: EventType::PUT_IN_MONEY_PLUS_INTEREST,
            //             dao_type: DaoType::Investment,
            //             component_address: Runtime::global_address(),
            //             meta_data: DaoEvent::PutInMoneyPlusInterest(event_metadata_if),
            //         });
            //     } else {
            //         let more_xrd_amount_required_by_the_community =
            //             required_amount - amount_getting_deposited;

            //         let event_metadata_else = PutInMoneyPlusInterestEvent {
            //             bond_creator_address,
            //             amount_getting_deposited,
            //             amount_required_by_the_community: required_amount,
            //             amount_taken_by_the_community: amount_getting_deposited,
            //             extra_amount_given_back_to_the_sender: extra_money_amount,
            //             more_xrd_amount_required_by_the_community,
            //             collateral_given_back: false,
            //         };

            //         Runtime::emit_event(PandaoEvent {
            //             event_type: EventType::PUT_IN_LESS_MONEY_PLUS_INTEREST,
            //             dao_type: DaoType::Investment,
            //             component_address: Runtime::global_address(),
            //             meta_data: DaoEvent::PutInMoneyPlusInterest(event_metadata_else),
            //         });
            //     }

            //     let collateral_resource_address =
            //         latest_bond_component.get_resource_address_of_collateral();

            //     let empty_bucket = Bucket::new(collateral_resource_address);

            //     (extra_money, empty_bucket)
            // }
        }
    }
}

//*initialize
// resim call-function package_sim1p4nk9h5kw2mcmwn5u2xcmlmwap8j6dzet7w7zztzz55p70rgqs4vag TokenWeigtedDao initiate "Panjab Investment DAO" 100 0 5 2 "https://pbs.twimg.com/profile_images/1643159245389713408/47gnTbms_200x200.jpg" "https://pbs.twimg.com/profile_images/1548373397289455616/OFhGnboY_400x400.jpg" "This is a DAO for managing community projects"
// resim call-function package_sim1pk3cmat8st4ja2ms8mjqy2e9ptk8y6cx40v4qnfrkgnxcp2krkpr92 TokenWeigtedDao initiate "Panjab Investment DAO" 100 0 5 2 "https://pbs.twimg.com/profile_images/1643159245389713408/47gnTbms_200x200.jpg" "https://pbs.twimg.com/profile_images/1548373397289455616/OFhGnboY_400x400.jpg" "This is a DAO for managing community projects" --manifest instantiate_pandao.rtm

// account_sim1c956qr3kxlgypxwst89j9yf24tjc7zxd4up38x37zr6q4jxdx9rhma
// component_sim1czwnyl3pfn955s45a2js64w8zjlptwz4y3w4wwwl944rk2l2ceapsc
//

//*obtain_token
// resim call-method component_sim1czwnyl3pfn955s45a2js64w8zjlptwz4y3w4wwwl944rk2l2ceapsc obtain_token resource_sim1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxakj8n3:5 1
// resim call-method component_sim1cpwu4wc6rg0am8l9prnh2lzqkk6hue6stzqhdx48rzvek2mmm5vp0p obtain_community_token resource_sim1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxakj8n3:5 1

//*create_zero_coupon_bonds
// resim call-method component_sim1cpwu4wc6rg0am8l9prnh2lzqkk6hue6stzqhdx48rzvek2mmm5vp0p create_zero_coupon_bond "Corporate Bond" "Issuer" "Contract ID 123" 0.05 "USD" 1694774400 1695052800 1000000 5 "Secondary Market" 105 100

//*purchase_a_bond
// resim call-method component_sim1cpwu4wc6rg0am8l9prnh2lzqkk6hue6stzqhdx48rzvek2mmm5vp0p purchase_bond resource_sim1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxakj8n3:105

//*sell_a_bond
// resim call-method component_sim1cpwu4wc6rg0am8l9prnh2lzqkk6hue6stzqhdx48rzvek2mmm5vp0p sell_bond resource_sim1tklvuzvc60lvdc2dmrszpa20n2tu3vw839x97gtq6ezvx2qu04k5yz:1

//*check_bond_maturity
// resim call-method component_sim1cpwu4wc6rg0am8l9prnh2lzqkk6hue6stzqhdx48rzvek2mmm5vp0p check_bond_maturity

//*get_bond_details
// resim call-method component_sim1cpwu4wc6rg0am8l9prnh2lzqkk6hue6stzqhdx48rzvek2mmm5vp0p get_bond_details

// create_proposal
// resim call-method component_sim1czwnyl3pfn955s45a2js64w8zjlptwz4y3w4wwwl944rk2l2ceapsc create_praposal "Panda Fridays" "Introduce a fun Panda-themed event every Friday." 10 1694774400 1695052800
// resim call-method 02012345 create_praposal "Panda Fridays" "Introduce a fun Panda-themed event every Friday." 10 1694774400 1695052800
// resim call-method component_sim1czwnyl3pfn955s45a2js64w8zjlptwz4y3w4wwwl944rk2l2ceapsc create_praposal "Proposal Title" "Description" 10 {"year":2024,"month":9,"day_of_month":15,"hour":0,"minute":0} {"year":2024,"month":9,"day_of_month":20,"hour":23,"minute":59}

//*test-net
// txid_tdx_2_1uyf8cmuvgd0kredvzg2fh4mfff79zaj5t6trmnwkaet3n36ahkcq6dcwgq
// package_tdx_2_1phtjjxh563e37wtp008re5zlpau7v7y8xudpy9mmw4cp22k56frszt
// component_tdx_2_1czddjhay2jv0e03h78mapw2k3y8mnqmn47hxyz7svm4tm76wf8azmq
// account_tdx_2_1285pq36tg53usvdhvwjlu40plmzf6dj8uyhrqxp6j0kvpl2znqtt54
// resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc
//*owner badge
// resource_tdx_2_1tkam4tmrj2xl4hry7gsvpx0sq56xljudckmxxhr72tehqj4mq3rzna

//*community native token
//resource_tdx_2_1thp48upl275dm4ar0675we2ew83fn04k3cg7ca57swlzumctk4xvgc

//*manifest to call obtain_token*/
// CALL_METHOD
//             Address("account_tdx_2_1285pq36tg53usvdhvwjlu40plmzf6dj8uyhrqxp6j0kvpl2znqtt54")
//             "withdraw"
//             Address("resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc")
//             Decimal("5")
//         ;

// TAKE_FROM_WORKTOP
//             Address("resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc")
//             Decimal("5")
//             Bucket("bucket1")
//         ;

// CALL_METHOD
//         Address("component_tdx_2_1czddjhay2jv0e03h78mapw2k3y8mnqmn47hxyz7svm4tm76wf8azmq")
//         "obtain_token"
//         Bucket("bucket1")
//         Decimal("1")
//         ;

// CALL_METHOD
//             Address("account_tdx_2_1285pq36tg53usvdhvwjlu40plmzf6dj8uyhrqxp6j0kvpl2znqtt54")
//             "deposit_batch"
//             Expression("ENTIRE_WORKTOP")
//         ;

//*create_propoosal
// CALL_METHOD
// Address("component_tdx_2_1czddjhay2jv0e03h78mapw2k3y8mnqmn47hxyz7svm4tm76wf8azmq")
// "create_praposal"
// "should we purchase laptop or not"
// "abc"
// 6u8
// Tuple(
// 2024u32 ,
// 9u8 ,
// 9u8 ,
// 20u8 ,
// 22u8 ,
// 22u8)
// Tuple(
// 2024u32 ,
// 9u8 ,
// 12u8 ,
// 1u8 ,
// 1u8 ,
// 1u8)
// ;

//*cast_a_vote

// CALL_METHOD
//             Address("account_tdx_2_1285pq36tg53usvdhvwjlu40plmzf6dj8uyhrqxp6j0kvpl2znqtt54")
//             "withdraw"
//             Address("resource_tdx_2_1thp48upl275dm4ar0675we2ew83fn04k3cg7ca57swlzumctk4xvgc") // community token
//             Decimal("1")
//         ;

// TAKE_FROM_WORKTOP
//             Address("resource_tdx_2_1thp48upl275dm4ar0675we2ew83fn04k3cg7ca57swlzumctk4xvgc")
//             Decimal("1")
//             Bucket("bucket2")
//         ;

// CALL_METHOD
//         Address("component_tdx_2_1czddjhay2jv0e03h78mapw2k3y8mnqmn47hxyz7svm4tm76wf8azmq")
//         "vote"
//         Bucket("bucket2")
//         true
//         ;

// CALL_METHOD
//             Address("account_tdx_2_1285pq36tg53usvdhvwjlu40plmzf6dj8uyhrqxp6j0kvpl2znqtt54")
//             "deposit_batch"
//             Expression("ENTIRE_WORKTOP")
//         ;

// CALL_METHOD
//             Address("component_tdx_2_1czddjhay2jv0e03h78mapw2k3y8mnqmn47hxyz7svm4tm76wf8azmq")
//             "execute_proposal"
//         ;

//* vote is being casted multiple times
//* execute proposal and try to check proposal creation by an account other that community creator
//* (I BELIEVE COMMUNITY CREATOR IS RESPONSIBLE FOR PROPOSAL CREATION)

//* TEST-CASES:
//* CAN ANY COMMUNITY MEMBER CREATE A PROPOSAL ?    (OR ONLY COMMUNITY CREATOR WILL CREATE)
//  yes! any member can create a proposal

//* CAN ANY COMMUNITY MEMBER EXECUTE THE PROPOSAL ? (OR ONLY PROPOSAL CREATOR WILL EXECUTE)
//* DO WE REALLY NEED TO HAVE A COMMUNITY TOKEN FOR PROPOSAL CREATION?*/
//*hustlepreet secondry account
// account_tdx_2_128e6fmjkhjqx0n8h9562rrvstl883wq22pzea4ucnnx0762ptlch4s

//*missing functions
// become_a_dao_member
