const path = require("path");
const k = require("@metaplex-foundation/kinobi");

// Paths.
const clientDir = path.join(__dirname, "..", "clients");
const idlDir = path.join(__dirname, "..", "idls");

//--- Asset program.

// Instanciate Kinobi.
const kAsset = k.createFromIdls([path.join(idlDir, "asset_program.json")]);

// Update programs.
kAsset.update(
  new k.UpdateProgramsVisitor({
    assetProgram: { name: "asset" },
  })
);

// Add missing types from the IDL.
kAsset.update(
  new k.TransformNodesVisitor([
    {
      selector: { kind: "programNode", name: "asset" },
      transformer: (node) => {
        k.assertProgramNode(node);
        return k.programNode({
          ...node,
          accounts: [
            ...node.accounts,
            // metadata account
            k.accountNode({
              name: "asset",
              data: k.accountDataNode({
                name: "assetAccountData",
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
                    name: "standard",
                    child: k.linkTypeNode("Standard"),
                  }),
                  k.structFieldTypeNode({
                    name: "mutable",
                    child: k.boolTypeNode(),
                  }),
                  k.structFieldTypeNode({
                    name: "holder",
                    child: k.publicKeyTypeNode(),
                  }),
                  k.structFieldTypeNode({
                    name: "group",
                    child: k.publicKeyTypeNode(),
                  }),
                  k.structFieldTypeNode({
                    name: "authority",
                    child: k.publicKeyTypeNode(),
                  }),
                  k.structFieldTypeNode({
                    name: "delegate",
                    child: k.linkTypeNode("Delegate"),
                  }),
                  k.structFieldTypeNode({
                    name: "name",
                    child: k.stringTypeNode({ size: k.fixedSize(35) }),
                  }),
                ]),
              }),
            }),
          ],
          definedTypes: [
            ...node.definedTypes,
            // discriminator
            k.definedTypeNodeFromIdl({
              name: "discriminator",
              type: {
                kind: "enum",
                variants: [{ name: "Uninitialized" }, { name: "Asset" }],
              },
            }),
            // standard
            k.definedTypeNodeFromIdl({
              name: "standard",
              type: {
                kind: "enum",
                variants: [{ name: "NonFungible" }, { name: "Subscription" }],
              },
            }),
            // state
            k.definedTypeNodeFromIdl({
              name: "state",
              type: {
                kind: "enum",
                variants: [{ name: "Unlocked" }, { name: "Locked" }],
              },
            }),
            // delegate role
            k.definedTypeNodeFromIdl({
              name: "delegateRole",
              type: {
                kind: "enum",
                variants: [
                  { name: "None" },
                  { name: "Transfer" },
                  { name: "Lock" },
                  { name: "Burn" },
                ],
              },
            }),
            // extension type
            k.definedTypeNodeFromIdl({
              name: "extensionType",
              type: {
                kind: "enum",
                variants: [
                  { name: "None" },
                  { name: "Attributes" },
                  { name: "Blob" },
                  { name: "Creators" },
                  { name: "Links" },
                ],
              },
            }),
            // delegate
            k.definedTypeNode({
              name: "delegate",
              data: k.structTypeNode([
                k.structFieldTypeNode({
                  name: "address",
                  child: k.publicKeyTypeNode(),
                }),
                k.structFieldTypeNode({
                  name: "roles",
                  child: k.numberTypeNode("u8"),
                }),
              ]),
            }),
            // extension header
            k.definedTypeNode({
              name: "extensionHeader",
              data: k.structTypeNode([
                k.structFieldTypeNode({
                  name: "kind",
                  child: k.numberTypeNode("u32"),
                }),
                k.structFieldTypeNode({
                  name: "length",
                  child: k.numberTypeNode("u32"),
                }),
                k.structFieldTypeNode({
                  name: "boundary",
                  child: k.numberTypeNode("u32"),
                }),
                k.structFieldTypeNode({
                  name: "padding",
                  child: k.numberTypeNode("u32"),
                }),
              ]),
              internal: true,
            }),
            // attributes
            k.definedTypeNode({
              name: "attributes",
              data: k.structTypeNode([
                k.structFieldTypeNode({
                  name: "traits",
                  child: k.arrayTypeNode(k.linkTypeNode("trait"), {
                    size: k.remainderSize(),
                  }),
                }),
              ]),
            }),
            // trait
            k.definedTypeNode({
              name: "trait",
              data: k.structTypeNode([
                k.structFieldTypeNode({
                  name: "traitType",
                  child: k.stringTypeNode({
                    size: k.prefixedSize(k.numberTypeNode("u8")),
                  }),
                }),
                k.structFieldTypeNode({
                  name: "value",
                  child: k.stringTypeNode({
                    size: k.prefixedSize(k.numberTypeNode("u8")),
                  }),
                }),
              ]),
            }),
            // blob
            k.definedTypeNode({
              name: "blob",
              data: k.structTypeNode([
                k.structFieldTypeNode({
                  name: "contentType",
                  child: k.stringTypeNode({
                    size: k.prefixedSize(k.numberTypeNode("u8")),
                  }),
                }),
                k.structFieldTypeNode({
                  name: "data",
                  child: k.arrayTypeNode(k.numberTypeNode("u8"), {
                    size: k.remainderSize(),
                  }),
                }),
              ]),
            }),
            // links
            k.definedTypeNode({
              name: "links",
              data: k.structTypeNode([
                k.structFieldTypeNode({
                  name: "values",
                  child: k.arrayTypeNode(k.linkTypeNode("link"), {
                    size: k.remainderSize(),
                  }),
                }),
              ]),
            }),
            // link
            k.definedTypeNode({
              name: "link",
              data: k.structTypeNode([
                k.structFieldTypeNode({
                  name: "name",
                  child: k.stringTypeNode({
                    size: k.prefixedSize(k.numberTypeNode("u8")),
                  }),
                }),
                k.structFieldTypeNode({
                  name: "uri",
                  child: k.stringTypeNode({
                    size: k.prefixedSize(k.numberTypeNode("u8")),
                  }),
                }),
              ]),
            }),
            // creators
            k.definedTypeNode({
              name: "creators",
              data: k.structTypeNode([
                k.structFieldTypeNode({
                  name: "creators",
                  child: k.arrayTypeNode(k.linkTypeNode("creator"), {
                    size: k.fixedSize(5),
                  }),
                }),
              ]),
            }),
            // creator
            k.definedTypeNode({
              name: "creator",
              data: k.structTypeNode([
                k.structFieldTypeNode({
                  name: "address",
                  child: k.publicKeyTypeNode(),
                }),
                k.structFieldTypeNode({
                  name: "verified",
                  child: k.boolTypeNode(),
                }),
                k.structFieldTypeNode({
                  name: "share",
                  child: k.numberTypeNode("u8"),
                }),
                k.structFieldTypeNode({
                  name: "padding",
                  child: k.bytesTypeNode(k.fixedSize(6)),
                }),
              ]),
            }),
          ],
        });
      },
    },
  ])
);

// Update instructions.
kAsset.update(
  new k.UpdateInstructionsVisitor({
    create: {
      accounts: {
        holder: { defaultsTo: k.identityDefault() },
        systemProgram: {
          defaultsTo: k.conditionalDefault("account", "payer", {
            ifTrue: k.programDefault(
              "systemProgram",
              "11111111111111111111111111111111"
            ),
          }),
        },
      },
    },
    allocate: {
      accounts: {
        systemProgram: {
          defaultsTo: k.conditionalDefault("account", "payer", {
            ifTrue: k.programDefault(
              "systemProgram",
              "11111111111111111111111111111111"
            ),
          }),
        },
      },
    },
  })
);

// Set default values.
kAsset.update(
  new k.SetStructDefaultValuesVisitor({
    initialize: {
      data: k.vNone(),
    },
    CreateInstructionData: {
      standard: k.vEnum("Standard", "NonFungible"),
      mutable: k.vScalar(true),
    },
  })
);

// Set ShankAccount discriminator.
const assetKey = (name) => ({
  field: "discriminator",
  value: k.vEnum("Discriminator", name),
});
kAsset.update(
  new k.SetAccountDiscriminatorFromFieldVisitor({
    asset: assetKey("Asset"),
  })
);

// Use custom serializers.
kAsset.update(
  new k.UseCustomAccountSerializerVisitor({
    asset: { extract: true },
  })
);

// Render JavaScript.
kAsset.accept(
  new k.RenderJavaScriptVisitor(
    path.join(clientDir, "asset", "js", "src", "generated"),
    {
      prettier: require(path.join(
        clientDir,
        "asset",
        "js",
        ".prettierrc.json"
      )),
    }
  )
);

// Render Rust.
kAsset.accept(
  new k.RenderRustVisitor(
    path.join(clientDir, "asset", "rust", "src", "generated"),
    {
      formatCode: true,
      crateFolder: path.join(clientDir, "asset", "rust"),
    }
  )
);

//--- Bridge program.

const kBridge = k.createFromIdls([path.join(idlDir, "bridge_program.json")]);

// Update programs.
kBridge.update(
  new k.UpdateProgramsVisitor({
    bridgeProgram: { name: "bridge" },
  })
);

// Add missing types from the IDL.
kBridge.update(
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
kBridge.update(
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
kBridge.update(
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
kBridge.update(
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
kBridge.update(
  new k.SetAccountDiscriminatorFromFieldVisitor({
    vault: bridgeKey("Vault"),
  })
);

// Render JavaScript.
kBridge.accept(
  new k.RenderJavaScriptVisitor(
    path.join(clientDir, "bridge", "js", "src", "generated"),
    {
      prettier: require(path.join(
        clientDir,
        "bridge",
        "js",
        ".prettierrc.json"
      )),
      dependencyMap: {
        mplTokenMetadata: "@metaplex-foundation/mpl-token-metadata",
      },
    }
  )
);

// Render Rust.
kBridge.accept(
  new k.RenderRustVisitor(
    path.join(clientDir, "bridge", "rust", "src", "generated"),
    {
      formatCode: true,
      crateFolder: path.join(clientDir, "bridge", "rust"),
    }
  )
);
