import testConfiguration from './test_configuration.json';
import {expect} from "chai";
import {KeyringPair} from "@polkadot/keyring/types";
import { addFundstoThePool, buyFromPool, createPool, getOwnerFee, getUserTokens, removeLiquidityFromPool, sellToPool, swapTokenPairs } from './testHandlers/liquidityBootstrappingDexHelper';
import { mintAssetsToWallet } from '@composable/utils/mintingHelper';
import {waitForBlocks} from "@composable/utils/polkadotjs";

/**
 * This suite includes tests for the constantProductDex Pallet.
 * Tested functionalities are:
 * Create - AddLiquidity - Buy - Sell - Swap - RemoveLiquidity with basic calculations with constantProductFormula and OwnerFee.
 * Mainly consists of happy path testing.
 */
describe('tx.liquidityBootstrapping Tests', function () {

  let sudoKey: KeyringPair,
    walletId1: KeyringPair,
    walletId2: KeyringPair;
  let poolId: number,
    baseAssetId: number,
    quoteAssetId: number,
    wallet1LpTokens: number,
    baseAmount: number,
    quoteAmount: number,
    ownerFee: number,
    walletId1Account: string,
    walletId2Account: string;

  before('Initialize variables', function() {
    sudoKey = walletAlice;
    walletId1 = walletEve.derive("/test/liquidityBootstrapping/walletId1");
    walletId2 = walletBob.derive("/test/liquidityBootstrapping/walletId2");
    walletId1Account = api.createType('AccountId32', walletId1.address).toString();
    walletId2Account = api.createType('AccountId32', walletId2.address).toString();
    baseAssetId = 4;
    quoteAssetId = 129;
    baseAmount = 2500;
    quoteAmount = 2500;
    //sets the owner fee to 1.00%/Type Permill
    ownerFee = 10000;
  });

  before('Minting assets', async function() {
    this.timeout(8*60*1000);
    await mintAssetsToWallet(walletId1, walletAlice, [1, baseAssetId, quoteAssetId]);
    await mintAssetsToWallet(walletId2, walletAlice, [1, baseAssetId, quoteAssetId]);
  });

  describe('tx.constantProductDex Success Tests', function() {
    if(!testConfiguration.enabledTests.successTests.enabled){
      return;
    }

    it('Users can create a constantProduct pool', async function() {
      if(!testConfiguration.enabledTests.successTests.createPool.enabled){
        return;
      }
      this.timeout(2*60*1000);
      const result = await createPool(
        sudoKey,
        baseAssetId,
        quoteAssetId,
        ownerFee
      );
      expect(result.isOk).to.be.true;
    })

    it('Given that users has sufficient balance, User1 can send funds to pool', async function(){
      if(!testConfiguration.enabledTests.successTests.addLiquidityTests.enabled){
        return;
      }
      this.timeout(2*60*1000);
      await waitForBlocks(2);
      const result = await addFundstoThePool(
        walletId1,
        baseAmount,
        quoteAmount
      );
      //Once funds added to the pool, User is deposited with LP Tokens.
      wallet1LpTokens = result.returnedLPTokens.toNumber();
      expect(result.baseAdded.toNumber()).to.be.equal(baseAmount);
      expect(result.quoteAdded.toNumber()).to.be.equal(quoteAmount);
      expect(result.walletIdResult.toString()).to.be.equal(walletId1Account);
    });

    it('User1 can buy from the pool and router respects the constantProductFormula', async function() {
      if(!testConfiguration.enabledTests.successTests.buyTest.enabled){
        return;
      }
      this.timeout(2 * 60 * 1000);
      const result = await buyFromPool(walletId1, baseAssetId, 100000000000);
      expect(result.accountId.toString()).to.be.equal(walletId1Account);
      //Expected amount is calculated based on the constantProductFormula which is 1:1 for this case.
      expect(result.quoteAmount.toNumber()).to.be.equal(result.expectedConversion);
    });

    it('User1 can sell on the pool', async function(){
      if(!testConfiguration.enabledTests.successTests.sellTest.enabled){
        return;
      }
      this.timeout(2*60*1000);
      const accountIdSeller = await sellToPool(walletId1, baseAssetId, 100000000000);
      expect(accountIdSeller).to.be.equal(walletId1Account);
    });

    it('User2 can swap from the pool', async function(){
      if(!testConfiguration.enabledTests.successTests.swapTest.enabled){
        return;
      }
      this.timeout(2*60*1000);
      const quotedAmount = 12;
      const result = await swapTokenPairs(
        walletId2,
        baseAssetId,
        quoteAssetId,
        quotedAmount,
      );
      console.debug(result); // ToDo (D. Roth): Update!
    });

    it('User1 can remove liquidity from the pool by using LP Tokens', async function(){
      if(!testConfiguration.enabledTests.successTests.removeLiquidityTest.enabled){
        return;
      }
      this.timeout(2*60*1000);
      //Randomly checks an integer value that is always < mintedLPTokens.
      const result = await removeLiquidityFromPool(walletId1, Math.floor(Math.random()*wallet1LpTokens));
      console.debug(result.toString());
    });
  });
})




