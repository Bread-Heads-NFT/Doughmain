import { UmiPlugin } from '@metaplex-foundation/umi';
import { createBglDoughProgram } from './generated';

export const bglDough = (): UmiPlugin => ({
  install(umi) {
    umi.programs.add(createBglDoughProgram(), false);
  },
});
