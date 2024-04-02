const path = require("path");
const { generateIdl } = require("@metaplex-foundation/shank-js");

const idlDir = path.join(__dirname, "..", "idls");
const binaryInstallDir = path.join(__dirname, "..", ".crates");
const programDir = path.join(__dirname, "..", "programs");

generateIdl({
  generator: "shank",
  programName: "asset_program",
  programId: "AssetGtQBTSgm5s91d1RAQod5JmaZiJDxqsgtqrZud73",
  idlDir,
  binaryInstallDir,
  programDir: path.join(programDir, "asset", "program"),
});

generateIdl({
  generator: "shank",
  programName: "bridge_program",
  programId: "BridgezKrNugsZwTcyAMYba643Z93RzC2yN1Y24LwAkm",
  idlDir,
  binaryInstallDir,
  programDir: path.join(programDir, "bridge"),
});

generateIdl({
  generator: "shank",
  programName: "proxy_program",
  programId: "Proxy11111111111111111111111111111111111111",
  idlDir,
  binaryInstallDir,
  programDir: path.join(programDir, "proxy"),
});
