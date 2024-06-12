import * as mainstay from "@nx-pkg/mainstay";
import { BN, Program, web3 } from "@nx-pkg/mainstay";
import { assert } from "chai";
import { createPriceFeed, setFeedPrice, getFeedData } from "./oracleUtils";

describe("pyth-oracle", () => {
  const provider = mainstay.MainstayProvider.env();
  mainstay.setProvider(provider);
  const program = mainstay.workspace.Pyth as Program;

  it("initialize", async () => {
    const price = 50000;
    const priceFeedAddress = await createPriceFeed({
      oracleProgram: program,
      initPrice: price,
      expo: -6,
      provider,
    });
    const feedData = await getFeedData(program, priceFeedAddress);
    assert.strictEqual(feedData.price, price);
  });

  it("change feed price", async () => {
    const price = 50000;
    const expo = -7;
    const priceFeedAddress = await createPriceFeed({
      oracleProgram: program,
      initPrice: price,
      expo: expo,
      provider,
    });
    const feedDataBefore = await getFeedData(program, priceFeedAddress);
    assert.strictEqual(feedDataBefore.price, price);
    assert.strictEqual(feedDataBefore.exponent, expo);

    const newPrice = 55000;
    await setFeedPrice(program, newPrice, priceFeedAddress);
    const feedDataAfter = await getFeedData(program, priceFeedAddress);
    assert.strictEqual(feedDataAfter.price, newPrice);
    assert.strictEqual(feedDataAfter.exponent, expo);
  });
});
