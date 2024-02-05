const path = require("path");
const k = require("@metaplex-foundation/kinobi");

// Paths.
const clientDir = path.join(__dirname, "..", "clients");
const idlDir = path.join(__dirname, "..", "idls");

const kinobi = k.createFromIdls([path.join(idlDir, "bridge_program.json")]);

// Update programs.
kinobi.update(
  new k.UpdateProgramsVisitor({
    bridgeProgram: { name: "bridge" },
  })
);

// Add missing types from the IDL.
kinobi.update(
  new k.TransformNodesVisitor([
    {
      selector: { kind: "programNode", name: "bridge" },
      transformer: (node) => {
        k.assertProgramNode(node);
        return k.programNode({
          ...node,
          accounts: [
            ...node.accounts,
            // metadata account
            k.accountNode({
              name: "vault",
              data: k.accountDataNode({
                name: "vaultAccountData",
                struct: k.structTypeNode([
                  k.structFieldTypeNode({
                    name: "discriminator",
                    child: k.linkTypeNode("Discriminator"),
                  }),
                  k.structFieldTypeNode({
                    name: "state",
                    child: k.linkTypeNode("State"),
                  }),
                  k.structFieldTypeNode({
                    name: "bump",
                    child: k.numberTypeNode("u8"),
                  }),
                  k.structFieldTypeNode({
                    name: "mint",
                    child: k.publicKeyTypeNode(),
                  }),
                  k.structFieldTypeNode({
                    name: "assetBump",
                    child: k.numberTypeNode("u8"),
                  }),
                ]),
              }),
            }),
          ],
        });
      },
    },
  ])
);

// Update accounts.
kinobi.update(
  new k.UpdateAccountsVisitor({
    vault: {
      seeds: [
        k.stringConstantSeed("nifty::bridge::vault"),
        k.publicKeySeed("mint", "The address of the mint"),
      ],
    },
  })
);

// Set default account values accross multiple instructions.
kinobi.update(
  new k.SetInstructionAccountDefaultValuesVisitor([
    // default accounts
    {
      account: "vault",
      ...k.pdaDefault("vault"),
    },
    {
      account: "updateAuthority",
      ignoreIfOptional: true,
      ...k.identityDefault(),
    },
    {
      account: "metadata",
      ...k.pdaDefault("metadata", {
        importFrom: "mplTokenMetadata",
        seeds: { mint: k.accountDefault("mint") },
      }),
    },
    {
      account: "niftyAssetProgram",
      ignoreIfOptional: true,
      ...k.programDefault(
        "niftyAsset",
        "AssetGtQBTSgm5s91d1RAQod5JmaZiJDxqsgtqrZud73"
      ),
    },
  ])
);

// Update instructions.
kinobi.update(
  new k.UpdateInstructionsVisitor({
    create: {
      accounts: {
        asset: {
          defaultsTo: k.resolverDefault("resolveBridgeAsset", [
            k.dependsOnAccount("mint"),
          ]),
        },
      },
      args: {
        version: {
          type: k.numberTypeNode("u8"),
          defaultsTo: k.valueDefault(k.vScalar(1)),
        },
      },
    },
    bridge: {
      accounts: {
        asset: {
          defaultsTo: k.resolverDefault("resolveBridgeAsset", [
            k.dependsOnAccount("mint"),
          ]),
        },
        vault: { defaultsTo: k.pdaDefault("vault") },
        tokenOwner: { defaultsTo: k.identityDefault() },
        token: {
          defaultsTo: k.pdaDefault("associatedToken", {
            importFrom: "mplToolbox",
            seeds: {
              mint: k.accountDefault("mint"),
              owner: k.accountDefault("owner"),
            },
          }),
        },
        metadata: {
          defaultsTo: k.pdaDefault("metadata", {
            importFrom: "mplTokenMetadata",
            seeds: { mint: k.accountDefault("mint") },
          }),
        },
        masterEdition: {
          defaultsTo: k.pdaDefault("masterEdition", {
            importFrom: "mplTokenMetadata",
            seeds: { mint: k.accountDefault("mint") },
          }),
        },
        tokenRecord: {
          defaultsTo: k.conditionalDefault(
            "arg",
            "tokenStandard",
            { value: k.vEnum("TokenStandard", "ProgrammableNonFungible") },
            {
              ifTrue: k.pdaDefault("tokenRecord", {
                importFrom: "mplTokenMetadata",
                seeds: {
                  mint: k.accountDefault("mint"),
                  token: k.accountDefault("token"),
                },
              }),
            }
          ),
        },
        vaultToken: {
          defaultsTo: k.pdaDefault("associatedToken", {
            importFrom: "mplToolbox",
            seeds: {
              mint: k.accountDefault("mint"),
              owner: k.accountDefault("vault"),
            },
          }),
        },
        vaultTokenRecord: {
          defaultsTo: k.conditionalDefault(
            "arg",
            "tokenStandard",
            { value: k.vEnum("TokenStandard", "ProgrammableNonFungible") },
            {
              ifTrue: k.pdaDefault("tokenRecord", {
                importFrom: "mplTokenMetadata",
                seeds: {
                  mint: k.accountDefault("mint"),
                  token: k.accountDefault("vaultToken"),
                },
              }),
            }
          ),
        },
      },
      args: {
        tokenStandard: {
          type: k.linkTypeNode("TokenStandard", {
            importFrom: "mplTokenMetadata",
          }),
          defaultsTo: k.valueDefault(
            k.vEnum(
              "TokenStandard",
              "NonFungible",
              undefined,
              "mplTokenMetadata"
            )
          ),
        },
      },
    },
  })
);

// Set ShankAccount discriminator.
const bridgeKey = (name) => ({
  field: "discriminator",
  value: k.vEnum("Discriminator", name),
});
kinobi.update(
  new k.SetAccountDiscriminatorFromFieldVisitor({
    vault: bridgeKey("Vault"),
  })
);

// Render JavaScript.
kinobi.accept(
  new k.RenderJavaScriptVisitor(
    path.join(clientDir, "js", "bridge", "src", "generated"),
    {
      prettier: require(path.join(
        clientDir,
        "js",
        "bridge",
        ".prettierrc.json"
      )),
      dependencyMap: {
        mplTokenMetadata: "@metaplex-foundation/mpl-token-metadata",
      },
    }
  )
);

// Render Rust.
kinobi.accept(
  new k.RenderRustVisitor(
    path.join(clientDir, "rust", "bridge", "src", "generated"),
    {
      formatCode: true,
      crateFolder: path.join(clientDir, "rust", "bridge"),
    }
  )
);
