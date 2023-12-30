const path = require("path");
const k = require("@metaplex-foundation/kinobi");

// Paths.
const clientDir = path.join(__dirname, "..", "clients");
const idlDir = path.join(__dirname, "..", "idls");

// Instanciate Kinobi.
const kinobi = k.createFromIdls([path.join(idlDir, "asset_program.json")]);

// Update programs.
kinobi.update(
  new k.UpdateProgramsVisitor({
    assetProgram: { name: "asset" },
  })
);

// Add missing types from the IDL.
kinobi.update(
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
                    name: "bump",
                    child: k.numberTypeNode("u8"),
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
                    child: k.publicKeyTypeNode(),
                  }),
                  k.structFieldTypeNode({
                    name: "name",
                    child: k.stringTypeNode({ size: k.fixedSize(32) }),
                  }),
                  k.structFieldTypeNode({
                    name: "symbol",
                    child: k.stringTypeNode({ size: k.fixedSize(10) }),
                  }),
                ]),
              }),
            }),
          ],
          definedTypes: [
            ...node.definedTypes,
            // attributes
            k.definedTypeNodeFromIdl({
              name: "attributes",
              type: {
                kind: "struct",
                fields: [
                  {
                    name: "traits",
                    type: { vec: { defined: "trait" }, size: "remainder" },
                  },
                ],
              },
            }),
            // trait
            k.definedTypeNode({
              name: "trait",
              data: k.structTypeNode([
                k.structFieldTypeNode({
                  name: "traitType",
                  child: k.stringTypeNode({ size: k.fixedSize(16) }),
                }),
                k.structFieldTypeNode({
                  name: "value",
                  child: k.stringTypeNode({ size: k.fixedSize(16) }),
                }),
              ]),
            }),
            // image
            k.definedTypeNode({
              name: "image",
              data: k.structTypeNode([
                k.structFieldTypeNode({
                  name: "data",
                  child: k.arrayTypeNode(k.numberTypeNode("u8"), {
                    size: k.remainderSize(),
                  }),
                }),
              ]),
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
    asset: {
      seeds: [
        k.stringConstantSeed("asset"),
        k.publicKeySeed("canvas", "Address to derive the PDA from"),
      ],
    },
  })
);

// Update instructions.
kinobi.update(
  new k.UpdateInstructionsVisitor({
    create: {
      accounts: {
        asset: { defaultsTo: k.pdaDefault("asset") },
      },
    },
    initialize: {
      accounts: {
        asset: { defaultsTo: k.pdaDefault("asset") },
      },
      internal: true,
    },
    write: {
      accounts: {
        asset: { defaultsTo: k.pdaDefault("asset") },
      },
    },
  })
);

// Set default values.
kinobi.update(
  new k.SetStructDefaultValuesVisitor({
    initialize: {
      data: k.vNone(),
    },
  })
);

// Set ShankAccount discriminator.
const key = (name) => ({
  field: "discriminator",
  value: k.vEnum("Discriminator", name),
});
kinobi.update(
  new k.SetAccountDiscriminatorFromFieldVisitor({
    asset: key("Asset"),
  })
);

// Render JavaScript.
const jsDir = path.join(clientDir, "js", "src", "generated");
const prettier = require(path.join(clientDir, "js", ".prettierrc.json"));
kinobi.accept(new k.RenderJavaScriptVisitor(jsDir, { prettier }));

// Render Rust.
const crateDir = path.join(clientDir, "rust");
const rustDir = path.join(clientDir, "rust", "src", "generated");
kinobi.accept(
  new k.RenderRustVisitor(rustDir, {
    formatCode: true,
    crateFolder: crateDir,
  })
);
