# Monetary Futarchy

Polkadot does not have a predefined issuance schedule by design. *Once the network launches and achieves relative stability*, it will be necessary to communicate clear expectations for the future issuance schedule to stakeholders. This module uses a chosen committee to relay clear expectations of future changes akin to the US Federal Reserve's DOT plot.

## Understanding the Polkadot Treasury

The original purpose of the treasury is to keep the proportion of tokens staked constant; not necessarily constant but constraining liquidity to mitigate outside attacks while also allowing sufficient price discovery with the liquid supply.

In reality, the size of the network will not influence the state and corporation-based powers that could bring it down at its inception. Forcibly keeping some proportion of supply illiquid is a defense mechanism in the context of proof of stake networks.

**NOTE**:
Lower circulating supply (thinner float) does not necessarily mean stronger price support. It just means more volatility (and potential price manipulation) in both directions. **there is a balance that needs to be struck**

So, then why do we need this mechanism? More granular governance over the Treasury's spending governance...

## User Story

* a committee projects a range for changes to issuance
* output a DOT plot with expectations
* record justification for each member

*maybe?*
* price collateral for relevant proposals in proportion to deviation from projections
* generalize to other decisions outside of issuance

*eventually?*
This committee is doing substantial work that is vital to the system `=>` they should participate in the block reward scheme

## rules => decisions => accountability

**inspiration**
* robin hanson
* merkle's paper on DAOs (include quote)