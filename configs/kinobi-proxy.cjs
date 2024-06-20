const anchorIdl = require("@kinobi-so/nodes-from-anchor");
const jsRenderer = require("@kinobi-so/renderers-js-umi");
const k = require("kinobi");

// Paths.
const path = require("path");
const clientDir = path.join(__dirname, "..", "clients");
const idlDir = path.join(__dirname, "..", "idls");

// Instanciate Kinobi.
const idl = anchorIdl.rootNodeFromAnchor(
  require(path.join(idlDir, "proxy_program.json"))
);
const kinobi = k.createFromRoot(idl);

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
  jsRenderer.renderVisitor(
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
