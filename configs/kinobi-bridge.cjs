const anchorIdl = require("@kinobi-so/nodes-from-anchor");
const jsRenderer = require("@kinobi-so/renderers-js-umi");
const rustRendered = require("@kinobi-so/renderers-rust");
const k = require("kinobi");

// Paths.
const path = require("path");
const clientDir = path.join(__dirname, "..", "clients");
const idlDir = path.join(__dirname, "..", "idls");

// Instanciate Kinobi.
const idl = anchorIdl.rootNodeFromAnchor(
  require(path.join(idlDir, "bridge_program.json"))
);
const kinobi = k.createFromRoot(idl);

// Update programs.
kinobi.update(
  k.updateProgramsVisitor({
    bridgeProgram: { name: "bridge" },
  })
);

// Add missing types from the IDL.
kinobi.update(
  k.bottomUpTransformerVisitor([
    {
      select: "[programNode]bridge",
      transform: (node) => {
        k.assertIsNode(node, "programNode");
        return {
          ...node,
          accounts: [
            ...node.accounts,
            // vault account
            k.accountNode({
              name: "vault",
              data: k.structTypeNode([
                k.structFieldTypeNode({
                  name: "discriminator",
                  type: k.definedTypeLinkNode("Discriminator"),
                }),
                k.structFieldTypeNode({
                  name: "state",
                  type: k.definedTypeLinkNode("State"),
                }),
                k.structFieldTypeNode({
                  name: "bump",
                  type: k.numberTypeNode("u8"),
                }),
                k.structFieldTypeNode({
                  name: "mint",
                  type: k.publicKeyTypeNode(),
                }),
                k.structFieldTypeNode({
                  name: "assetBump",
                  type: k.numberTypeNode("u8"),
                }),
              ]),
            }),
          ],
        };
      },
    },
  ])
);

// Update accounts.
kinobi.update(
  k.updateAccountsVisitor({
    vault: {
      seeds: [
        k.constantPdaSeedNodeFromString("utf8", "vault"),
        k.variablePdaSeedNode(
          "mint",
          k.publicKeyTypeNode(),
          "The address of the mint account"
        ),
      ],
    },
  })
);

// Set default account values accross multiple instructions.
kinobi.update(
  k.setInstructionAccountDefaultValuesVisitor([
    // default accounts
    {
      account: "vault",
      ignoreIfOptional: true,
      defaultValue: k.pdaValueNode("vault"),
    },
    {
      account: "updateAuthority",
      ignoreIfOptional: true,
      defaultValue: k.identityValueNode(),
    },
    {
      account: "metadata",
      ignoreIfOptional: true,
      defaultValue: k.pdaValueNode(
        k.pdaLinkNode("metadata", "mplTokenMetadata"),
        [k.pdaSeedValueNode("mint", k.accountValueNode("mint"))]
      ),
    },
    {
      account: "niftyAssetProgram",
      ignoreIfOptional: true,
      defaultValue: k.publicKeyValueNode(
        "AssetGtQBTSgm5s91d1RAQod5JmaZiJDxqsgtqrZud73",
        "niftyAsset"
      ),
    },
  ])
);

// Update instructions.
kinobi.update(
  k.updateInstructionsVisitor({
    create: {
      accounts: {
        asset: {
          defaultValue: k.resolverValueNode("resolveBridgeAsset", {
            dependsOn: [k.accountValueNode("mint")],
          }),
        },
      },
    },
    bridge: {
      accounts: {
        asset: {
          defaultValue: k.resolverValueNode("resolveBridgeAsset", {
            dependsOn: [k.accountValueNode("mint")],
          }),
        },
        vault: { defaultValue: k.pdaValueNode("vault") },
        tokenOwner: { defaultValue: k.identityValueNode() },
        token: {
          defaultValue: k.pdaValueNode(
            k.pdaLinkNode("associatedToken", "mplToolbox"),
            [
              k.pdaSeedValueNode("mint", k.accountValueNode("mint")),
              k.pdaSeedValueNode("owner", k.accountValueNode("owner")),
            ]
          ),
        },
        metadata: {
          defaultValue: k.pdaValueNode(
            k.pdaLinkNode("metadata", "mplTokenMetadata"),
            [k.pdaSeedValueNode("mint", k.accountValueNode("mint"))]
          ),
        },
        masterEdition: {
          defaultValue: k.pdaValueNode(
            k.pdaLinkNode("masterEdition", "mplTokenMetadata"),
            [k.pdaSeedValueNode("mint", k.accountValueNode("mint"))]
          ),
        },
        tokenRecord: {
          defaultValue: k.conditionalValueNode({
            condition: k.argumentValueNode("tokenStandard"),
            value: k.enumValueNode(
              k.definedTypeLinkNode("TokenStandard", "mplTokenMetadata"),
              "ProgrammableNonFungible"
            ),
            ifTrue: k.pdaValueNode(
              k.pdaLinkNode("tokenRecord", "mplTokenMetadata"),
              [
                k.pdaSeedValueNode("mint", k.accountValueNode("mint")),
                k.pdaSeedValueNode("token", k.accountValueNode("token")),
              ]
            ),
          }),
        },
        vaultToken: {
          defaultValue: k.pdaValueNode(
            k.pdaLinkNode("associatedToken", "mplToolbox"),
            [
              k.pdaSeedValueNode("mint", k.accountValueNode("mint")),
              k.pdaSeedValueNode("owner", k.accountValueNode("vault")),
            ]
          ),
        },
        vaultTokenRecord: {
          defaultValue: k.conditionalValueNode({
            condition: k.argumentValueNode("tokenStandard"),
            value: k.enumValueNode(
              k.definedTypeLinkNode("TokenStandard", "mplTokenMetadata"),
              "ProgrammableNonFungible"
            ),
            ifTrue: k.pdaValueNode(
              k.pdaLinkNode("tokenRecord", "mplTokenMetadata"),
              [
                k.pdaSeedValueNode("mint", k.accountValueNode("mint")),
                k.pdaSeedValueNode("token", k.accountValueNode("vaultToken")),
              ]
            ),
          }),
        },
        authorizationRulesProgram: {
          defaultValue: k.conditionalValueNode({
            condition: k.argumentValueNode("tokenStandard"),
            value: k.enumValueNode(
              k.definedTypeLinkNode("TokenStandard", "mplTokenMetadata"),
              "ProgrammableNonFungible"
            ),
            ifTrue: k.publicKeyValueNode(
              "auth9SigNpDKz4sJJ1DfCTuZrZNSAgh9sFD3rboVmgg",
              "mplTokenAuthRules"
            ),
          }),
        },
      },
      arguments: {
        tokenStandard: {
          type: k.definedTypeLinkNode("TokenStandard", "mplTokenMetadata"),
          defaultValue: k.enumValueNode(
            k.definedTypeLinkNode("TokenStandard", "mplTokenMetadata"),
            "NonFungible"
          ),
        },
      },
    },
  })
);

// Set ShankAccount discriminator.
const key = (name) => ({
  field: "discriminator",
  value: k.enumValueNode("Discriminator", name),
});
kinobi.update(
  new k.setAccountDiscriminatorFromFieldVisitor({
    Vault: key("Vault"),
  })
);

// Render JavaScript.
kinobi.accept(
  jsRenderer.renderVisitor(
    path.join(clientDir, "js", "bridge", "src", "generated"),
    {
      prettier: require(
        path.join(clientDir, "js", "bridge", ".prettierrc.json")
      ),
      dependencyMap: {
        mplTokenMetadata: "@metaplex-foundation/mpl-token-metadata",
        mplToolbox: "@metaplex-foundation/mpl-toolbox",
      },
    }
  )
);

// Render Rust.
kinobi.accept(
  rustRendered.renderVisitor(
    path.join(clientDir, "rust", "bridge", "src", "generated"),
    {
      formatCode: true,
      crateFolder: path.join(clientDir, "rust", "bridge"),
    }
  )
);
