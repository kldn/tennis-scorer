## 1. Database Migration

- [ ] 1.1 建立 migration：ALTER TABLE matches ADD COLUMN opponent_user_id UUID REFERENCES users(id)
- [ ] 1.2 同一 migration：ADD COLUMN opponent_email TEXT, ADD COLUMN opponent_name TEXT
- [ ] 1.3 同一 migration：ADD COLUMN opponent_claim_token UUID UNIQUE
- [ ] 1.4 新增 index：CREATE INDEX idx_matches_opponent_user_id ON matches(opponent_user_id)
- [ ] 1.5 新增 index：CREATE INDEX idx_matches_opponent_email ON matches(opponent_email)

## 2. Models 更新

- [ ] 2.1 `CreateMatchRequest` 新增 optional 欄位：opponent_email, opponent_name
- [ ] 2.2 `MatchResponse` 新增欄位：opponent_user_id, opponent_email, opponent_name, opponent_claim_token
- [ ] 2.3 新增 `ClaimMatchRequest` struct：{ token: Uuid }
- [ ] 2.4 新增 `ClaimMatchResponse` struct：{ match_id, message }

## 3. Create Match 更新

- [ ] 3.1 修改 `create_match` handler：插入 opponent_email 和 opponent_name
- [ ] 3.2 有 opponent 時自動產生 claim_token（Uuid::new_v4）
- [ ] 3.3 若 opponent_email 有值，嘗試查找已註冊使用者並填入 opponent_user_id
- [ ] 3.4 更新 SQL INSERT 語句包含 opponent 欄位

## 4. Claim 端點

- [ ] 4.1 新增 `claim_match` handler：POST /api/matches/claim
- [ ] 4.2 驗證 token 存在且 opponent_user_id 尚未填入
- [ ] 4.3 設定 opponent_user_id = 當前使用者 id，清除 claim_token
- [ ] 4.4 回傳認領成功的 match 資訊

## 5. 查詢更新

- [ ] 5.1 更新 `list_matches` SQL：SELECT 包含 opponent 欄位
- [ ] 5.2 更新 `get_match` SQL：SELECT 包含 opponent 欄位
- [ ] 5.3 MatchResponse 序列化包含 opponent 資訊

## 6. Auto-Match

- [ ] 6.1 在使用者建立時（auth/me handler），查詢 opponent_email 匹配的 matches
- [ ] 6.2 批次更新匹配的 matches：SET opponent_user_id = new_user_id

## 7. Route 註冊

- [ ] 7.1 在 matches/mod.rs 新增 POST /matches/claim route
- [ ] 7.2 確保 claim route 需要認證（AuthUser extractor）

## 8. 測試

- [ ] 8.1 測試建立含 opponent 資訊的 match
- [ ] 8.2 測試 claim flow（建立 → 認領 → 驗證）
- [ ] 8.3 測試重複認領拒絕（已有 opponent_user_id）
- [ ] 8.4 測試無效 claim token
- [ ] 8.5 測試 auto-match（使用者註冊時自動連結）
