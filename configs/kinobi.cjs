const path = require("path");
const k = require("@metaplex-foundation/kinobi");

// Paths.
const clientDir = path.join(__dirname, "..", "clients");
const idlDir = path.join(__dirname, "..", "idls");

// Instantiate Kinobi.
const kinobi = k.createFromIdls([path.join(idlDir, "bgl_dough_program.json")]);

// Update programs.
kinobi.update(
  new k.updateProgramsVisitor({
    bglDoughProgram: { name: "bglDough" },
  })
);

// Appends a custom account to the program.
kinobi.update(
  new k.bottomUpTransformerVisitor([{
    select: ["[programNode]", node => 'name' in node && node.name === "bglDough"],
    transform: (node) => {
      console.log(node);
      return k.programNode({
        ...node,
        accounts: [
          ...node.accounts,
          k.accountNode({
            name: "escrow",
            data: k.structTypeNode([
              k.structFieldTypeNode({
                name: "data",
                type: k.bytesTypeNode(k.remainderSizeNode()),
              })
            ]),
          })
        ],
      });
    }
  }])
)

// // Update accounts.
// kinobi.update(
//   new k.updateAccountsVisitor({
//     myPdaAccount: {
//       seeds: [
//         k.constantPdaSeedNodeFromString("myPdaAccount"),
//         k.programIdPdaSeedNode(),
//         k.variablePdaSeedNode("authority", k.publicKeyTypeNode(), "The address of the authority"),
//         k.variablePdaSeedNode("name", k.stringTypeNode(), "The name of the account"),
//       ],
//     },
//   })
// );

// // Update instructions.
// kinobi.update(
//   new k.updateInstructionsVisitor({
//     create: {
//       byteDeltas: [
//         k.instructionByteDeltaNode(k.accountLinkNode("myAccount")),
//       ],
//     },
//   })
// );

// // Set ShankAccount discriminator.
// const key = (name) => ({ field: "key", value: k.enumValueNode("Key", name) });
// kinobi.update(
//   new k.setAccountDiscriminatorFromFieldVisitor({
//     myAccount: key("MyAccount"),
//     myPdaAccount: key("MyPdaAccount"),
//   })
// );

// Update Accounts.
kinobi.update(
  k.updateAccountsVisitor({
    escrow: {
      seeds: [
        k.constantPdaSeedNodeFromString("escrow"),
        k.variablePdaSeedNode("asset", k.publicKeyTypeNode(), "The address of the asset"),
      ],
    },
  })
);

// Update Instructions.
let programSigner = { defaultValue: k.publicKeyValueNode("8rNE2yecH6AsLVpSPmbUE2UTCcQDhzah9rab6kW1iENy") };
const ataPdaDefault = (mint = "mint", owner = "owner") =>
  k.pdaValueNode(k.pdaLinkNode("associatedToken", "mplToolbox"), [
    k.pdaSeedValueNode("mint", k.accountValueNode(mint)),
    k.pdaSeedValueNode("owner", k.accountValueNode(owner))
  ]);
kinobi.update(
  k.updateInstructionsVisitor({
    addToAssetV1: {
      accounts: {
        programSigner,
      }
    },
    crankV1: {
      accounts: {
        programSigner,
      }
    },
    feedSplTokenV1: {
      accounts: {
        escrow: { defaultValue: k.pdaValueNode("escrow") },
        feederAta: { defaultValue: ataPdaDefault("mint", "feeder") },
        escrowAta: { defaultValue: ataPdaDefault("mint", "escrow") },
        programSigner,
        associatedTokenProgram: {
          defaultValue: k.publicKeyValueNode("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL")
        },
      }
    }
  })
);

// Render JavaScript.
const jsDir = path.join(clientDir, "js", "src", "generated");
const prettier = require(path.join(clientDir, "js", ".prettierrc.json"));
kinobi.accept(new k.renderJavaScriptVisitor(jsDir, { prettier }));

// Render Rust.
const crateDir = path.join(clientDir, "rust");
const rustDir = path.join(clientDir, "rust", "src", "generated");
kinobi.accept(
  new k.renderRustVisitor(rustDir, {
    formatCode: true,
    crateFolder: crateDir,
  })
);
