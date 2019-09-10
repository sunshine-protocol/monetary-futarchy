# Monetary Futarchy

**WARNING**: *nobody has verified that any these thoughts hold any merit; they are just dumb ideas until someone with more clout chooses to give them relevance :P*

Polkadot does not have a predefined issuance schedule by design. Once the network launches and achieves relative stability, it will be necessary to communicate clear expectations for the future issuance schedule to stakeholders. This module uses a designated committee to relay clear expectations of future changes akin to Central Bank [forward guidance](https://www.federalreserve.gov/faqs/what-is-forward-guidance-how-is-it-used-in-the-federal-reserve-monetary-policy.htm).

SEE THE [PROPOSAL](#PROPOSAL) FOR SPECIFIC DETAILS

## Understanding the Polkadot Treasury

The original purpose of the [treasury](https://www.federalreserve.gov/faqs/what-is-forward-guidance-how-is-it-used-in-the-federal-reserve-monetary-policy.htm) is to keep the proportion of tokens staked constant while minting liquid validator rewards. **Why?** The proportion of liquid supply determines how cheap/easy it is to overcome the BFT threshold (1/3) and attack the blockchain. *I don't really buy the argument that it aligns the token value with network security because PoW...* **How?** [`OnDilution`](https://github.com/paritytech/substrate/blob/master/srml/treasury/src/lib.rs#L339) runtime hook *mints extra funds for the treasury to keep the ratio of tokens_staked to total_issuance equal pre-dilution and post-dilution*".

So, the treasury exists to provide some control over the **`locked : liquid`** ratio. 
* Too much liquidity implies vulnerability to outside attacks, especially during the initial launch (at which point the total market cap will optimistically total a few billion USD, less than many nations and corporations)
* Too little liquidity will negatively impact price discovery `=>` more volatility (and potential price manipulation)

At the network's launch, the treasury will generously dilute rewards. For every block author reward, four times that amount is sent to the treasury. Eventually, this kind of aggressive dilution won't be necessary. *Who will determine when that is and how will it be coordinated and enforced through governance?*

**The purpose of this project is to raise awareness of the treasury's governance and it's significant influence on ecosystem development**. The treasury is fed by Polkadot's dilution mechanism. In practice, this mechanism extracts a silent tax on liquid DOTs that is redistributed to the treasury and validators/nominators. Value is not zero sum, but the [Cantillon Effect](https://www.aier.org/article/sound-money-project/cantillon-effects-and-money-neutrality) demonstrates that the first to receive new funds benefit disproportionately. 

If this isn't immediately clear, consider the following example. I give you five beads and say that all five beads are backed by $10. So you might reason that each bead is worth $2 (assuming there was a market lol). If I issue five more beads, but don't increase the amount that backs the beads, you would reason that each bead is worth $1. This is basically how inflation works, but it doesn't happen all at once. It happens slowly over time and prices adjust slowly over time. So the first to spend the new funds, are spending in a market that is treating each bead as if it is worth $2. Eventually, prices adjust and somewhere down the line, each bead is now worth $1.

So, the takeaway is the issuance redistributes wealth...so DOT holders really need to pay attention to the structure of Polkadot's monetary policy because the initial issuance schedule should not be enforced indefinitely. **([1](#futarchy)) How will changes be made to this issuance schedule and what kind of mechanisms are in place to prevent (or expensively price) radical changes?**

Moreover, **([2](#treasury)) once significant dilution is channeled to the treasury, who will manage those funds?** The value accumulated in the treasury represents part of the silent tax on liquid DOT holders -- these stakeholders must have a say in how it is spent! 

## Proposal

So, basically I want to introduce a council tasked with
1. forward guidance on issuance and spending decisions
2. fund management of the treasury

Different stakeholders have different perceptions of the token. Some DOT holders (like VCs) may in the future (in a low DOT price context) prefer less dilution flowing to the treasury. Others (like Parity) might prioritize the chain's security to token's short-term value valuation. We need a council for monetary policy because the ecosystem contains a diverse set of stakeholders with strong opposing opinions on token purpose (although this only makes a difference in a rough price environment). 

The council will represent stakeholders with domain-specific experts that hold their views and represent their interests. Just like other Polkadot councils, this council will be selected by DOT holders via Phragmen's algorithm (and will be rotating). This council should significantly bias low voter turnout, with the expectations that DOT holders will probably delegate most decisions to the domain-specific experts on the council. 

The process for pitching an investment will also be open to the public. To prevent proposal spam, I am thinking it might be appropriate to require significantly more collateral from normal (not-on-council) DOT holders for council-relevant proposals. Members of the council can cheaply propose changes that are voted on, while normal DOT holders must either gain the sponsorship of a council member OR crowdfund collateral from other DOT holders to trigger a referendum. This ensures most proposals that are voted on are from council members, but it also provides a path for disgruntled DOT holders to instigate change without the support of the council. In other words, it prevents DOT holders from being pushed out of governance, but still delegates responsibility in the optimistic setting to the council.

If the council was hypothetically "*captured*" and attempted to vote maliciously, the DOT holders could easily reject the proposal in the referendum. If the *captured council* refused to act in the face of some market calamity, the DOT holders can still crowdfund a proposal above the required threshold to trigger a referendum in which case a high DOT turnout would still guarantee passage. This is a nice balance of power that facilitates delegation of opinion in optimistic scenarios without sacrificing future governance power.

## Monetary Futarchy <a name = "futarchy"></a>
> This is heavily inspired by [forward guidance](https://www.bloomberg.com/quicktake/forward-guidance) -- similarly, it does not need to be binding.

The monetary policy council might present a DOT plot which predicts inflation (defined by how many new coins will be minted) for the next `x` periods (some defined amt of time). For context, the Fed does this with interest rates and calls it the DOT plot, but it would be way cooler if we were to use the name.

![example dot plot](dot_plot.png)

At first, there does not need to be any consequence to these projections. They are a way of relaying expectations in a transparent way. If nothing else, this exercise encourages thinking ahead and encourages relative stability by psychologically anchoring future decisions to these projections.

### Radical Change should be Expensive

If we wanted to take things a step further, it would be possible to set the collateral requirement for monetary governance proposals according to how the proposal aligns with the council's projections. More specifically, radical proposals that veer from the current council projection path (maybe measured in standard deviation) require significantly more collateral than proposals that tweak the existing schedule. 

## Treasury Fund Management <a name = "treasury"></a>

I'm still cooking something up here. I'm leaning towards a system in which the treasury maintains a balance sheet with assets that are varying degrees of liquid relative to DOT tokens. At the network's launch, almost all of the treasury's assets should be DOTs and this should be the case for a while, but eventually the treasury could diversify into less liquid assets. This could even include lending DOTs to stakeholders or even donating to some projects. It could also involve burning DOTs if the network's stakeholders decide they want to take back some of the silent tax that was extracted from the liquid supply.

I wouldn't be all that surprised if the system started to look a lot like the existing system. The relay chain's treasury offers low rates to parachains which offer higher interest rates to apps/users. The treasury could adjust rates to constrain/liberate liquidity based on the ecosystem's economic state.

### staking derivatives will add complexity

Another thing to consider is the liquidity of the funds staked by validators. If the funds are indefinitely locked up, they'll probably be used in a derivatives market soon enough. There will probably also be validator insurance markets. All of these markets will be open to all DOT holders, thereby adding complexity to an already very complicated system. Side (*synthetic*) bets subsidize the underlying, but also increase the systemic risk of the system.