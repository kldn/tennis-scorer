## 1. Database Migration

- [ ] 1.1 建立 migration：CREATE TABLE friend_requests（id, from_user_id, to_user_id, status, created_at, UNIQUE）
- [ ] 1.2 同一 migration：CREATE TABLE friendships（id, user_id, friend_id, created_at, UNIQUE）
- [ ] 1.3 新增 indexes：friend_requests(from_user_id), friend_requests(to_user_id), friendships(user_id)

## 2. Models

- [ ] 2.1 建立 `src/social/models.rs`：SendFriendRequestBody（to_user_id 或 email）
- [ ] 2.2 FriendRequestResponse（id, from_user, to_user, status, created_at）
- [ ] 2.3 FriendResponse（id, display_name, avatar_url）
- [ ] 2.4 HeadToHeadResponse（wins, losses, total_matches, recent_matches）
- [ ] 2.5 ActionBody（action: "accept" | "reject"）

## 3. SocialService

- [ ] 3.1 建立 `src/social/service.rs`：SocialService struct（持有 PgPool）
- [ ] 3.2 `send_request(from_user_id, to_user_id_or_email)`：建立 pending 請求
- [ ] 3.3 `accept_request(request_id, user_id)`：更新 status + 雙向插入 friendships
- [ ] 3.4 `reject_request(request_id, user_id)`：更新 status 為 rejected
- [ ] 3.5 `list_friends(user_id)`：查詢 friendships + JOIN users
- [ ] 3.6 `friend_matches(user_id, friend_id)`：查詢好友的比賽（需驗證好友關係）
- [ ] 3.7 `head_to_head(user_id, friend_id)`：計算勝負統計

## 4. Handlers

- [ ] 4.1 建立 `src/social/handlers.rs`
- [ ] 4.2 `send_friend_request`：POST /api/social/friend-request
- [ ] 4.3 `respond_to_request`：POST /api/social/friend-request/{id}
- [ ] 4.4 `list_friends`：GET /api/social/friends
- [ ] 4.5 `get_friend_matches`：GET /api/social/friends/{id}/matches
- [ ] 4.6 `get_head_to_head`：GET /api/social/head-to-head/{id}

## 5. Route 註冊

- [ ] 5.1 建立 `src/social/mod.rs`：定義 routes()
- [ ] 5.2 在 `lib.rs` 新增 `pub mod social;` 和 `.merge(social::routes())`

## 6. 驗證邏輯

- [ ] 6.1 防止自我好友請求（from == to）
- [ ] 6.2 防止重複請求（UNIQUE constraint + 應用層檢查）
- [ ] 6.3 已是好友時拒絕發送請求
- [ ] 6.4 只有 to_user 可以 accept/reject
- [ ] 6.5 friend_matches 需驗證好友關係存在

## 7. 測試

- [ ] 7.1 測試好友請求完整流程（發送 → 接受 → 驗證好友關係）
- [ ] 7.2 測試拒絕好友請求
- [ ] 7.3 測試重複/自我請求拒絕
- [ ] 7.4 測試好友清單查詢
- [ ] 7.5 測試 head-to-head 統計
