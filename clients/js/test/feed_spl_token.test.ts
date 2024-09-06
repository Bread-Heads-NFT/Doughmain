import { generateSigner, percentAmount, some } from '@metaplex-foundation/umi';
import test from 'ava';
import { AssetV1, CollectionV1, create, createCollection, DataSectionPlugin, ExternalPluginAdapterSchema, fetchAsset, fetchCollection } from '@metaplex-foundation/mpl-core';
import { createFungible } from '@metaplex-foundation/mpl-token-metadata';
import { createAssociatedToken, findAssociatedTokenPda, mintTokensTo } from '@metaplex-foundation/mpl-toolbox';
import { addToAssetV1, addToCollectionV1, crankV1, DoughData, feedSplTokenV1, getDoughDataSerializer, PROGRAM_SIGNER } from '../src';
import { createUmi } from './_setup';

test('it can feed the Dough Pets an SPL Token', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const collection = generateSigner(umi);

  // Create a collection.
  await createCollection(umi, {
    collection,
    name: 'Test Collection',
    uri: 'https://example.com/collection',
  }).sendAndConfirm(umi);

  // When we create a new account.
  await addToCollectionV1(umi, {
    collection: collection.publicKey,
  }).sendAndConfirm(umi);

  // Then an account was created with the correct data.
  t.like(await fetchCollection(umi, collection.publicKey), <CollectionV1>{
    publicKey: collection.publicKey,
    updateAuthority: umi.identity.publicKey,
    name: 'Test Collection',
    uri: 'https://example.com/collection',
    linkedAppDatas: [{
      type: 'LinkedAppData',
      authority: { type: 'UpdateAuthority', address: undefined },
      dataAuthority: { type: 'Address', address: PROGRAM_SIGNER },
      lifecycleChecks: undefined,
      offset: 103n,
      schema: ExternalPluginAdapterSchema.Binary,
    }]
  });

  // We create an asset.
  const asset = generateSigner(umi);
  await create(umi, {
    asset,
    collection: await fetchCollection(umi, collection.publicKey),
    name: "Test Asset",
    uri: "https://example.com/asset",
  }).sendAndConfirm(umi);

  // We turn the asset into a Dough Pet.
  await addToAssetV1(umi, {
    asset: asset.publicKey,
    collection: collection.publicKey,
    name: 'Doughbert',
  }).sendAndConfirm(umi);

  // Then an account was created with the correct data.
  const assetData0 = await fetchAsset(umi, asset.publicKey);
  t.like(assetData0, <AssetV1>{
    publicKey: asset.publicKey,
    updateAuthority: { type: 'Collection', address: collection.publicKey },
    name: 'Test Asset',
    uri: 'https://example.com/asset',
    owner: umi.identity.publicKey,
  });

  if (assetData0.dataSections) {
    t.like(assetData0.dataSections[0], <DataSectionPlugin>{
      type: 'DataSection',
      parentKey: { type: 'LinkedAppData', dataAuthority: { type: 'Address', address: PROGRAM_SIGNER } },
    });

    const doughData = getDoughDataSerializer().deserialize(assetData0.dataSections[0].data)[0];
    t.like(doughData, <DoughData>{ name: 'Doughbert', health: 10, happiness: 10, hunger: 10, points: 0 });
  } else {
    t.fail("No Data Sections");
  }

  // Delay 17 seconds.
  // eslint-disable-next-line no-promise-executor-return
  await new Promise((resolve) => setTimeout(resolve, 16000));

  // We crank the Dough Pet.
  await crankV1(umi, {
    asset: asset.publicKey,
    collection: collection.publicKey,
  }).sendAndConfirm(umi);

  // Then the crank properly decrements the stats.
  const assetData1 = await fetchAsset(umi, asset.publicKey);
  t.like(assetData1, <AssetV1>{
    publicKey: asset.publicKey,
    updateAuthority: { type: 'Collection', address: collection.publicKey },
    name: 'Test Asset',
    uri: 'https://example.com/asset',
    owner: umi.identity.publicKey,
  });

  if (assetData1.dataSections) {
    t.like(assetData1.dataSections[0], <DataSectionPlugin>{
      type: 'DataSection',
      parentKey: { type: 'LinkedAppData', dataAuthority: { type: 'Address', address: PROGRAM_SIGNER } },
    });

    const doughData = getDoughDataSerializer().deserialize(assetData1.dataSections[0].data)[0];
    t.like(doughData, <DoughData>{ name: 'Doughbert', health: 10, happiness: 10, hunger: 9, points: 0 });
  } else {
    t.fail("No Data Sections");
  }

  const mint = generateSigner(umi)
  await createFungible(umi, {
    mint,
    name: 'My Fungible',
    uri: 'https://example.com/my-fungible.json',
    sellerFeeBasisPoints: percentAmount(5.5),
    decimals: some(7), // for 0 decimals use some(0)
  }).sendAndConfirm(umi)

  await createAssociatedToken(umi, {
    mint: mint.publicKey,
  }).sendAndConfirm(umi);

  await mintTokensTo(umi, {
    mint: mint.publicKey,
    token: findAssociatedTokenPda(umi, {
      mint: mint.publicKey,
      owner: umi.identity.publicKey,
    }),
    amount: 100
  }).sendAndConfirm(umi);

  await feedSplTokenV1(umi, {
    asset: asset.publicKey,
    collection: collection.publicKey,
    mint: mint.publicKey,
    feeder: umi.identity.publicKey,
    amount: 1
  }).sendAndConfirm(umi);
});
