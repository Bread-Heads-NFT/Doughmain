/* eslint-disable import/no-extraneous-dependencies */
import { createUmi as basecreateUmi } from '@metaplex-foundation/umi-bundle-tests';
import { mplTokenMetadata } from '@metaplex-foundation/mpl-token-metadata';
import { mplToolbox } from '@metaplex-foundation/mpl-toolbox';
import { bglDough } from '../src';

export const createUmi = async () =>
  (await basecreateUmi()).use(bglDough()).use(mplTokenMetadata()).use(mplToolbox());
