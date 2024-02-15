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
                variants: [
                  { name: "NonFungible" },
                  { name: "Subscription" },
                  { name: "Soulbound" },
                ],
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
                  { name: "Metadata" },
                  { name: "Grouping" },
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
                    size: k.remainderSize(),
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
            // metadata
            k.definedTypeNode({
              name: "metadata",
              data: k.structTypeNode([
                k.structFieldTypeNode({
                  name: "symbol",
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
            // grouping
            k.definedTypeNode({
              name: "grouping",
              data: k.structTypeNode([
                k.structFieldTypeNode({
                  name: "size",
                  child: k.numberTypeNode("u64"),
                }),
                k.structFieldTypeNode({
                  name: "max_size",
                  child: k.numberTypeNode("u64"),
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
kinobi.update(
  new k.SetStructDefaultValuesVisitor({
    initialize: {
      data: k.vNone(),
    },
    create: {
      holder: k.identityDefault(),
    },
    CreateInstructionData: {
      standard: k.vEnum("Standard", "NonFungible"),
      mutable: k.vScalar(true),
    },
    UpdateInstructionData: {
      name: k.vNone(),
      mutable: k.vNone(),
      extension: k.vNone(),
    },
  })
);

// Set ShankAccount discriminator.
const assetKey = (name) => ({
  field: "discriminator",
  value: k.vEnum("Discriminator", name),
});
kinobi.update(
  new k.SetAccountDiscriminatorFromFieldVisitor({
    asset: assetKey("Asset"),
  })
);

// Use custom serializers.
kinobi.update(
  new k.UseCustomAccountSerializerVisitor({
    asset: { extract: true },
  })
);

// Render JavaScript.
kinobi.accept(
  new k.RenderJavaScriptVisitor(
    path.join(clientDir, "js", "asset", "src", "generated"),
    {
      prettier: require(path.join(
        clientDir,
        "js",
        "asset",
        ".prettierrc.json"
      )),
    }
  )
);

// Render Rust.
kinobi.accept(
  new k.RenderRustVisitor(
    path.join(clientDir, "rust", "asset", "src", "generated"),
    {
      formatCode: true,
      crateFolder: path.join(clientDir, "rust", "asset"),
    }
  )
);
