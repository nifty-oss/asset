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
    assetProgram: { name: "asset" }
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
                    child: k.linkTypeNode("Discriminator")
                  }),
                  k.structFieldTypeNode({
                    name: "state",
                    child: k.linkTypeNode("State")
                  }),
                  k.structFieldTypeNode({
                    name: "standard",
                    child: k.linkTypeNode("Standard"),
                  }),
                  k.structFieldTypeNode({
                    name: "mutable",
                    child: k.boolTypeNode()
                  }),
                  k.structFieldTypeNode({
                    name: "holder",
                    child: k.publicKeyTypeNode()
                  }),
                  k.structFieldTypeNode({
                    name: "group",
                    child: k.publicKeyTypeNode()
                  }),
                  k.structFieldTypeNode({
                    name: "authority",
                    child: k.publicKeyTypeNode()
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
            // attributes
            k.definedTypeNodeFromIdl({
              name: "attributes",
              type: {
                kind: "struct",
                fields: [
                  {
                    name: "traits",
                    type: { vec: { defined: "trait" }, size: "remainder" }
                  }
                ]
              }
            }),
            // trait
            k.definedTypeNode({
              name: "trait",
              data: k.structTypeNode([
                k.structFieldTypeNode({
                  name: "traitType",
                  child: k.stringTypeNode({ size: k.fixedSize(16) })
                }),
                k.structFieldTypeNode({
                  name: "value",
                  child: k.stringTypeNode({ size: k.fixedSize(16) })
                })
              ])
            }),
            // image
            k.definedTypeNode({
              name: "image",
              data: k.structTypeNode([
                k.structFieldTypeNode({
                  name: "data",
                  child: k.arrayTypeNode(k.numberTypeNode("u8"), {
                    size: k.remainderSize()
                  })
                })
              ])
            })
          ]
        });
      }
    }
  ])
);

// Update instructions.
kinobi.update(
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
    initialize: {
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
      internal: true
    },
    write: {
      internal: true,
    },
  })
);

// Set default values.
kinobi.update(
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
const key = (name) => ({
  field: "discriminator",
  value: k.vEnum("Discriminator", name)
});
kinobi.update(
  new k.SetAccountDiscriminatorFromFieldVisitor({
    asset: key("Asset")
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
    crateFolder: crateDir
  })
);
