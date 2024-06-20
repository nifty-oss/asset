const anchorIdl = require("@kinobi-so/nodes-from-anchor");
const rustRendered = require("@kinobi-so/renderers-rust");
const k = require("kinobi");

// Paths.
const path = require("path");
const programDir = path.join(__dirname, "..", "programs");
const idlDir = path.join(__dirname, "..", "idls");

// Instanciate Kinobi.
const idl = anchorIdl.rootNodeFromAnchor(
  require(path.join(idlDir, "nifty_asset_interface.json"))
);
const kinobi = k.createFromRoot(idl);

// Update programs.
kinobi.update(
  k.updateProgramsVisitor({
    niftyAssetInterface: { name: "interface" },
  })
);

// Add missing types from the IDL.
kinobi.update(
  k.bottomUpTransformerVisitor([
    {
      select: "[programNode]interface",
      transform: (node) => {
        k.assertIsNode(node, "programNode");
        return {
          ...node,
          definedTypes: [
            ...node.definedTypes,
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
        payer: { defaultValue: k.identityValueNode() },
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

//kinobi.accept(k.consoleLogVisitor(k.getDebugStringVisitor({ indent: true })));

// Render Rust.
kinobi.accept(
  rustRendered.renderVisitor(
    path.join(programDir, "asset", "interface", "src", "generated"),
    {
      formatCode: true,
      crateFolder: path.join(programDir, "asset", "interface"),
    }
  )
);
