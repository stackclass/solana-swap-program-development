// Migrations are managed by Anchor
// This file can be used for custom deployment scripts

import * as anchor from "@coral-xyz/anchor";

async function main() {
  console.log("Running deployment script...");
  // Custom deployment logic can be added here
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
