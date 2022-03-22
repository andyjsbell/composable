import { sendAndWaitForSuccess } from '@composable/utils/polkadotjs';
import {KeyringPair} from "@polkadot/keyring/types";
import { u128 } from '@polkadot/types-codec';

/**
 *Contains handler methods for the constantProductDex Tests. 
 */
let poolId: number;  
let constantProductk: number;
let baseAmountTotal: number;
let quoteAmountTotal: number;
let mintedLPTokens: number;
baseAmountTotal = 0;
quoteAmountTotal = 0;
mintedLPTokens = 0;

export async function createPool(sudoKey: KeyringPair, baseAssetId: number, quoteAssetId: number, fee: number) {
  const pool = api.createType('PalletLiquidityBootstrappingPool', {
    owner: api.createType('AccountId32', sudoKey.publicKey),
    pair: api.createType('ComposableTraitsDefiCurrencyPair', {
      base: api.createType('u128', baseAssetId),
      quote: api.createType('u128', quoteAssetId)
    }),
    sale: api.createType('PalletLiquidityBootstrappingSale', {
      start: api.createType('u32', 0),
      end: api.consts.liquidityBootstrapping.maxSaleDuration,
      initialWeight: api.consts.liquidityBootstrapping.maxInitialWeight,
      finalWeight: api.consts.liquidityBootstrapping.minFinalWeight
    }),
    fee: api.createType('Permill', fee)
  });
  const {data: [result],} = await sendAndWaitForSuccess(
    api,
    sudoKey,
    api.events.sudo.Sudid.is,
    api.tx.sudo.sudo(api.tx.liquidityBootstrapping.create(pool))
  );
  return result;
}
export async function addFundstoThePool(walletId:KeyringPair, baseAmount:number, quoteAmount:number){
  const baseAmountParam = api.createType('u128', baseAmount);
  const quoteAmountParam = api.createType('u128', quoteAmount);
  const keepAliveParam = api.createType('bool', true);
  const {data: [,walletIdResult,baseAdded, quoteAdded,returnedLPTokens]} = await sendAndWaitForSuccess(
    api,
    walletId,
    api.events.liquidityBootstrapping.LiquidityAdded.is,
    api.tx.liquidityBootstrapping.addLiquidity(
      poolId,
      baseAmountParam, 
      quoteAmountParam,
      keepAliveParam
    )
  );
  mintedLPTokens += returnedLPTokens.toNumber();
  baseAmountTotal += baseAdded.toNumber();
  quoteAmountTotal += quoteAdded.toNumber();
  return {walletIdResult, baseAdded, quoteAdded, returnedLPTokens};
}

export async function buyFromPool(walletId: KeyringPair, assetId:number, amountToBuy: number){
  const poolIdParam = api.createType('u128', poolId);
  const assetIdParam = api.createType('u128', assetId);
  const amountParam = api.createType('u128', amountToBuy);
  const keepAlive = api.createType('bool', true);
  constantProductk = baseAmountTotal*quoteAmountTotal;
  const expectedConversion = Math.floor((constantProductk/(baseAmountTotal-amountToBuy)))-quoteAmountTotal;
  const {data: [accountId,poolArg,quoteArg,swapArg,amountgathered,quoteAmount,ownerFee] } = await sendAndWaitForSuccess(
    api,
    walletId,
    api.events.liquidityBootstrapping.Swapped.is,
    api.tx.liquidityBootstrapping.buy(
      poolIdParam,
      assetIdParam,
      amountParam,
      keepAlive
    )
  );
  return {accountId, quoteAmount, expectedConversion, ownerFee};
}

export async function sellToPool(walletId: KeyringPair, assetId: number, amount:number){
  const poolIdParam = api.createType('u128', poolId);
  const assetIdParam = api.createType('u128', assetId);
  const amountParam = api.createType('u128', amount);
  const keepAliveParam = api.createType('bool', false);
  const {data: [result, ...rest]} = await sendAndWaitForSuccess(
    api,
    walletId,
    api.events.liquidityBootstrapping.Swapped.is,
    api.tx.liquidityBootstrapping.sell(
      poolIdParam,
      assetIdParam,
      amountParam,
      keepAliveParam
    )
  )
  return result.toString();        
}

export async function removeLiquidityFromPool(walletId: KeyringPair, lpTokens:number){
  const expectedLPTokens = mintedLPTokens-lpTokens;
  const poolIdParam = api.createType('u128', poolId);
  const {data: [resultPoolId,resultWallet,resultBase,resultQuote,remainingLpTokens]}=await sendAndWaitForSuccess(
    api,
    walletId,
    api.events.liquidityBootstrapping.LiquidityRemoved.is, // Doesn't Exist!
    api.tx.liquidityBootstrapping.removeLiquidity(
      poolIdParam
    )
  );   
  return {remainingLpTokens, expectedLPTokens}
}

export async function swapTokenPairs(wallet: KeyringPair, 
  baseAssetId: number,
  quoteAssetId:number,
  quoteAmount: number,
  minReceiveAmount = 0
  ){
    const poolIdParam = api.createType('u128', poolId);
    const currencyPair = api.createType('ComposableTraitsDefiCurrencyPair', {
      base: api.createType('u128', baseAssetId),
      quote: api.createType('u128',quoteAssetId)
    });
    const quoteAmountParam = api.createType('u128', quoteAmount);
    const minReceiveParam = api.createType('u128', minReceiveAmount);
    const keepAliveParam = api.createType('bool', true);
    const {data: [resultPoolId,resultWallet,resultQuote,resultBase,resultBaseAmount,returnedQuoteAmount,]}= await sendAndWaitForSuccess(
      api,
      wallet,
      api.events.liquidityBootstrapping.Swapped.is,
      api.tx.liquidityBootstrapping.swap(
        poolIdParam,
        currencyPair,
        quoteAmountParam,
        minReceiveParam,
        keepAliveParam
      )
    );
    return {returnedQuoteAmount};
}

export async function getUserTokens(walletId: KeyringPair, assetId: number){
  const {free, reserved, frozen} = await api.query.tokens.accounts(walletId.address, assetId); 
  return free.toNumber();
}

export async function getOwnerFee(poolId: number){
  const result = await api.query.liquidityBootstrapping.pools(api.createType('u128', poolId));
  return result.unwrap()
}