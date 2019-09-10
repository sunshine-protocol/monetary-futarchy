//! Monetary Futarchy
//! - at the moment, just continuous projections of expected spending among council members
//! - use phragmen (support::ChangeMembers) for council selection
//! - experiment with pricing relevant proposals according to stdev from projections
//! - would prefer to use offchain workers for graphing projections in DOT plot and calculating proposal collateral
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

// Projection submitted by member
#[derive(Encode, Decode, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Projection<Balance, BlockNumber> {
    // spending estimate
    spending_estimate: Balance,
    // block number
    time: BlockNumber,
} // TODO: make it more like a schedule instead of one estimate for one period

pub trait Trait: system::Trait {
    // overarching event type
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

    /// The balances type
    type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;

    // how frequently proposals are passed from the dispatchQ
    type ProjectionPeriod: Get<Self::BlockNumber>;
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

        // permissionless to make initial testing straightforward 
        // TODO: use phragmen instead (see https://github.com/paritytech/substrate/pull/3364 -- support::ChangeMembers)
        fn join_council(origin) -> Result {
            let new_member = ensure_signed(origin)?;
            ensure!(!Self::is_on_council(&new_member), "is already a member");

            <MonetaryCouncil<T>>::mutate(|c| c.push(new_member.clone()));
            Ok(())
        }

        fn submit_dot_point(origin, spending_estimate: BalanceOf<T>) -> Result {
            let council_member = ensure_signed(origin)?;
            ensure!(Self::is_on_council(&council_member), "not on the council");

            // even if the projection already exists, there is no cost to change votes (so less bribery encouragement)
            // so we just overwrite the previous projections (can add more nuanced rep later...)
            let time = <system::Module<T>>::block_number();
            <CurrentProjections<T>>::insert(council_member, Projection { spending_estimate, time });
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