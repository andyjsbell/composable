import testConfiguration from './test_configuration.json';
import {expect} from "chai";
import {KeyringPair} from "@polkadot/keyring/types";
import { addFundstoThePool, buyFromPool, createPool, getOwnerFee, getUserTokens, removeLiquidityFromPool, sellToPool, swapTokenPairs } from './testHandlers/liquidityBootstrappingDexHelper';
import { mintAssetsToWallet } from '@composable/utils/mintingHelper';
import {waitForBlocks} from "@composable/utils/polkadotjs";

/**
 * This suite includes tests for the liquidityBootstrapping Pallet.
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
    baseAmount = 250000000000;
    quoteAmount = 250000000000;
    //sets the owner fee to 1.00%/Type Permill
    ownerFee = 10000;
  });

  before('Minting assets', async function() {
    this.timeout(8*60*1000);
    await mintAssetsToWallet(sudoKey, sudoKey, [baseAssetId, quoteAssetId])
    await mintAssetsToWallet(walletId1, sudoKey, [1, baseAssetId]);
    await mintAssetsToWallet(walletId2, sudoKey, [1, baseAssetId, quoteAssetId]);
  });

  describe('tx.liquidityBootstrapping Success Tests', function() {
    if(!testConfiguration.enabledTests.successTests.enabled){
      return;
    }

    it('Users can create a liquidityBootstrapping pool', async function() {
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

    it('Pool creator can add more liquidity', async function(){
      if(!testConfiguration.enabledTests.successTests.addLiquidityTests.enabled){
        return;
      }
      this.timeout(2*60*1000);
      const result = await addFundstoThePool(
        sudoKey,
        baseAmount,
        quoteAmount
      );
      //Once funds added to the pool, User is deposited with LP Tokens.
      wallet1LpTokens = result.returnedLPTokens.toNumber();
      expect(result.baseAdded.toNumber()).to.be.equal(baseAmount);
      expect(result.quoteAdded.toNumber()).to.be.equal(quoteAmount);
      expect(result.walletIdResult.toString()).to.be.equal(walletId1Account);
    });

    it('User1 can buy from the pool', async function() {
      if(!testConfiguration.enabledTests.successTests.buyTest.enabled){
        return;
      }
      this.timeout(2 * 60 * 1000);
      const result = await buyFromPool(walletId1, baseAssetId, 1000000000);
      expect(result.accountId.toString()).to.be.equal(walletId1Account);
      expect(result.quoteAmount.toNumber()).to.be.equal(result.expectedConversion);
    });

    it('User1 can sell on the pool', async function(){
      if(!testConfiguration.enabledTests.successTests.sellTest.enabled){
        return;
      }
      this.timeout(2*60*1000);
      const {data: [resultPoolId, resultAccountId]} = await sellToPool(walletId1, baseAssetId, 100000000000);
      expect(resultAccountId.toString()).to.be.equal(api.createType('AccountId32', walletId1.publicKey).toString());
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

    it('Pool creator should be able to remove liquidity', async function(){
      if(!testConfiguration.enabledTests.successTests.removeLiquidityTest.enabled){
        return;
      }
      this.timeout(2*60*1000);
      //Randomly checks an integer value that is always < mintedLPTokens.
      const result = await removeLiquidityFromPool(sudoKey, Math.floor(Math.random()*wallet1LpTokens));
      console.debug(result.toString());
    });
  });
})




