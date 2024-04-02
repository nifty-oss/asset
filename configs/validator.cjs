const path = require("path");

const programDir = path.join(__dirname, "..", "programs");

function getProgram(programBinary) {
  return path.join(programDir, ".bin", programBinary);
}

module.exports = {
  validator: {
    commitment: "processed",
    programs: [
      {
        label: "Asset",
        programId: "AssetGtQBTSgm5s91d1RAQod5JmaZiJDxqsgtqrZud73",
        deployPath: getProgram("asset_program.so"),
      },
      {
        label: "Bridge",
        programId: "BridgezKrNugsZwTcyAMYba643Z93RzC2yN1Y24LwAkm",
        deployPath: getProgram("bridge_program.so"),
      },
      {
        label: "Proxy",
        programId: "Proxy11111111111111111111111111111111111111",
        deployPath: getProgram("proxy_program.so"),
      },
      {
        label: "Token Metadata",
        programId: "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s",
        deployPath: getProgram("mpl_token_metadata.so"),
      },
    ],
  },
};
