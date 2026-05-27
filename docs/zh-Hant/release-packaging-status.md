# Zed 繁體中文 macOS 發佈包狀態

日期：2026-05-26
正式發佈驗證：2026-05-27
分支：`zh-hant-ui-slice`
App artifact commit：`b9410f4a4cd8e1fdc5bd1a2cb2304a72068f693d`

## 結論

Apple Silicon 版 Zed 繁體中文 DMG 已完成正式發佈驗證，可分享給一般 macOS Apple Silicon 使用者安裝。

正式上傳目錄：

`target/aarch64-apple-darwin/release/release-stage/`

主要安裝檔：

`target/aarch64-apple-darwin/release/Zed-aarch64.dmg`

這個 DMG 內含 `Zed Dev.app`，已完成 Developer ID 簽章、Apple notarization、stapling、Gatekeeper assessment 與乾淨環境 smoke test。

ZIP 仍有完整性驗證，可作為備援 artifact；公開給一般使用者時建議優先分享已 stapled notarization ticket 的 DMG。

## 目前產物

- DMG：`target/aarch64-apple-darwin/release/Zed-aarch64.dmg`
- DMG 大小：約 `129M`
- ZIP：`target/aarch64-apple-darwin/release/Zed-aarch64.zip`
- ZIP 大小：約 `117M`
- App：`Zed Dev.app`
- Bundle ID：`dev.zed.Zed-Dev`
- App version：`1.5.0`
- Build：`20260526.095402`
- 架構：`arm64`
- 簽章憑證：`Developer ID Application: KO-JUI CHEN (3PM99X2THU)`
- Team ID：`3PM99X2THU`

SHA-256：

- DMG：`ab35a5a3b96134605a8f13b4d1d70c2154564540c39d9e39f88f0bda5dc68cac`
- ZIP：`201c26a59786125d82ab436218206316930905b4c18463dc1077f4dde4725a05`

## Apple Notarization

- notarytool profile：`zed-zh-hant-notary`
- notarization submission ID：`8c50bcf9-8d5c-4e6a-950f-52bbf09cb72e`
- Apple notarization status：`Accepted`
- `xcrun stapler staple`：passed
- `xcrun stapler validate`：passed
- Gatekeeper：
  - command：`spctl -a -vv --type open --context context:primary-signature target/aarch64-apple-darwin/release/Zed-aarch64.dmg`
  - result：`accepted`
  - source：`Notarized Developer ID`

## 已完成驗證

- `script/bundle-mac aarch64-apple-darwin` 已成功產生 `.app` 與 `.dmg`。
- `codesign --verify --deep --strict --verbose=4 Zed Dev.app` 通過。
- app 與 DMG 都由 `Developer ID Application: KO-JUI CHEN (3PM99X2THU)` 簽章。
- app signature 包含 hardened runtime 與 timestamp。
- `hdiutil verify Zed-aarch64.dmg` 通過。
- `unzip -t Zed-aarch64.zip` 通過。
- `script/notarize-mac-release --artifact target/aarch64-apple-darwin/release/Zed-aarch64.dmg --profile zed-zh-hant-notary` 已完成 Apple notarization 與 stapling。
- `script/verify-mac-release --require-notarization` 通過。
- `script/check-mac-release-readiness` 通過：
  - `Public release gate: PASS.`
- `script/smoke-mac-release --launch-gui` 通過：
  - 從 DMG 掛載點確認 `Zed Dev.app`。
  - `Applications` symlink 指向 `/Applications`。
  - `Contents/MacOS/cli --version` 輸出 `Zed 1.5.0`。
  - LaunchServices 可識別 app 為 `Zed Dev`。
  - 乾淨 user-data 有建立 `db`、`extensions`。
  - 測試結束後 app 關閉且 DMG 成功卸載。
- `script/stage-mac-release-artifacts --require-notarization --artifact-commit b9410f4a4cd8e1fdc5bd1a2cb2304a72068f693d` 通過。
- `shasum -a 256 -c SHA256SUMS.txt` 在 release-stage 目錄內通過。

## Release Stage

目錄：

`target/aarch64-apple-darwin/release/release-stage/`

內容：

- `Zed-aarch64.dmg`
- `Zed-aarch64.zip`
- `Zed-aarch64.release-manifest.json`
- `Zed-aarch64.release-manifest.txt`
- `SHA256SUMS.txt`
- `README.txt`

manifest 目前顯示：

- app code signature：`passed`
- DMG hdiutil verify：`passed`
- ZIP unzip test：`passed`
- stapler validate：`passed`
- Gatekeeper：`accepted`

## 重跑命令

正式發佈前可重跑：

```sh
script/check-mac-release-readiness
script/verify-mac-release --require-notarization
script/smoke-mac-release --launch-gui
script/stage-mac-release-artifacts --require-notarization --artifact-commit b9410f4a4cd8e1fdc5bd1a2cb2304a72068f693d
```

上傳前可驗證 release-stage checksum：

```sh
cd target/aarch64-apple-darwin/release/release-stage
shasum -a 256 -c SHA256SUMS.txt
```

## 限制與後續

- 目前正式包是 `aarch64-apple-darwin`，也就是 Apple Silicon 版。
- 尚未產生 Intel `x86_64-apple-darwin` 或 Universal 版。
- `zed-zh-hant-notary` profile 存在於本機 Keychain，不會提交到 repo。
- 若要支援 Intel Mac，需要另行 build、sign、notarize、smoke test 對應 artifact。
