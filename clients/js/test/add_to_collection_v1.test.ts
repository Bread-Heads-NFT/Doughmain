import { generateSigner } from '@metaplex-foundation/umi';
import test from 'ava';
import { CollectionV1, createCollection, ExternalPluginAdapterSchema, fetchCollection } from '@metaplex-foundation/mpl-core';
import { addToCollectionV1, PROGRAM_SIGNER } from '../src';
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
});
