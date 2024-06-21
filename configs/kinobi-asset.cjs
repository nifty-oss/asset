const anchorIdl = require("@kinobi-so/nodes-from-anchor");
const jsRenderer = require("@kinobi-so/renderers-js-umi");
const rustRenderer = require("@kinobi-so/renderers-rust");
const k = require("kinobi");

// Paths.
const path = require("path");
const clientDir = path.join(__dirname, "..", "clients");
const idlDir = path.join(__dirname, "..", "idls");

// Instanciate Kinobi.
const idl = anchorIdl.rootNodeFromAnchor(
  require(path.join(idlDir, "asset_program.json"))
);
const kinobi = k.createFromRoot(idl);

// Update programs.
kinobi.update(
  k.updateProgramsVisitor({
    assetProgram: { name: "asset" },
  })
);

// Add missing types from the IDL.
kinobi.update(
  k.bottomUpTransformerVisitor([
    {
      select: "[programNode]asset",
      transform: (node) => {
        k.assertIsNode(node, "programNode");
        return {
          ...node,
          accounts: [
            ...node.accounts,
            // asset account
            k.accountNode({
              name: "internalAsset",
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
                  name: "standard",
                  type: k.definedTypeLinkNode("Standard"),
                }),
                k.structFieldTypeNode({
                  name: "mutable",
                  type: k.booleanTypeNode(),
                }),
                k.structFieldTypeNode({
                  name: "owner",
                  type: k.publicKeyTypeNode(),
                }),
                k.structFieldTypeNode({
                  name: "group",
                  type: k.definedTypeLinkNode("nullablePublicKey", "hooked"),
                }),
                k.structFieldTypeNode({
                  name: "authority",
                  type: k.publicKeyTypeNode(),
                }),
                k.structFieldTypeNode({
                  name: "delegate",
                  type: k.definedTypeLinkNode("Delegate"),
                }),
                k.structFieldTypeNode({
                  name: "name",
                  type: k.fixedSizeTypeNode(k.stringTypeNode("utf8"), 35),
                }),
              ]),
            }),
          ],
          definedTypes: [
            ...node.definedTypes,
            // discriminator
            k.definedTypeNode({
              name: "discriminator",
              type: k.enumTypeNode([
                k.enumEmptyVariantTypeNode("Uninitialized"),
                k.enumEmptyVariantTypeNode("Asset"),
              ]),
            }),
            // standard
            k.definedTypeNode({
              name: "standard",
              type: k.enumTypeNode([
                k.enumEmptyVariantTypeNode("NonFungible"),
                k.enumEmptyVariantTypeNode("Managed"),
                k.enumEmptyVariantTypeNode("Soulbound"),
                k.enumEmptyVariantTypeNode("Proxied"),
              ]),
            }),
            // state
            k.definedTypeNode({
              name: "state",
              type: k.enumTypeNode([
                k.enumEmptyVariantTypeNode("Unlocked"),
                k.enumEmptyVariantTypeNode("Locked"),
              ]),
            }),
            // delegate role
            k.definedTypeNode({
              name: "delegateRole",
              type: k.enumTypeNode([
                k.enumEmptyVariantTypeNode("None"),
                k.enumEmptyVariantTypeNode("Transfer"),
                k.enumEmptyVariantTypeNode("Lock"),
                k.enumEmptyVariantTypeNode("Burn"),
              ]),
            }),
            // extension type
            k.definedTypeNode({
              name: "extensionType",
              type: k.enumTypeNode([
                k.enumEmptyVariantTypeNode("None"),
                k.enumEmptyVariantTypeNode("Attributes"),
                k.enumEmptyVariantTypeNode("Blob"),
                k.enumEmptyVariantTypeNode("Creators"),
                k.enumEmptyVariantTypeNode("Links"),
                k.enumEmptyVariantTypeNode("Metadata"),
                k.enumEmptyVariantTypeNode("Grouping"),
                k.enumEmptyVariantTypeNode("Royalties"),
                k.enumEmptyVariantTypeNode("Manager"),
                k.enumEmptyVariantTypeNode("Proxy"),
                k.enumEmptyVariantTypeNode("Properties"),
                k.enumEmptyVariantTypeNode("Bucket"),
              ]),
            }),
            // delegate
            k.definedTypeNode({
              name: "delegate",
              type: k.structTypeNode([
                k.structFieldTypeNode({
                  name: "address",
                  type: k.definedTypeLinkNode("nullablePublicKey", "hooked"),
                }),
                k.structFieldTypeNode({
                  name: "roles",
                  type: k.definedTypeLinkNode("delegateRoles", "hooked"),
                }),
              ]),
            }),
            // extension header
            k.definedTypeNode({
              name: "extensionHeader",
              type: k.structTypeNode([
                k.structFieldTypeNode({
                  name: "kind",
                  type: k.numberTypeNode("u32"),
                }),
                k.structFieldTypeNode({
                  name: "length",
                  type: k.numberTypeNode("u32"),
                }),
                k.structFieldTypeNode({
                  name: "boundary",
                  type: k.numberTypeNode("u32"),
                }),
                k.structFieldTypeNode({
                  name: "padding",
                  type: k.numberTypeNode("u32"),
                }),
              ]),
              internal: true,
            }),
            // attributes
            k.definedTypeNode({
              name: "attributes",
              type: k.structTypeNode([
                k.structFieldTypeNode({
                  name: "values",
                  type: k.arrayTypeNode(
                    k.definedTypeLinkNode("trait"),
                    k.remainderCountNode()
                  ),
                }),
              ]),
            }),
            // trait
            k.definedTypeNode({
              name: "trait",
              type: k.structTypeNode([
                k.structFieldTypeNode({
                  name: "name",
                  type: k.sizePrefixTypeNode(
                    k.stringTypeNode("utf8"),
                    k.numberTypeNode("u8")
                  ),
                }),
                k.structFieldTypeNode({
                  name: "value",
                  type: k.sizePrefixTypeNode(
                    k.stringTypeNode("utf8"),
                    k.numberTypeNode("u8")
                  ),
                }),
              ]),
            }),
            // blob
            k.definedTypeNode({
              name: "blob",
              type: k.structTypeNode([
                k.structFieldTypeNode({
                  name: "contentType",
                  type: k.sizePrefixTypeNode(
                    k.stringTypeNode("utf8"),
                    k.numberTypeNode("u8")
                  ),
                }),
                k.structFieldTypeNode({
                  name: "data",
                  type: k.arrayTypeNode(
                    k.numberTypeNode("u8"),
                    k.remainderCountNode()
                  ),
                }),
              ]),
            }),
            // links
            k.definedTypeNode({
              name: "links",
              type: k.structTypeNode([
                k.structFieldTypeNode({
                  name: "values",
                  type: k.arrayTypeNode(
                    k.definedTypeLinkNode("link"),
                    k.remainderCountNode()
                  ),
                }),
              ]),
            }),
            // link
            k.definedTypeNode({
              name: "link",
              type: k.structTypeNode([
                k.structFieldTypeNode({
                  name: "name",
                  type: k.sizePrefixTypeNode(
                    k.stringTypeNode("utf8"),
                    k.numberTypeNode("u8")
                  ),
                }),
                k.structFieldTypeNode({
                  name: "uri",
                  type: k.sizePrefixTypeNode(
                    k.stringTypeNode("utf8"),
                    k.numberTypeNode("u8")
                  ),
                }),
              ]),
            }),
            // creators
            k.definedTypeNode({
              name: "creators",
              type: k.structTypeNode([
                k.structFieldTypeNode({
                  name: "values",
                  type: k.arrayTypeNode(
                    k.definedTypeLinkNode("creator"),
                    k.remainderCountNode()
                  ),
                }),
              ]),
            }),
            // creator
            k.definedTypeNode({
              name: "creator",
              type: k.structTypeNode([
                k.structFieldTypeNode({
                  name: "address",
                  type: k.publicKeyTypeNode(),
                }),
                k.structFieldTypeNode({
                  name: "verified",
                  type: k.booleanTypeNode(),
                }),
                k.structFieldTypeNode({
                  name: "share",
                  type: k.numberTypeNode("u8"),
                }),
              ]),
            }),
            // metadata
            k.definedTypeNode({
              name: "metadata",
              type: k.structTypeNode([
                k.structFieldTypeNode({
                  name: "symbol",
                  type: k.sizePrefixTypeNode(
                    k.stringTypeNode("utf8"),
                    k.numberTypeNode("u8")
                  ),
                }),
                k.structFieldTypeNode({
                  name: "description",
                  type: k.sizePrefixTypeNode(
                    k.stringTypeNode("utf8"),
                    k.numberTypeNode("u8")
                  ),
                }),
                k.structFieldTypeNode({
                  name: "uri",
                  type: k.sizePrefixTypeNode(
                    k.stringTypeNode("utf8"),
                    k.numberTypeNode("u8")
                  ),
                }),
                k.structFieldTypeNode({
                  name: "imageUri",
                  type: k.sizePrefixTypeNode(
                    k.stringTypeNode("utf8"),
                    k.numberTypeNode("u8")
                  ),
                }),
              ]),
            }),
            // grouping
            k.definedTypeNode({
              name: "grouping",
              type: k.structTypeNode([
                k.structFieldTypeNode({
                  name: "size",
                  type: k.numberTypeNode("u64"),
                }),
                k.structFieldTypeNode({
                  name: "maxSize",
                  type: k.numberTypeNode("u64"),
                }),
                k.structFieldTypeNode({
                  name: "delegate",
                  type: k.definedTypeLinkNode("nullablePublicKey", "hooked"),
                }),
              ]),
            }),
            // manager
            k.definedTypeNode({
              name: "manager",
              type: k.structTypeNode([
                k.structFieldTypeNode({
                  name: "delegate",
                  type: k.definedTypeLinkNode("delegate"),
                }),
              ]),
            }),
            // proxy
            k.definedTypeNode({
              name: "proxy",
              type: k.structTypeNode([
                k.structFieldTypeNode({
                  name: "program",
                  type: k.publicKeyTypeNode(),
                }),
                k.structFieldTypeNode({
                  name: "seeds",
                  type: k.arrayTypeNode(
                    k.numberTypeNode("u8"),
                    k.fixedCountNode(32)
                  ),
                }),
                k.structFieldTypeNode({
                  name: "bump",
                  type: k.numberTypeNode("u8"),
                }),
                k.structFieldTypeNode({
                  name: "authority",
                  type: k.definedTypeLinkNode("nullablePublicKey", "hooked"),
                }),
              ]),
            }),
            // bucket
            k.definedTypeNode({
              name: "bucket",
              type: k.structTypeNode([
                k.structFieldTypeNode({
                  name: "data",
                  type: k.arrayTypeNode(
                    k.numberTypeNode("u8"),
                    k.remainderCountNode()
                  ),
                }),
              ]),
            }),
            // type (for properties extension)
            k.definedTypeNode({
              name: "type",
              type: k.enumTypeNode([
                k.enumEmptyVariantTypeNode("Text"),
                k.enumEmptyVariantTypeNode("Number"),
                k.enumEmptyVariantTypeNode("Boolean"),
              ]),
            }),
          ],
        };
      },
    },
  ])
);

// Update instructions.
kinobi.update(
  k.updateInstructionsVisitor({
    allocate: {
      accounts: {
        systemProgram: {
          defaultValue: k.conditionalValueNode({
            condition: k.accountValueNode("payer"),
            ifTrue: k.publicKeyValueNode(
              "11111111111111111111111111111111",
              "systemProgram"
            ),
          }),
        },
      },
    },
    approve: {
      accounts: {
        owner: { defaultValue: k.identityValueNode() },
      },
    },
    create: {
      accounts: {
        owner: { defaultValue: k.identityValueNode() },
        authority: { defaultValue: k.identityValueNode() },
        systemProgram: {
          defaultValue: k.conditionalValueNode({
            condition: k.accountValueNode("payer"),
            ifTrue: k.publicKeyValueNode(
              "11111111111111111111111111111111",
              "systemProgram"
            ),
          }),
        },
      },
      arguments: {
        extensions: {
          defaultValue: k.noneValueNode(),
        },
      },
    },
    handover: {
      accounts: {
        authority: { defaultValue: k.identityValueNode() },
      },
    },
    group: {
      accounts: {
        authority: { defaultValue: k.identityValueNode() },
      },
    },
    lock: {
      accounts: {
        signer: { defaultValue: k.identityValueNode() },
      },
    },
    remove: {
      accounts: {
        authority: { defaultValue: k.identityValueNode() },
        recipient: { defaultValue: k.identityValueNode() },
      },
    },
    resize: {
      accounts: {
        authority: { defaultValue: k.identityValueNode() },
        systemProgram: {
          defaultValue: k.conditionalValueNode({
            condition: k.accountValueNode("payer"),
            ifTrue: k.publicKeyValueNode(
              "11111111111111111111111111111111",
              "systemProgram"
            ),
          }),
        },
      },
    },
    revoke: {
      accounts: {
        signer: { defaultValue: k.identityValueNode() },
      },
    },
    transfer: {
      accounts: {
        signer: { defaultValue: k.identityValueNode() },
      },
    },
    ungroup: {
      accounts: {
        authority: { defaultValue: k.identityValueNode() },
      },
    },
    unlock: {
      accounts: {
        signer: { defaultValue: k.identityValueNode() },
      },
    },
    update: {
      accounts: {
        authority: { defaultValue: k.identityValueNode() },
        systemProgram: {
          defaultValue: k.conditionalValueNode({
            condition: k.accountValueNode("payer"),
            ifTrue: k.publicKeyValueNode(
              "11111111111111111111111111111111",
              "systemProgram"
            ),
          }),
        },
      },
    },
    write: {
      accounts: {
        systemProgram: {
          defaultValue: k.publicKeyValueNode(
            "11111111111111111111111111111111",
            "systemProgram"
          ),
        },
      },
    },
  })
);

// Set more struct default values dynamically.
kinobi.update(
  k.bottomUpTransformerVisitor([
    {
      select: "[instructionNode|instructionArgumentNode]standard",
      transform: (node) => {
        k.assertIsNode(node, ["instructionNode", "instructionArgumentNode"]);
        return {
          ...node,
          defaultValue: k.enumValueNode("Standard", "NonFungible"),
        };
      },
    },
    {
      select: "[instructionNode|instructionArgumentNode]mutable",
      transform: (node) => {
        k.assertIsNode(node, ["instructionNode", "instructionArgumentNode"]);
        return {
          ...node,
          defaultValue: k.booleanValueNode(true),
        };
      },
    },
    {
      select: (node) => {
        const names = ["name", "mutable", "extension"];
        return (
          k.isNode(node, ["instructionNode", "instructionArgumentNode"]) &&
          k.isNode(node.type, "optionTypeNode") &&
          names.includes(node.name)
        );
      },
      transform: (node) => {
        k.assertIsNode(node, ["instructionNode", "instructionArgumentNode"]);
        return {
          ...node,
          defaultValueStrategy: "optional",
          defaultValue: k.noneValueNode(),
        };
      },
    },
  ])
);

// Set ShankAccount discriminator.
const key = (name) => ({
  field: "discriminator",
  value: k.enumValueNode("Discriminator", name),
});
kinobi.update(
  new k.setAccountDiscriminatorFromFieldVisitor({
    Asset: key("Asset"),
  })
);

// Render JavaScript.
kinobi.accept(
  jsRenderer.renderVisitor(
    path.join(clientDir, "js", "asset", "src", "generated"),
    {
      prettier: require(
        path.join(clientDir, "js", "asset", ".prettierrc.json")
      ),
      internalNodes: [
        "allocate",
        "approve",
        "burn",
        "create",
        "group",
        "handover",
        "lock",
        "remove",
        "resize",
        "revoke",
        "transfer",
        "ungroup",
        "unlock",
        "unverify",
        "update",
        "verify",
        "write",
        "internalAsset",
      ],
      customAccountData: [
        {
          name: "internalAsset",
          extract: true,
        },
      ],
    }
  )
);

// Render Rust.
kinobi.accept(
  rustRenderer.renderVisitor(
    path.join(clientDir, "rust", "asset", "src", "generated"),
    {
      formatCode: true,
      crateFolder: path.join(clientDir, "rust", "asset"),
    }
  )
);
