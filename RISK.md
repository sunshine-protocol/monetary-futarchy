# Privacy by Default is Necessary for Public Good Investments

**Price slippage** describes how the size of a trade can influence how the price changes as the trade is executed. While there exist strategies to batch trades as to not spook the market, there is no strategy that can perfectly elude the law of demand and supply (except maybe [market manipulation](https://arxiv.org/pdf/1811.10109.pdf)). 

If fund management is completely transparent for all decisions, market actors can easily watch the decision-making processes unfold and intercept the trade. Even if all investment pitches require a ceiling price for buying (or floor for selling), this would only limit the arbitrage of scavenger market actors. The defacto transparency of on-chain runtime logic complicates governance if all decisions are made in complete publicity. 

Actually, even if the decision-making process is only viewable by participants, this would cause information asymmetry in which participants are given access to insider information. I don't know how we can prevent participants from acting on some information asymmetry in every scenario...usually I feel like modern blockchains just throw sortition at this sort of problem at the protocol layer. In this scenario, we could use a dispute resolution protocol in which insider trader claims are voted on by a jury of actors selected randomly:

> Add a `court` module which selects a random subset of members not involved in the incident (didn't bring forth the charge or get accused) and vote on whether the evidence is satisfactory. Bringing the claim to the `court` requires collateral and evidence -- if the accuser wins, they presumably receive some funds from the decidedly guilty defendant. If the defendant wins, they receive the bond put up by the accuser. Appeals require putting up more collateral and there is a hard limit on the number of appeals. This limit should be still be adjusted according to the participant size and the rolling average of claims brought to the court (to basically manage this system so it doesn't allow for as many appeals when there is high demand for the court).

**ANYWAYS**, the original point was that we need a system wherein members and coinvoters can vote on proposals, but cannot see the votes of their peers. This an open problem.

## What does this mean for forward guidance?

Polkadot's treasury can still use forward guidance for relaying expectations for the DOT issuance schedule. If the treasury starts making investments, we'll probably need to use [zk stuff](https://github.com/LayerXcom/zero-chain/tree/master/modules/encrypted-balances) to execute the trade, but the size of the trade will always influence how easy it is to hide shifting market dynamics. These assets still have a relatively low volume so expect high volatility and high sensitivity to supply/demand adjustments. Might be a rocky start so I'm thinking it might be easier if the Polkadot treasury only makes donations and **loans** capital to companies, parachains, etc. 

Once volume increases and the correct privacy enhancing infrastructure is in place, governance might be able to extend to donations without information asymmetry or complete publicity. Until then, I recommend limiting the treasury's functionality to liquidity provision rather than direct investment. Although the difference is actually negligible in some settings.

## Qualifying Investments: Early Stage, Private (Not Publicly Traded) <a name = "conclusion"></a>

Investments should be proposed as term sheets. While VCs and hedge funds can swoop in and interrupt an investment, this might still achieve the intended purpose of funding the target project. I'm still considering how this should work, but the lack of liquidity prevents the clear arbitrage opportunity discussed above.

It would be cool to structure the approval process as multi-round conversation between the intended recipient and the fund/treasury. Who is allowed to initiate that conversation and where does it go from there? what are the dispute resolution scenarios? Should this *conversation*-esque format also be used for negotiating changing milestones and sub-milestones for grants executed on-chain? Probably.

> *MPC is prohibitively expensive in this setting usually, but it could make sense if/when the transaction size is large enough (and expect this to be increasingly common as larger transactions occur more often and MPC increases in efficiency).* I also always imagine MPC as a multi-round conversation 
