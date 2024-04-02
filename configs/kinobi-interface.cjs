const path = require("path");
const k = require("@metaplex-foundation/kinobi");

// Paths.
const programDir = path.join(__dirname, "..", "programs");
const idlDir = path.join(__dirname, "..", "idls");

// Instanciate Kinobi.
const kinobi = k.createFromIdls([
  path.join(idlDir, "nifty_asset_interface.json"),
]);

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
            k.definedTypeNodeFromIdl({
              name: "standard",
              type: {
                kind: "enum",
                variants: [
                  { name: "NonFungible" },
                  { name: "Managed" },
                  { name: "Soulbound" },
                  { name: "Proxied" },
                ],
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
                  { name: "Metadata" },
                  { name: "Grouping" },
                  { name: "Royalties" },
                  { name: "Manager" },
                  { name: "Proxy" },
                ],
              },
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
    create: {
      accounts: {
        owner: { defaultValue: k.identityValueNode() },
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
  k.renderRustVisitor(
    path.join(programDir, "asset", "interface", "src", "generated"),
    {
      formatCode: true,
      crateFolder: path.join(programDir, "asset", "interface"),
    }
  )
);
