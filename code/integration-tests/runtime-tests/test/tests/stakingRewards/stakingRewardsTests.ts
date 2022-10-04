import { expect } from "chai";
import { ApiPromise } from "@polkadot/api";
import testConfiguration from "./test_configuration.json";
import { KeyringPair } from "@polkadot/keyring/types";
import { getNewConnection } from "@composable/utils/connectionHelper";
import { getDevWallets } from "@composable/utils/walletHelper";
import { sendAndWaitForSuccess, waitForBlocks } from "@composable/utils/polkadotjs";
import { ComposableTraitsStakingRewardPool, ComposableTraitsStakingStake } from "@composable/types/interfaces";
import { Null, Option, u128, u64 } from "@polkadot/types-codec";
import BN from "bn.js";
import { before } from "mocha";
import { mintAssetsToWallet } from "@composable/utils/mintingHelper";

/**
 * Staking Rewards Pallet Tests
 */
describe.only("[SHORT] tx.stakingRewards Tests", function () {
  if (!testConfiguration.enabledTests.query.enabled) return;
  this.retries(0);

  let api: ApiPromise;
  let sudoKey: KeyringPair,
    walletStaker: KeyringPair,
    walletStaker2: KeyringPair,
    walletOwner: KeyringPair,
    walletRewardAdder: KeyringPair;
  let fNFTCollectionId1: u128,
    fNFTCollectionId2: u128,
    fNFTCollectionId3: u128,
    fNFTCollectionId4Pica: u128,
    fNFTCollectionId5Pblo: u128;
  let fNFTInstanceId1: u64,
    fNFTInstanceId2: u64,
    fNFTInstanceId3: u64,
    fNFTInstanceId4Pica: u64,
    fNFTInstanceId5Pblo: u64;
  let amountAfterStake: BN;

  let stakeAmountAfterExtending: BN;

  const STAKE_AMOUNT = 100_000_000_000;
  const UNLOCK_PENALTY = 100000000;

  const POOL_1_BASE_ASSET_ID = 10000;
  const POOL_1_REWARD_ASSET_ID = 10001;

  const POOL_2_BASE_ASSET_ID = 20000;
  const POOL_2_REWARD_ASSET_ID_1 = 20001;
  const POOL_2_REWARD_ASSET_ID_2 = 20002;

  const PBLO_ASSET_ID = 5;

  let poolId1: u128, poolId2: u128;

  before("Setting up the tests", async function () {
    this.timeout(60 * 1000);
    // Getting connection & wallets
    const { newClient, newKeyring } = await getNewConnection();
    api = newClient;
    const { devWalletAlice, devWalletBob, devWalletEve } = getDevWallets(newKeyring);
    sudoKey = devWalletAlice;
    walletStaker = devWalletBob.derive("/test/staking-rewards/staker");
    walletStaker2 = devWalletBob.derive("/test/staking-rewards/staker2");
    walletRewardAdder = devWalletBob.derive("/test/staking-rewards/reward/adder");
    walletOwner = devWalletEve.derive("/test/staking-rewards/owner");
  });

  before("Providing funds", async function () {
    this.timeout(5 * 60 * 1000);
    await mintAssetsToWallet(api, walletStaker, sudoKey, [
      1,
      PBLO_ASSET_ID,
      POOL_1_BASE_ASSET_ID,
      POOL_1_REWARD_ASSET_ID,
      POOL_2_BASE_ASSET_ID,
      POOL_2_REWARD_ASSET_ID_1,
      1_000_000_000_000_000
    ]);
    await mintAssetsToWallet(api, walletStaker2, sudoKey, [
      1,
      PBLO_ASSET_ID,
      POOL_1_BASE_ASSET_ID,
      POOL_1_REWARD_ASSET_ID,
      POOL_2_BASE_ASSET_ID,
      POOL_2_REWARD_ASSET_ID_1,
      1_000_000_000_000_000
    ]);
    await mintAssetsToWallet(api, walletOwner, sudoKey, [
      1,
      PBLO_ASSET_ID,
      POOL_1_BASE_ASSET_ID,
      POOL_1_REWARD_ASSET_ID,
      POOL_2_BASE_ASSET_ID,
      POOL_2_REWARD_ASSET_ID_1,
      1_000_000_000_000_000
    ]);
    await mintAssetsToWallet(api, walletRewardAdder, sudoKey, [
      1,
      PBLO_ASSET_ID,
      POOL_1_BASE_ASSET_ID,
      POOL_1_REWARD_ASSET_ID,
      POOL_2_BASE_ASSET_ID,
      POOL_2_REWARD_ASSET_ID_1,
      1_000_000_000_000_000
    ]);
  });

  after("Closing the connection", async function () {
    await api.disconnect();
  });

  describe.only("tx.stakingRewards.createRewardPool Tests", function () {
    it("Sudo can create a new staking reward pool", async function () {
      this.timeout(2 * 60 * 1000);
      // Parameters
      const currentBlockNumber = await api.query.system.number();
      const startBlock = api.createType("u32", currentBlockNumber.addn(4));
      const endBlock = api.createType("u32", currentBlockNumber.addn(24));
      const assetId = api.createType("u128", POOL_1_BASE_ASSET_ID);
      const rewardAssetId = POOL_1_REWARD_ASSET_ID.toString();
      const maxRewards = "1000000000";
      const rewardPeriodPerSecond = "100000";
      const amount = (10 ** 12).toString();
      const durationPreset = {
        "2592000": "1000000000"
      };
      // Creating pool config parameter
      const poolConfig = api.createType("ComposableTraitsStakingRewardPoolConfiguration", {
        RewardRateBasedIncentive: {
          owner: walletOwner.publicKey,
          assetId: assetId,
          startBlock: startBlock,
          endBlock: endBlock,
          rewardConfigs: api.createType("BTreeMap<u128, ComposableTraitsStakingRewardConfig>", {
            "10001": {
              maxRewards: maxRewards,
              rewardRate: {
                period: {
                  PerSecond: rewardPeriodPerSecond
                },
                amount: amount
              }
            }
          }),
          lock: {
            durationPresets: durationPreset,
            unlockPenalty: UNLOCK_PENALTY
          },
          shareAssetId: 10000001,
          financialNftAssetId: 10000002
        }
      });

      // Transaction
      const {
        data: [resultPoolId, resultOwner, resultEndBlock]
      } = await sendAndWaitForSuccess(
        api,
        sudoKey,
        api.events.stakingRewards.RewardPoolCreated.is,
        api.tx.sudo.sudo(api.tx.stakingRewards.createRewardPool(poolConfig))
      );
      poolId1 = resultPoolId;

      // Verifications
      // Querying pool info
      const poolInfo = <Option<ComposableTraitsStakingRewardPool>>await api.query.stakingRewards.rewardPools(poolId1);
      console.debug(poolInfo.unwrap().toHuman());
      expect(poolInfo.unwrap().owner.toString())
        .to.be.equal(resultOwner.toString())
        .to.be.equal(api.createType("AccountId32", walletOwner.publicKey).toString());
      expect(poolInfo.unwrap().assetId.toString()).to.equal(POOL_1_BASE_ASSET_ID.toString());
      expect(resultEndBlock.toNumber()).to.be.equal(endBlock.toNumber());
    });

    it("Sudo can create a second new staking reward pool", async function () {
      this.timeout(2 * 60 * 1000);
      // Parameters
      const currentBlockNumber = await api.query.system.number();
      const startBlock = api.createType("u32", currentBlockNumber.addn(4));
      const endBlock = api.createType("u32", currentBlockNumber.addn(16));
      const assetId = api.createType("u128", POOL_2_BASE_ASSET_ID);
      const maxRewards1 = (100 * 10 ** 12).toString();
      const maxRewards2 = (100 * 10 ** 12).toString();
      const maxRewards3 = (100 * 10 ** 12).toString();
      const rewardPeriodPerSecond = "100000";
      const amount1 = (0.1 * 10 ** 12).toString();
      const amount2 = (0.5 * 10 ** 12).toString();
      const amount3 = (10 ** 12).toString();

      const durationPreset = {
        "2592000": "1000000000"
      };
      // Creating pool config parameter
      const poolConfig = api.createType("ComposableTraitsStakingRewardPoolConfiguration", {
        RewardRateBasedIncentive: {
          owner: walletOwner.publicKey,
          assetId: assetId,
          startBlock: startBlock,
          endBlock: endBlock,
          rewardConfigs: api.createType("BTreeMap<u128, ComposableTraitsStakingRewardConfig>", {
            // The dict keys are the reward asset IDs!
            "20001": {
              maxRewards: maxRewards1,
              rewardRate: {
                period: {
                  PerSecond: rewardPeriodPerSecond
                },
                amount: amount1
              }
            },
            "20002": {
              maxRewards: maxRewards2,
              rewardRate: {
                period: {
                  PerSecond: rewardPeriodPerSecond
                },
                amount: amount2
              }
            },
            "20003": {
              maxRewards: maxRewards3,
              rewardRate: {
                period: {
                  PerSecond: rewardPeriodPerSecond
                },
                amount: amount3
              }
            }
          }),
          lock: {
            durationPresets: durationPreset,
            unlockPenalty: UNLOCK_PENALTY
          },
          shareAssetId: 20000001,
          financialNftAssetId: 20000002
        }
      });

      // Transaction
      const {
        data: [resultPoolId, resultOwner, resultEndBlock]
      } = await sendAndWaitForSuccess(
        api,
        sudoKey,
        api.events.stakingRewards.RewardPoolCreated.is,
        api.tx.sudo.sudo(api.tx.stakingRewards.createRewardPool(poolConfig))
      );
      poolId2 = resultPoolId;

      // Verifications
      // Querying pool info
      const poolInfo = <Option<ComposableTraitsStakingRewardPool>>await api.query.stakingRewards.rewardPools(poolId2);
      expect(poolInfo.unwrap().owner.toString())
        .to.be.equal(resultOwner.toString())
        .to.be.equal(api.createType("AccountId32", walletOwner.publicKey).toString());
      expect(poolInfo.unwrap().assetId.toString()).to.equal(POOL_2_BASE_ASSET_ID.toString());
      expect(resultEndBlock.toNumber()).to.be.equal(endBlock.toNumber());
    });
  });

  describe.only("tx.stakingRewards.addToRewardsPot", function () {
    it("Pool owner can add rewards to pot", async function () {
      this.timeout(2 * 60 * 1000);

      // Parameters
      const poolId = poolId1;
      const assetId = POOL_1_REWARD_ASSET_ID;
      const amount = 1000000000;
      const keepAlive = false;
      // ToDo (D. Roth)
      const poolInfoB = <Option<any>> await api.query.stakingRewards.rewardsPotIsEmpty(poolId1, null);
      expect(poolInfoB.unwrapOr(undefined)).to.be.undefined;
      // Transaction
      const {
        data: [resultPoolId, resultAssetId, resultAmount]
      } = await sendAndWaitForSuccess(
        api,
        walletOwner,
        api.events.stakingRewards.RewardsPotIncreased.is,
        api.tx.stakingRewards.addToRewardsPot(poolId, assetId, amount, keepAlive)
      );

      // Verification
      expect(poolId).to.be.bignumber.equal(resultPoolId);
      expect(new BN(assetId)).to.be.bignumber.equal(resultAssetId);
      expect(new BN(amount)).to.be.bignumber.equal(resultAmount);
      const poolInfo = <Option<any>>await api.query.stakingRewards.rewardsPotIsEmpty(poolId1, "");
      console.debug(poolInfo.unwrap().toHuman());
    });

    it("Pool owner can add rewards to second pot", async function () {
      this.timeout(2 * 60 * 1000);

      // Parameters
      const poolId = poolId2;
      const assetId = POOL_2_REWARD_ASSET_ID_1;
      const amount = 1000000000;
      const keepAlive = false;

      // Transaction
      const {
        data: [resultPoolId, resultAssetId, resultAmount]
      } = await sendAndWaitForSuccess(
        api,
        walletOwner,
        api.events.stakingRewards.RewardsPotIncreased.is,
        api.tx.stakingRewards.addToRewardsPot(poolId, assetId, amount, keepAlive)
      );

      // Verification
      expect(poolId).to.be.bignumber.equal(resultPoolId);
      expect(new BN(assetId)).to.be.bignumber.equal(resultAssetId);
      expect(new BN(amount)).to.be.bignumber.equal(resultAmount);
    });

    it("Some user can add rewards to second pot", async function () {
      this.timeout(2 * 60 * 1000);

      // Parameters
      const poolId = poolId2;
      const assetId = POOL_2_REWARD_ASSET_ID_1;
      const amount = 1000000000;
      const keepAlive = false;

      // Transaction
      const {
        data: [resultPoolId, resultAssetId, resultAmount]
      } = await sendAndWaitForSuccess(
        api,
        walletRewardAdder,
        api.events.stakingRewards.RewardsPotIncreased.is,
        api.tx.stakingRewards.addToRewardsPot(poolId, assetId, amount, keepAlive)
      );

      // Verification
      expect(poolId).to.be.bignumber.equal(resultPoolId);
      expect(new BN(assetId)).to.be.bignumber.equal(resultAssetId);
      expect(new BN(amount)).to.be.bignumber.equal(resultAmount);
    });
  });

  describe("tx.stakingRewards.updateRewardsPool", function () {
    it("Pool owner can update pool configuration", async function () {
      this.timeout(2 * 60 * 1000);
      // Getting funds before
      const poolInfoBefore = <Option<ComposableTraitsStakingRewardPool>>(
        await api.query.stakingRewards.rewardPools(poolId1)
      );
      // Parameters
      const amount = "1000000000";
      const rewardUpdates = api.createType("BTreeMap<u128, ComposableTraitsStakingRewardUpdate>", {
        "10001": {
          rewardRate: {
            period: {
              PerSecond: 5000
            },
            amount: amount
          }
        }
      });

      // Transaction
      const {
        data: [resultPoolId]
      } = await sendAndWaitForSuccess(
        api,
        sudoKey,
        api.events.stakingRewards.RewardPoolUpdated.is,
        api.tx.sudo.sudo(api.tx.stakingRewards.updateRewardsPool(poolId1, rewardUpdates))
      );

      // Verification
      expect(resultPoolId).to.be.bignumber.equal(poolId1);
      const poolInfoAfter = <Option<ComposableTraitsStakingRewardPool>>(
        await api.query.stakingRewards.rewardPools(poolId1)
      );
      expect(poolInfoAfter.unwrap().owner.toString()).to.equal(
        api.createType("AccountId32", walletOwner.publicKey).toString()
      );
      expect(poolInfoAfter.unwrap().assetId.toString()).to.equal("4");
      expect(poolInfoAfter.unwrap().rewards[1]["rewardRate"]["amount"].toString())
        .to.be.equal(amount.toString())
        .to.be.greaterThan(poolInfoBefore.unwrap().rewards[1]["rewardRate"]["amount"]);
    });
  });

  describe.only("tx.stakingRewards.stake Tests", function () {
    it("Users can stake in the newly created reward pool", async function () {
      this.timeout(2 * 60 * 1000);
      // Parameters
      const userFundsBefore = await api.rpc.assets.balanceOf(POOL_1_BASE_ASSET_ID.toString(), walletStaker.publicKey);
      const durationPreset = 2592000;

      // Transaction
      const {
        data: [
          resultPoolId,
          resultOwnerAccountId,
          resultAmount,
          resultDurationPreset,
          resultFNFTCollectionId,
          resultFNFTInstanceId,
          resultKeepAlive
        ]
      } = await sendAndWaitForSuccess(
        api,
        walletStaker,
        api.events.stakingRewards.Staked.is,
        api.tx.stakingRewards.stake(poolId1, STAKE_AMOUNT, durationPreset)
      );

      console.debug(api.createType('AccountId32', resultOwnerAccountId).toString());
      console.debug(api.createType('AccountId32', walletOwner.publicKey).toString());
      console.debug(api.createType('AccountId32', walletStaker.publicKey).toString());
      // Verification
      expect(resultPoolId).to.be.bignumber.equal(poolId1);
      expect(resultAmount.toString()).to.equal(STAKE_AMOUNT.toString());
      expect(resultDurationPreset.toString()).to.equal(durationPreset.toString());
      fNFTCollectionId1 = resultFNFTCollectionId;
      fNFTInstanceId1 = resultFNFTInstanceId;
      expect(resultKeepAlive.isTrue).to.be.true;

      // Comparing with data from Query
      const stakeInfoAfter = <Option<ComposableTraitsStakingStake>>(
        await api.query.stakingRewards.stakes(fNFTCollectionId1, "")
      );
      console.debug(stakeInfoAfter.toHuman());
      expect(stakeInfoAfter.unwrap().stake).to.be.bignumber.equal(new BN(STAKE_AMOUNT));
      amountAfterStake = stakeInfoAfter.unwrap().stake;
      expect(stakeInfoAfter.unwrap().share).to.be.bignumber.equal(new BN(STAKE_AMOUNT));
      expect(stakeInfoAfter.unwrap().lock.unlockPenalty).to.be.bignumber.equal(new BN(UNLOCK_PENALTY));

      // Checking funds
      const userFundsAfter = await api.rpc.assets.balanceOf(POOL_1_BASE_ASSET_ID.toString(), walletStaker.publicKey);
      expect(new BN(userFundsAfter.toString())).to.be.bignumber.lessThan(
        new BN(userFundsBefore.toString()).add(new BN(STAKE_AMOUNT))
      );
    });

    it("Another User can stake in the newly created reward pool", async function () {
      this.timeout(2 * 60 * 1000);
      // Getting funds before
      const userFundsBefore = await api.rpc.assets.balanceOf(POOL_1_BASE_ASSET_ID.toString(), walletStaker.publicKey);
      // Parameters
      const durationPreset = 2592000;

      // Transaction
      const {
        data: [
          resultPoolId,
          resultOwnerAccountId,
          resultAmount,
          resultDurationPreset,
          resultFNFTCollectionId,
          resultFNFTInstanceId,
          resultKeepAlive
        ]
      } = await sendAndWaitForSuccess(
        api,
        walletStaker2,
        api.events.stakingRewards.Staked.is,
        api.tx.stakingRewards.stake(poolId1, STAKE_AMOUNT, durationPreset)
      );

      // Verification
      expect(resultPoolId).to.be.bignumber.equal(poolId1);
      expect(resultAmount.toString()).to.equal(STAKE_AMOUNT.toString());
      expect(resultDurationPreset.toString()).to.equal(durationPreset.toString());
      fNFTCollectionId2 = resultFNFTCollectionId;
      fNFTInstanceId2 = resultFNFTInstanceId;

      // Comparing with data from query
      const stakeInfoAfter = <Option<ComposableTraitsStakingStake>>(
        await api.query.stakingRewards.stakes(fNFTCollectionId2, "")
      );
      console.debug(stakeInfoAfter.toHuman());
      expect(stakeInfoAfter.unwrap().stake).to.be.bignumber.equal(new BN(STAKE_AMOUNT));
      amountAfterStake = stakeInfoAfter.unwrap().stake;
      expect(stakeInfoAfter.unwrap().share).to.be.bignumber.equal(new BN(STAKE_AMOUNT));
      expect(stakeInfoAfter.unwrap().lock.unlockPenalty).to.be.bignumber.equal(new BN(UNLOCK_PENALTY));

      // Checking funds
      const userFundsAfter = await api.rpc.assets.balanceOf(POOL_1_BASE_ASSET_ID.toString(), walletStaker.publicKey);
      expect(new BN(userFundsAfter.toString())).to.be.bignumber.lessThan(
        new BN(userFundsBefore.toString()).add(new BN(STAKE_AMOUNT))
      );
    });

    it("User can stake in the second created reward pool", async function () {
      this.timeout(2 * 60 * 1000);
      // Getting funds before transaction
      const userFundsBefore = await api.rpc.assets.balanceOf(POOL_2_BASE_ASSET_ID.toString(), walletStaker.publicKey);
      // Parameters
      const durationPreset = 2592000;

      // Transaction
      const {
        data: [
          resultPoolId,
          resultOwnerAccountId,
          resultAmount,
          resultDurationPreset,
          resultFNFTCollectionId,
          resultFNFTInstanceId,
          resultKeepAlive
        ]
      } = await sendAndWaitForSuccess(
        api,
        walletStaker2,
        api.events.stakingRewards.Staked.is,
        api.tx.stakingRewards.stake(poolId2, STAKE_AMOUNT, durationPreset)
      );

      // Verification
      expect(resultPoolId).to.be.bignumber.equal(poolId2);
      expect(resultAmount.toString()).to.equal(STAKE_AMOUNT.toString());
      expect(resultDurationPreset.toString()).to.equal(durationPreset.toString());
      fNFTCollectionId3 = resultFNFTCollectionId;
      fNFTInstanceId3 = resultFNFTInstanceId;

      // Comparing with query
      const stakeInfoAfter = <Option<ComposableTraitsStakingStake>>(
        await api.query.stakingRewards.stakes(fNFTCollectionId2, "")
      );
      console.debug(stakeInfoAfter.toHuman());
      expect(stakeInfoAfter.unwrap().stake).to.be.bignumber.equal(new BN(STAKE_AMOUNT));
      amountAfterStake = stakeInfoAfter.unwrap().stake;
      expect(stakeInfoAfter.unwrap().share).to.be.bignumber.equal(new BN(STAKE_AMOUNT));
      expect(stakeInfoAfter.unwrap().lock.unlockPenalty).to.be.bignumber.equal(new BN(UNLOCK_PENALTY));

      // Checking funds
      const userFundsAfter = await api.rpc.assets.balanceOf(POOL_1_BASE_ASSET_ID.toString(), walletStaker.publicKey);
      expect(new BN(userFundsAfter.toString())).to.be.bignumber.lessThan(
        new BN(userFundsBefore.toString()).add(new BN(STAKE_AMOUNT))
      );
    });

    it("Users can stake in the preconfigured PICA pool", async function () {
      this.timeout(2 * 60 * 1000);
      // Getting funds before transaction
      const userFundsBefore = await api.rpc.assets.balanceOf("1", walletStaker.publicKey);
      // Parameters
      const durationPreset = 604800;
      const picaPoolId = 1;

      // Transaction
      const {
        data: [
          resultPoolId,
          resultOwnerAccountId,
          resultAmount,
          resultDurationPreset,
          resultFNFTCollectionId,
          resultFNFTInstanceId,
          resultKeepAlive
        ]
      } = await sendAndWaitForSuccess(
        api,
        walletStaker,
        api.events.stakingRewards.Staked.is,
        api.tx.stakingRewards.stake(picaPoolId, STAKE_AMOUNT, durationPreset)
      );

      // Verification
      expect(resultPoolId.toNumber()).to.equal(picaPoolId);
      expect(resultAmount.toString()).to.equal(STAKE_AMOUNT.toString());
      expect(resultDurationPreset.toString()).to.equal(durationPreset.toString());
      fNFTCollectionId4Pica = resultFNFTCollectionId;
      fNFTInstanceId4Pica = resultFNFTInstanceId;
      expect(resultKeepAlive.isTrue).to.be.true;

      // Checking queries
      const stakeInfoAfter = <Option<ComposableTraitsStakingStake>>(
        await api.query.stakingRewards.stakes(fNFTCollectionId1, "")
      );
      expect(stakeInfoAfter.unwrap().stake).to.be.bignumber.equal(new BN(STAKE_AMOUNT));
      amountAfterStake = stakeInfoAfter.unwrap().stake;
      expect(stakeInfoAfter.unwrap().share).to.be.bignumber.equal(new BN(STAKE_AMOUNT));
      expect(stakeInfoAfter.unwrap().lock.unlockPenalty).to.be.bignumber.equal(new BN(UNLOCK_PENALTY));

      // Checking funds
      const userFundsAfter = await api.rpc.assets.balanceOf("1", walletStaker.publicKey);
      expect(new BN(userFundsAfter.toString())).to.be.bignumber.lessThan(
        new BN(userFundsBefore.toString()).add(new BN(STAKE_AMOUNT))
      );
    });

    it("Users can stake in the preconfigured PBLO pool", async function () {
      this.timeout(2 * 60 * 1000);
      // Get funds before transaction
      const userFundsBefore = await api.rpc.assets.balanceOf("5", walletStaker2.publicKey);
      // Parameters
      const durationPreset = 604800;
      const pbloPoolId = 5;

      // Transaction
      const {
        data: [
          resultPoolId,
          resultOwnerAccountId,
          resultAmount,
          resultDurationPreset,
          resultFNFTCollectionId,
          resultFNFTInstanceId,
          resultKeepAlive
        ]
      } = await sendAndWaitForSuccess(
        api,
        walletStaker2,
        api.events.stakingRewards.Staked.is,
        api.tx.stakingRewards.stake(pbloPoolId, STAKE_AMOUNT, durationPreset)
      );

      // Verification
      expect(resultPoolId.toNumber()).to.equal(pbloPoolId);
      expect(resultAmount.toString()).to.equal(STAKE_AMOUNT.toString());
      expect(resultDurationPreset.toString()).to.equal(durationPreset.toString());
      fNFTCollectionId5Pblo = resultFNFTCollectionId;
      fNFTInstanceId5Pblo = resultFNFTInstanceId;
      expect(resultKeepAlive.isTrue).to.be.true;
      // Querying stake info
      const stakeInfoAfter = <Option<ComposableTraitsStakingStake>>(
        await api.query.stakingRewards.stakes(fNFTCollectionId1, "")
      );
      expect(stakeInfoAfter.unwrap().stake).to.be.bignumber.equal(new BN(STAKE_AMOUNT));
      amountAfterStake = stakeInfoAfter.unwrap().stake;
      expect(stakeInfoAfter.unwrap().share).to.be.bignumber.equal(new BN(STAKE_AMOUNT));
      expect(stakeInfoAfter.unwrap().lock.unlockPenalty).to.be.bignumber.equal(new BN(UNLOCK_PENALTY));
      // Checking funds
      const userFundsAfter = await api.rpc.assets.balanceOf("1", walletStaker2.publicKey);
      expect(new BN(userFundsAfter.toString())).to.be.bignumber.lessThan(
        new BN(userFundsBefore.toString()).add(new BN(STAKE_AMOUNT))
      );
    });
  });

  describe.only("tx.stakingRewards.claim Tests", function () {
    it("Users can claim available rewards from newly created staking rewards pool", async function () {
      this.timeout(2 * 60 * 1000);
      // Get funds before transaction
      const userFundsBefore = await api.rpc.assets.balanceOf(
        POOL_1_REWARD_ASSET_ID.toString(),
        walletStaker2.publicKey
      );
      // Transaction
      const {
        data: [resultOwner, resultFNFTCollectionId, resultFNFTInstanceId]
      } = await sendAndWaitForSuccess(
        api,
        walletStaker,
        api.events.stakingRewards.Claimed.is,
        api.tx.stakingRewards.claim(fNFTCollectionId1, fNFTInstanceId1)
      );

      // Verification
      expect(resultOwner.toString()).to.be.equal(api.createType("AccountId32", walletStaker.publicKey).toString());
      expect(resultFNFTCollectionId.toString()).to.equal(fNFTCollectionId1.toString());
      expect(resultFNFTInstanceId.toString()).to.equal(fNFTInstanceId1.toString());

      // Comparing with data from Query
      const stakeInfoAfter = <Option<ComposableTraitsStakingStake>>(
        await api.query.stakingRewards.stakes(fNFTCollectionId1, "")
      );
      console.debug(stakeInfoAfter.toString());

      // Checking funds
      const userFundsAfter = await api.rpc.assets.balanceOf(POOL_1_REWARD_ASSET_ID.toString(), walletStaker2.publicKey);
      console.debug(userFundsAfter.toString());
      console.debug(new BN(userFundsAfter.toString()).eq(new BN(stakeInfoAfter.unwrap().reductions['10001'])));
      expect(new BN(userFundsAfter.toString())).to.be.bignumber.lessThan(
        new BN(userFundsBefore.toString()).add(new BN(STAKE_AMOUNT))
      );
    });
  });

  describe("tx.stakingRewards.extend Tests", function () {
    it("User should be able to extend their position", async function () {
      this.timeout(2 * 60 * 1000);
      // Getting funds before
      const userFundsBefore = await api.rpc.assets.balanceOf(POOL_1_BASE_ASSET_ID.toString(), walletStaker.publicKey);
      // Parameters
      const amount = STAKE_AMOUNT * 2;

      // Transaction
      const {
        data: [resultFNFTCollectionId, resultFNFTInstanceId, resultAmount]
      } = await sendAndWaitForSuccess(
        api,
        walletStaker,
        api.events.stakingRewards.StakeAmountExtended.is,
        api.tx.stakingRewards.extend(fNFTCollectionId1, fNFTInstanceId1, amount)
      );

      // Verification
      expect(resultFNFTCollectionId).to.be.bignumber.equal(fNFTCollectionId1);
      expect(resultFNFTInstanceId).to.be.bignumber.equal(fNFTInstanceId1);
      expect(resultAmount.toString()).to.equal(amount.toString());
      // Querying stake
      const stakeInfoAfter = <Option<ComposableTraitsStakingStake>>(
        await api.query.stakingRewards.stakes(fNFTCollectionId1, "")
      );
      console.debug(stakeInfoAfter.toHuman());
      stakeAmountAfterExtending = stakeInfoAfter.unwrap().stake;
      expect(stakeInfoAfter.unwrap().stake).to.be.bignumber.equal(new BN(amount).add(amountAfterStake));
      // ToDo if still the case if multiple stakers!
      expect(stakeInfoAfter.unwrap().share).to.be.bignumber.equal(new BN(amount).add(amountAfterStake));
      // Checking funds
      const userFundsAfter = await api.rpc.assets.balanceOf(POOL_1_BASE_ASSET_ID.toString(), walletStaker.publicKey);
      expect(new BN(userFundsAfter.toString())).to.be.bignumber.lessThan(
        new BN(userFundsBefore.toString()).add(new BN(amount))
      );
    });
  });

  describe("tx.stakingRewards.split Tests", function () {
    it("User can split their position", async function () {
      this.timeout(2 * 60 * 1000);
      // Transaction
      const {
        data: [resultPositions]
      } = await sendAndWaitForSuccess(
        api,
        walletStaker,
        api.events.stakingRewards.SplitPosition.is,
        api.tx.stakingRewards.split(fNFTCollectionId1, fNFTInstanceId1, 500_000)
      );

      // Verification
      expect(resultPositions.length).to.be.equal(2);
      // Querying stake info
      const stakeInfoAfter = <Option<ComposableTraitsStakingStake>>(
        await api.query.stakingRewards.stakes(fNFTCollectionId1, "")
      );
      console.debug(stakeInfoAfter.toHuman());
      expect(stakeInfoAfter.unwrap().stake).to.be.bignumber.equal(stakeAmountAfterExtending.muln(0.5));
      expect(stakeInfoAfter.unwrap().share).to.be.bignumber.equal(stakeAmountAfterExtending.muln(0.5));
      expect(stakeInfoAfter.unwrap().lock.unlockPenalty).to.be.bignumber.equal(new BN(UNLOCK_PENALTY));
    });
  });

  describe("tx.stakingRewards.unstake Tests", function () {
    it("User should be able to unstake funds from pool before it has ended and get slashed", async function () {
      this.timeout(2 * 60 * 1000);
      // Getting user funds before
      const userFundsBefore = await api.rpc.assets.balanceOf(POOL_2_BASE_ASSET_ID.toString(), walletStaker.publicKey);

      // Transaction
      const {
        data: [resultPoolId, resultOwner, resultFNFTInstanceId, resultRewardAssetId, resultAmountSlashed]
      } = await sendAndWaitForSuccess(
        api,
        walletStaker2,
        api.events.stakingRewards.UnstakeRewardSlashed.is,
        api.tx.stakingRewards.unstake(fNFTCollectionId2, fNFTInstanceId2)
      );

      // Verification
      console.debug(resultPoolId.toHuman());
      console.debug(resultRewardAssetId.toHuman());
      expect(resultOwner.toString()).to.be.equal(api.createType("AccountId32", walletStaker2.publicKey).toString());
      expect(resultFNFTInstanceId).to.be.bignumber.equal(fNFTInstanceId2);
      expect(resultAmountSlashed).to.be.bignumber.greaterThan(new BN(0));

      // Expecting wallets stake to return nothing.
      const stakeInfoAfter = await api.query.stakingRewards.stakes(fNFTCollectionId2, "").catch(function (e) {
        return e;
      });
      expect(stakeInfoAfter.toString()).to.equal("");

      // Checking user funds
      const userFundsAfter = await api.rpc.assets.balanceOf(POOL_2_BASE_ASSET_ID.toString(), walletStaker2.publicKey);
      expect(new BN(userFundsAfter.toString())).to.be.bignumber.greaterThan(new BN(userFundsBefore.toString()));
    });

    it("User should be able to unstake funds from pool", async function () {
      this.timeout(4 * 60 * 1000);
      // Waiting a few blocks to safely unstake funds.
      await waitForBlocks(api, 6);
      // Getting funds before
      const userFundsBefore = await api.rpc.assets.balanceOf(POOL_1_BASE_ASSET_ID.toString(), walletStaker.publicKey);

      // Transaction
      const {
        data: [resultAccountId, resultPositionId]
      } = await sendAndWaitForSuccess(
        api,
        walletStaker,
        api.events.stakingRewards.Unstaked.is,
        api.tx.stakingRewards.unstake(fNFTCollectionId1, fNFTInstanceId1)
      );
      // Verification
      expect(resultAccountId.toString()).to.be.equal(api.createType("AccountId32", walletStaker.publicKey).toString());
      expect(resultPositionId).to.be.bignumber.equal(fNFTCollectionId1);

      // Getting wallets stake should return nothing.
      const stakeInfoAfter = await api.query.stakingRewards.stakes(fNFTCollectionId1, "").catch(function (e) {
        return e;
      });
      expect(stakeInfoAfter.toString()).to.equal("");

      // Checking wallets funds
      const userFundsAfter = await api.rpc.assets.balanceOf(POOL_1_BASE_ASSET_ID.toString(), walletStaker.publicKey);
      expect(new BN(userFundsAfter.toString())).to.be.bignumber.greaterThan(new BN(userFundsBefore.toString()));
    });

    it("User should be able to unstake funds from PICA pool", async function () {
      this.timeout(4 * 60 * 1000);
      // Getting funds before
      const userFundsBefore = await api.rpc.assets.balanceOf("1", walletStaker.publicKey);
      // Parameters
      /*const poolInfo = await api.query.stakingRewards.rewardPools(1);
      const shareAssetId = poolInfo.unwrap()["shareAssetId"];
      const financialNftAssetId = poolInfo.unwrap()["financialNftAssetId"];*/
      const fNFTCollectionId = fNFTCollectionId4Pica;
      const fNFTInstanceId = fNFTInstanceId4Pica;

      // Transaction
      const {
        data: [resultOwner, resultFNFTCollectionId, resultFNFTInstanceId]
      } = await sendAndWaitForSuccess(
        api,
        walletStaker,
        api.events.stakingRewards.Unstaked.is,
        api.tx.stakingRewards.unstake(fNFTCollectionId, fNFTInstanceId)
      );

      // Verification
      expect(resultOwner.toString()).to.be.equal(api.createType("AccountId32", walletStaker.publicKey).toString());
      expect(resultFNFTCollectionId).to.be.bignumber.equal(fNFTCollectionId);
      expect(resultFNFTInstanceId).to.be.bignumber.equal(fNFTInstanceId);

      // Expecting query for wallets stake to return nothing.
      const stakeInfoAfter = await api.query.stakingRewards.stakes(fNFTCollectionId1, "").catch(function (e) {
        return e;
      });
      expect(stakeInfoAfter.toString()).to.equal("");

      // Checking funds
      const userFundsAfter = await api.rpc.assets.balanceOf("1", walletStaker.publicKey);
      expect(new BN(userFundsAfter.toString())).to.be.bignumber.greaterThan(new BN(userFundsBefore.toString()));
    });

    it("User should be able to unstake funds from PBLO pool", async function () {
      this.timeout(4 * 60 * 1000);
      // Getting funds before
      const userFundsBefore = await api.rpc.assets.balanceOf("5", walletStaker2.publicKey);
      // Parameters
      /*const poolInfo = await api.query.stakingRewards.rewardPools(5);
      const shareAssetId = poolInfo.unwrap()["shareAssetId"];
      const financialNftAssetId = poolInfo.unwrap()["financialNftAssetId"];*/
      const fNFTCollectionId = fNFTCollectionId5Pblo;
      const fNFTInstanceId = fNFTInstanceId5Pblo;

      // Transaction
      const {
        data: [resultOwner, resultFNFTCollectionId, resultFNFTInstanceId]
      } = await sendAndWaitForSuccess(
        api,
        walletStaker2,
        api.events.stakingRewards.Unstaked.is,
        api.tx.stakingRewards.unstake(fNFTCollectionId, fNFTInstanceId)
      );

      // Verification
      expect(resultOwner.toString()).to.be.equal(api.createType("AccountId32", walletStaker2.publicKey).toString());
      expect(resultFNFTCollectionId).to.be.bignumber.equal(fNFTCollectionId);
      expect(resultFNFTInstanceId).to.be.bignumber.equal(fNFTInstanceId);

      // Expecting stake to not exist
      const stakeInfoAfter = await api.query.stakingRewards.stakes(resultFNFTCollectionId, "").catch(function (e) {
        return e;
      });
      expect(stakeInfoAfter.toString()).to.contain("");

      // Checking funds
      const userFundsAfter = await api.rpc.assets.balanceOf("5", walletStaker2.publicKey);
      expect(new BN(userFundsAfter.toString())).to.be.bignumber.greaterThan(new BN(userFundsBefore.toString()));
    });
  });

  describe("tx.stakingRewards.updateRewardsPool After End Tests", function () {
    it("Pool owner can update pool configuration", async function () {
      this.timeout(2 * 60 * 1000);
      // Parameters
      const poolInfoBefore = <Option<ComposableTraitsStakingRewardPool>>(
        await api.query.stakingRewards.rewardPools(poolId1)
      );
      const rewardUpdates = api.createType("BTreeMap<u128, ComposableTraitsStakingRewardUpdate>", {
        "10001": {
          rewardRate: {
            period: {
              PerSecond: "100000"
            },
            amount: "1000000000000000"
          }
        }
      });

      // Transaction
      const {
        data: [resultPoolId]
      } = await sendAndWaitForSuccess(
        api,
        sudoKey,
        api.events.stakingRewards.RewardPoolUpdated.is,
        api.tx.sudo.sudo(api.tx.stakingRewards.updateRewardsPool(poolId1, rewardUpdates))
      );

      // Verification
      expect(resultPoolId).to.be.bignumber.equal(poolId1);
      // Querying pool info
      const poolInfo = <Option<ComposableTraitsStakingRewardPool>>await api.query.stakingRewards.rewardPools(poolId1);
      expect(poolInfo.unwrap().owner.toString()).to.equal(
        api.createType("AccountId32", walletOwner.publicKey).toString()
      );
      expect(poolInfo.unwrap().assetId.toString()).to.equal(POOL_1_BASE_ASSET_ID.toString());
      // ToDo (D.Roth): Change comparison to amount from above.
      console.debug(poolInfo.unwrap().toHuman());
      /*expect(poolInfo.unwrap().rewards[0]["rewardRate"]["amount"]).to.be.greaterThan(
        poolInfoBefore.unwrap().rewards[0]["rewardRate"]["amount"]
      );*/
    });
  });
});
