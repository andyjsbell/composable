import testConfiguration from './test_configuration.json';
import {expect} from "chai";
import {KeyringPair} from "@polkadot/keyring/types";
import { addFundstoThePool, buyFromPool, createPool, removeLiquidityFromPool, sellToPool, swapTokenPairs } from './testHandlers/liquidityBootstrappingDexHelper';
import { mintAssetsToWallet } from '@composable/utils/mintingHelper';
import {waitForBlocks} from "@composable/utils/polkadotjs";

/**
 * This suite includes tests for the liquidityBootstrapping Pallet.
 */
describe('tx.liquidityBootstrapping Tests', function () {

  let sudoKey: KeyringPair,
    wallet: KeyringPair;
  let poolId: number,
    poolId2: number,
    baseAssetId: number,
    quoteAssetId: number,
    baseAmount: number,
    quoteAmount: number,
    ownerFee: number;

  before('Initialize variables', function() {
    sudoKey = walletAlice;
    wallet = walletEve.derive("/test/liquidityBootstrapping");
    baseAssetId = 4;
    quoteAssetId = 129;
    baseAmount = 250000000000;
    quoteAmount = 250000000000;
    //sets the owner fee to 1.00%/Type Permill
    ownerFee = 10000;
  });

  before('Minting assets', async function() {
    return;
    this.timeout(8*60*1000);
    await mintAssetsToWallet(sudoKey, sudoKey, [baseAssetId, quoteAssetId])
    await mintAssetsToWallet(wallet, sudoKey, [1, baseAssetId, quoteAssetId]);
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
      const end = api.createType('u32', api.consts.liquidityBootstrapping.maxSaleDuration);
      const {data: [result],} = await createPool(
        sudoKey,
        baseAssetId,
        quoteAssetId,
        ownerFee,
        end
      );
      expect(result.isOk).to.be.true;
      poolId = (await api.query.liquidityBootstrapping.poolCount()).toNumber() - 1;
    })

    it('Pool creator can add more liquidity', async function(){
      if(!testConfiguration.enabledTests.successTests.addLiquidityTests.enabled){
        return;
      }
      this.timeout(2*60*1000);
      const {data: [result,]} = await addFundstoThePool(
        sudoKey,
        poolId,
        baseAmount,
        quoteAmount
      );
      console.debug(result.toString());
    });

    it('User can buy from the pool', async function() {
      if(!testConfiguration.enabledTests.successTests.buyTest.enabled){
        return;
      }
      this.timeout(2 * 60 * 1000);
      const {data: [result, resultAccountId],} = await buyFromPool(wallet, poolId, baseAssetId, 1000000000);
      expect(resultAccountId.toString()).to.be.equal(api.createType('AccountId32', wallet.publicKey).toString());
    });

    it('User can sell on the pool', async function(){
      if(!testConfiguration.enabledTests.successTests.sellTest.enabled){
        return;
      }
      this.timeout(2*60*1000);
      const {data: [resultPoolId, resultAccountId]} = await sellToPool(wallet, poolId, baseAssetId, 100000000000);
      expect(resultAccountId.toString()).to.be.equal(api.createType('AccountId32', wallet.publicKey).toString());
    });

    it('User can swap from the pool', async function(){
      if(!testConfiguration.enabledTests.successTests.swapTest.enabled){
        return;
      }
      this.timeout(2*60*1000);
      const quotedAmount = baseAmount;
      const {data: [result],} = await swapTokenPairs(
        wallet,
        poolId,
        baseAssetId,
        quoteAssetId,
        quotedAmount,
      );
      console.debug(result.toString()); // ToDo (D. Roth): Update!
    });
  });

  describe.only('tx.liquidityBootstrapping Failure Tests', function() {
    if (!testConfiguration.enabledTests.successTests.enabled) {
      return;
    }

    it('Sudo can create a second liquidityBootstrapping pool', async function () {
      if (!testConfiguration.enabledTests.successTests.createPool.enabled) {
        return;
      }
      this.timeout(2 * 60 * 1000);
      const startDelay = 3;
      const end = api.createType('u32',
        api.createType('u32', (await api.query.system.number()).toNumber() + startDelay + 1000)
      )
      const {data: [result],} = await createPool(
        sudoKey,
        baseAssetId,
        quoteAssetId,
        ownerFee,
        end,
        startDelay
      );
      expect(result.isOk).to.be.true;
      poolId2 = (await api.query.liquidityBootstrapping.poolCount()).toNumber() - 1;
    })

    it('Pool creator can add more liquidity to second pool', async function () {
      if (!testConfiguration.enabledTests.successTests.addLiquidityTests.enabled) {
        return;
      }
      this.timeout(2 * 60 * 1000);
      const {data: [result,]} = await addFundstoThePool(
        sudoKey,
        poolId2,
        baseAmount,
        quoteAmount
      );
      console.debug(result.toString());
    });

    /**
     * Canceling the above created second pool.
     *
     * This only works if poolState == Ended
     */
    it('Pool creator can cancel second pool', async function () {
      if (!testConfiguration.enabledTests.successTests.removeLiquidityTest.enabled) {
        return;
      }
      this.timeout(2 * 60 * 1000);
      await waitForBlocks(10);
      const {data: [result],} = await removeLiquidityFromPool(sudoKey, poolId2);
      console.debug(result.toString());
    });
  });
})
