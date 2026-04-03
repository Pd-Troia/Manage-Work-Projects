import { execSync } from "child_process"
import { readFileSync, writeFileSync } from "fs"

const bump = process.argv[2]

if (!bump || !["major", "minor", "patch"].includes(bump)) {
  console.error("Uso: npm run release <major|minor|patch>")
  console.error("Exemplos:")
  console.error("  npm run release patch   → 0.4.0 → 0.4.1")
  console.error("  npm run release minor   → 0.4.0 → 0.5.0")
  console.error("  npm run release major   → 0.4.0 → 1.0.0")
  process.exit(1)
}

const pkg = JSON.parse(readFileSync("package.json", "utf8"))
const [major, minor, patch] = pkg.version.split(".").map(Number)

const version = bump === "major"
  ? `${major + 1}.0.0`
  : bump === "minor"
  ? `${major}.${minor + 1}.0`
  : `${major}.${minor}.${patch + 1}`

const tag = `v${version}`

// Bump package.json
pkg.version = version
writeFileSync("package.json", JSON.stringify(pkg, null, 2) + "\n")

// Bump tauri.conf.json
const conf = JSON.parse(readFileSync("src-tauri/tauri.conf.json", "utf8"))
conf.version = version
writeFileSync("src-tauri/tauri.conf.json", JSON.stringify(conf, null, 2) + "\n")

// Bump Cargo.toml
let cargo = readFileSync("src-tauri/Cargo.toml", "utf8")
cargo = cargo.replace(/^version = ".*"/m, `version = "${version}"`)
writeFileSync("src-tauri/Cargo.toml", cargo)

console.log(`✓ Versão atualizada para ${version}`)

// Commit
execSync(`git add package.json src-tauri/tauri.conf.json src-tauri/Cargo.toml`, { stdio: "inherit" })
execSync(`git commit -m "chore: bump version → ${version}"`, { stdio: "inherit" })
console.log(`✓ Commit criado`)

// Tag
execSync(`git tag ${tag}`, { stdio: "inherit" })
console.log(`✓ Tag ${tag} criada`)

// Push
execSync(`git push origin main`, { stdio: "inherit" })
execSync(`git push origin ${tag}`, { stdio: "inherit" })
console.log(`✓ Push concluído → CI disparado para ${tag}`)
