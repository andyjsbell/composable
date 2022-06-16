import BigNumber from "bignumber.js";
import { Token } from "../../defi/types";
import type { AccountId32 } from "@polkadot/types/interfaces/runtime";

export interface BondSlice {
  allBonds: AllBond[];
  activeBonds: ActiveBond[];
  addActiveBond: (
    bondOffer: BondOffer,
    vestingSchedule: VestingSchedule,
    currentBlock: number,
    currentTime: number
  ) => void;
  addBond: (
    bondOffer: BondOffer,
    principalAppoloPriceInUSD: number,
    rewardAppoloPriceInUSD: number
  ) => void;
  reset: () => void;
}

export interface BondOffer {
  offerId: number;
  beneficiary: AccountId32;
  currencyId: number;
  asset: Token | "string";
  bondPrice: BigNumber;
  nbOfBonds: number;
  maturity: number | "Infinite";
  reward: OfferReward;
}

export interface VestingSchedule {
  perPeriod: BigNumber;
  periodCount: number;
  window: Window;
  type: "block" | "moment";
}

type Window = { start: number; period: number };

interface OfferReward {
  currencyId: number;
  asset: Token | "string";
  amount: BigNumber;
  maturity: number;
}

type ActiveBond = {
  offerId: number;
  asset: Token | "string";
  pendingAmount: BigNumber;
  claimableAmount: BigNumber;
  vestingTime: string;
  bondOffer: BondOffer;
};

type AllBond = {
  offerId: number;
  asset: Token | "string";
  price: BigNumber;
  roi: BigNumber;
  totalPurchased: BigNumber;
  bondOffer: BondOffer;
};