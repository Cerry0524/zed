# Zed 繁體中文 macOS 發佈包狀態

日期：2026-05-26
再驗證：2026-05-27
分支：`zh-hant-ui-slice`
commit：`b9410f4a4cd8e1fdc5bd1a2cb2304a72068f693d`

## 結論

目前已產生可安裝測試的 Apple Silicon DMG：

`target/aarch64-apple-darwin/release/Zed-aarch64.dmg`

也已產生 ZIP 備援包：

`target/aarch64-apple-darwin/release/Zed-aarch64.zip`

這個 DMG 內含 `Zed Dev.app`，已用本機 Developer ID Application 憑證簽章，且補上 hardened runtime 與 timestamp。它可以用於本機或技術測試者的安裝驗證。

但它還不是正式公開散佈等級的安裝檔，因為尚未完成 Apple notarization，也沒有 stapled notarization ticket。一般使用者從網路下載後，Gatekeeper 可能會阻擋或顯示安全警告。

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

## 已完成驗證

- `script/bundle-mac aarch64-apple-darwin` 已成功產生 `.app` 與 `.dmg`。
- `codesign --verify --deep --strict --verbose=4 Zed Dev.app` 通過：
  - `valid on disk`
  - `satisfies its Designated Requirement`
- `codesign -dv --verbose=4 Zed Dev.app` 顯示：
  - `Authority=Developer ID Application: KO-JUI CHEN (3PM99X2THU)`
  - `TeamIdentifier=3PM99X2THU`
  - `flags=0x10000(runtime)`
  - `Timestamp=May 26, 2026`
- `hdiutil verify Zed-aarch64.dmg` 通過：
  - `checksum ... is VALID`
- `unzip -t Zed-aarch64.zip` 通過：
  - `No errors detected`
- `script/verify-mac-release` 通過寬鬆模式：
  - bundle structure
  - app and nested executable signatures
  - hardened runtime / timestamp / Team ID
  - app entitlements
  - DMG signature and checksum
  - ZIP integrity
- DMG 外層也已用 Developer ID Application 簽章並含 timestamp。

SHA-256：

- DMG：`d37a5106ee7ee73ba51a27636634186466c62a9306bf3cc48746426437825f94`
- ZIP：`201c26a59786125d82ab436218206316930905b4c18463dc1077f4dde4725a05`

## Smoke Test

已從 DMG 掛載點執行乾淨 user-data smoke test：

- 掛載：`/private/tmp/zed-zh-hant-dmg`
- user data：`/private/tmp/zed-zh-hant-release-smoke`
- 測試專案：`/private/tmp/zed-zh-hant-smoke-project`
- `cli --version` 輸出：`Zed 1.5.0 - /private/tmp/zed-zh-hant-dmg/Zed Dev.app`
- app 可由系統識別為：`Zed Dev`
- 啟動後有建立乾淨 profile 的 `db`、`extensions`、`threads`、`external_agents` 等資料。

## 尚未完成

以下驗證尚未通過或尚未執行完成：

- `xcrun stapler validate Zed-aarch64.dmg` 未通過：
  - `Zed-aarch64.dmg does not have a ticket stapled to it.`
- `spctl -a -vv --type open Zed-aarch64.dmg` 未通過：
  - `rejected`
  - `source=Insufficient Context`
- `script/verify-mac-release --require-notarization` 會在 `stapler validate` 階段失敗，符合目前未公證狀態。
- 未執行 `xcrun notarytool submit`，因此沒有 Apple notarization ticket。
- 未執行 `xcrun stapler staple`，因此 DMG 未 stapled。

## 缺少的公證憑證

目前 shell 環境沒有下列 Zed 官方腳本所需的變數：

- `MACOS_CERTIFICATE`
- `MACOS_CERTIFICATE_PASSWORD`
- `APPLE_NOTARIZATION_KEY`
- `APPLE_NOTARIZATION_KEY_ID`
- `APPLE_NOTARIZATION_ISSUER_ID`

也沒有偵測到可直接使用的 Apple ID / app-specific password 環境變數：

- `APPLE_ID`
- `AC_USERNAME`
- `AC_PASSWORD`
- `ASC_PROVIDER`

本機有可用的公證工具：

- `xcrun notarytool --version`：`1.1.2 (41)`

Keychain 檢查結果：

- `security find-generic-password -s com.apple.gke.notary.tool`：找不到項目。
- `security find-generic-password -s notarytool`：找不到項目。
- 常見 profile 名稱 `zed-zh-hant-notary`、`notarytool`、`notary`、`zed`、`zed-notary`、`developer-id`、`3PM99X2THU`、`Cerry`、`cerry`、`default`、`AC_PASSWORD` 均回報：
  - `No Keychain password item found for profile`
- Local Items keychain 內可看到 1 筆 `com.apple.gke.notary` 類型項目，但 profile 名稱不是可安全讀取的明文字串；目前沒有可直接用於 `xcrun notarytool submit` 的已知 profile 名稱。
- `Downloads`、`Documents`、`Desktop` 內未找到 `AuthKey_*.p8` 或其他 `.p8` 檔案。

## 下一步

正式公開分享前，應補齊其中一種 notarization 路徑：

此 repo 目前提供收尾腳本：

```sh
script/verify-mac-release
script/notarize-mac-release --artifact target/aarch64-apple-darwin/release/Zed-aarch64.dmg
```

若尚未建立 `notarytool` keychain profile，可先使用：

```sh
script/store-mac-notary-credentials --profile zed-zh-hant-notary
```

腳本會執行：

- `codesign --verify --deep --strict`
- nested executable signature checks
- hardened runtime / timestamp / Team ID checks
- `unzip -t`
- `xcrun notarytool submit --wait`
- `xcrun stapler staple`
- `xcrun stapler validate`
- `hdiutil verify`
- `spctl -a -vv --type open`

公證前可使用寬鬆模式重驗目前測試包：

```sh
script/verify-mac-release
```

公證後應使用正式 gate：

```sh
script/verify-mac-release --require-notarization
```

1. 使用 App Store Connect API key：
   - 準備 `.p8` key。
   - 設定 key id 與 issuer id。
   - 建立 keychain profile。
   - 執行 `script/notarize-mac-release --profile <profile>`。

   範例：

   ```sh
   script/store-mac-notary-credentials \
     --profile zed-zh-hant-notary \
     --key /path/to/AuthKey_XXXXXXXXXX.p8 \
     --key-id XXXXXXXXXX \
     --issuer XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX

   script/notarize-mac-release \
     --artifact target/aarch64-apple-darwin/release/Zed-aarch64.dmg \
     --profile zed-zh-hant-notary
   ```

2. 使用 notarytool keychain profile：
   - 先用 `xcrun notarytool store-credentials` 建立 profile。
   - 再執行 `script/notarize-mac-release --profile <profile>`。

   範例：

   ```sh
   xcrun notarytool store-credentials zed-zh-hant-notary \
     --apple-id your-apple-id@example.com \
     --team-id 3PM99X2THU \
     --password xxxx-xxxx-xxxx-xxxx

   script/notarize-mac-release \
     --artifact target/aarch64-apple-darwin/release/Zed-aarch64.dmg \
     --profile zed-zh-hant-notary
   ```

3. 使用 Apple ID app-specific password：
   - 準備 Apple ID、team/provider、app-specific password。
   - 建立 keychain profile。
   - 執行 `script/notarize-mac-release --profile <profile>`。

   範例：

   ```sh
   script/store-mac-notary-credentials \
     --profile zed-zh-hant-notary \
     --apple-id your-apple-id@example.com \
     --team-id 3PM99X2THU \
     --password xxxx-xxxx-xxxx-xxxx

   script/notarize-mac-release \
     --artifact target/aarch64-apple-darwin/release/Zed-aarch64.dmg \
     --profile zed-zh-hant-notary
   ```

## 分享建議

目前可以分享給熟悉 macOS 安全提示的技術測試者，並註明這是未公證測試包。

若要分享給一般使用者，必須先完成 notarization 與 stapling，否則使用者可能會遇到 Gatekeeper 阻擋。
