//! Monetary Futarchy
//! - at the moment, just continuous projections of expected spending among council members
#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "128"]
use parity_scale_codec::{Decode, Encode};
#[cfg(feature = "std")]
use runtime_primitives::traits::Zero;
use support::traits::{Currency, ReservableCurrency, Get};
use support::{
    decl_event, decl_module, decl_storage, dispatch::Result, ensure, StorageMap, StorageValue
};
use system::ensure_signed;

type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;
type DotPoints = Vec<(BalanceOf, BlockNumber>; // invariant: length == `TotalTerm`s

// temporary structure for encoding changes to spending parameterizations
// TODO: add Guillame's fixed issuance curve and make it adjustable
#[derive(Encode, Decode, Clone, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Projection<DotPoints> {
    // projections for total reward 
    total_reward: DotPoints,
    // dilution to treasury on block author reward
    treasury_dilute_on_block_author:  DotPoints,
    // block author reward
    block_author_reward: DotPoints,
    // dilution to treasury on relay chain validator reward
    treasury_dilute_on_relay_reward: DotPoints,
    // relay chain validator reward
    validator_relay_reward: DotPoints,
    // dilution to treasury on parachain rewards
    treasury_dilute_on_parachain_reward: DotPoints,
    // parachain rewards (eventually to be highest)
    validator_parachain_rewards: DotPoints,
}

// instead of hard coding this use number of variants of projection (this is WORST practices)
const total_projection_fields: usize = 7;

pub trait Trait: system::Trait {
    // overarching event type
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

    /// The balances type
    type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;

    // how long each projection lasts
    type TermDuration: Get<Self::BlockNumber>;

    // how many terms in a projection
    type TotalTerms: Get<u32>;
}

decl_event!(
    pub enum Event<T>
    where 
        Balance = BalanceOf<T>,
        AccountId = <T as system::Trait>::AccountId,
        BlockNumber = <T as system::Trait>::BlockNumber,
    {   
        // proposer's AccountId, BlockNumber at expected execution
        DotPlotPoint(AccountId, Balance, BlockNumber),
        // Abstained from making a projection
        Abstained(AccountId, BlockNumber),
        // new projection period started
        NewProjectionsPeriod(BlockNumber),
    }
);

decl_storage! {
    trait Store for Module<T: Trait> as ForwardGuidance {
        /// council members
        MonetaryCouncil get(monetary_council): Vec<T::AccountId>;
        /// Current Period's Projection
        CurrentProjections get(current_projections): map T::AccountId => Option<Projection<BalanceOf<T>, T::BlockNumber>>;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        // frequency with which projections are made
        const ProjectionPeriod: T::BlockNumber = T::ProjectionPeriod::get();

        // permissionless to make testing straightforward 
        // TODO: use phragmenn in practice instead (see https://github.com/paritytech/substrate/pull/3364 -- support::ChangeMembers)
        fn join_council(origin) -> Result {
            let new_member = ensure_signed(origin)?;
            ensure!(!Self::is_on_council(&new_member), "is already a member");

            <MonetaryCouncil<T>>::mutate(|c| c.push(new_member.clone()));
            Ok(())
        }

        fn submit_dot_point(origin, projections: Vec<Projection>) -> Result {
            let council_member = ensure_signed(origin)?;
            ensure!(Self::is_on_council(&council_member), "not on the council");

            ensure!(projections.len() == total_projection_fields, "doesn't match the current projection format");
            let _ = projections.into_iter().for_each(|dp| {
                // build the Projection struct with some builder pattern here
                // emit a single event with that outrageous struct
            });
            Ok(())
        }

        fn on_finalize(n: T::BlockNumber) {
            if (n % T::ProjectionPeriod::get()).is_zero() {
                <MonetaryCouncil<T>>::get().into_iter().for_each(|mem| {
                    if let Some(pro) = <CurrentProjections<T>>::get(mem.clone()) {
                        Self::deposit_event(RawEvent::DotPlotPoint(mem.clone(), pro.spending_estimate, pro.time));
                        // remove all the projections `=>` should be stored off-chain via event emission
                        <CurrentProjections<T>>::remove(mem);
                    } else {
                        Self::deposit_event(RawEvent::Abstained(mem, n));
                    }
                });
                let new_period_starts = n + 1.into();
                Self::deposit_event(RawEvent::NewProjectionsPeriod(new_period_starts));
            }
        }
    }
}

impl<T: Trait> Module<T> {
    pub fn is_on_council(who: &T::AccountId) -> bool {
        Self::monetary_council().contains(who)
    }
}