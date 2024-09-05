import { generateSigner } from '@metaplex-foundation/umi';
import test from 'ava';
import { AssetV1, CollectionV1, create, createCollection, DataSectionPlugin, ExternalPluginAdapterSchema, fetchAsset, fetchCollection } from '@metaplex-foundation/mpl-core';
import { addToAssetV1, addToCollectionV1, DoughData, getDoughDataSerializer, PROGRAM_SIGNER } from '../src';
import { createUmi } from './_setup';

test('it can create new accounts', async (t) => {
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
    programSigner: PROGRAM_SIGNER,
  }).sendAndConfirm(umi);

  // Then an account was created with the correct data.
  const assetData = await fetchAsset(umi, asset.publicKey);
  t.like(assetData, <AssetV1>{
    publicKey: asset.publicKey,
    updateAuthority: { type: 'Collection', address: collection.publicKey },
    name: 'Test Asset',
    uri: 'https://example.com/asset',
    owner: umi.identity.publicKey,
  });

  if (assetData.dataSections) {
    t.like(assetData.dataSections[0], <DataSectionPlugin>{
      type: 'DataSection',
      parentKey: { type: 'LinkedAppData', dataAuthority: { type: 'Address', address: PROGRAM_SIGNER } },
    });

    const doughData = getDoughDataSerializer().deserialize(assetData.dataSections[0].data)[0];
    t.like(doughData, <DoughData>{ name: 'Doughbert', health: 10, happiness: 10, hunger: 10 });
  } else {
    t.fail("No Data Sections");
  }
});
