const path = require("path");
const k = require("@metaplex-foundation/kinobi");

// Paths.
const clientDir = path.join(__dirname, "..", "clients");
const idlDir = path.join(__dirname, "..", "idls");

// Instanciate Kinobi.
const kinobi = k.createFromIdls([path.join(idlDir, "proxy_program.json")]);

// Update programs.
kinobi.update(
  k.updateProgramsVisitor({
    proxyProgram: { name: "proxy" },
  })
);

// Update instructions.
kinobi.update(
  k.updateInstructionsVisitor({
    create: {
      accounts: {
        asset: {
          defaultValue: k.resolverValueNode("resolveProxiedAsset", {
            dependsOn: [k.accountValueNode("stub")],
          }),
        },
        systemProgram: {
          defaultValue: k.conditionalValueNode({
            condition: k.accountValueNode("payer"),
            ifTrue: k.publicKeyValueNode(
              "11111111111111111111111111111111",
              "systemProgram"
            ),
          }),
        },
        niftyAssetProgram: {
          defaultValue: k.publicKeyValueNode(
            "AssetGtQBTSgm5s91d1RAQod5JmaZiJDxqsgtqrZud73",
            "niftyAsset"
          ),
        },
      },
      arguments: {
        version: {
          type: k.numberTypeNode("u8"),
          defaultValue: k.numberValueNode(1),
        },
      },
    },
  })
);

// Render JavaScript.
kinobi.accept(
  k.renderJavaScriptVisitor(
    path.join(clientDir, "js", "proxy", "src", "generated"),
    {
      prettier: require(path.join(
        clientDir,
        "js",
        "proxy",
        ".prettierrc.json"
      )),
    }
  )
);

/*
kinobi.accept(k.consoleLogVisitor(k.getDebugStringVisitor({ indent: true })));

// Render Rust.
kinobi.accept(
  k.renderRustVisitor(
    path.join(clientDir, "rust", "bridge", "src", "generated"),
    {
      formatCode: true,
      crateFolder: path.join(clientDir, "rust", "bridge"),
    }
  )
);
*/
