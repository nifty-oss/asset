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
    ],
  },
};
