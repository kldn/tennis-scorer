# Tennis Scorer System Architecture Design

Date: 2026-02-13

## Design Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Frontend platforms | Apple Watch (Swift), Wear OS (Flutter), Phone (Flutter) | Cross-platform with native watch experiences |
| Watch-Phone relationship | Watch records, Phone views | Watch is scoring device, Phone for stats/social |
| Data sync | Watch direct to cloud | Simpler architecture, offline-first + sync when online |
| Phone data source | Pure cloud API | Data is small (~3KB/match), network latency acceptable |
| Authentication | Firebase Auth (Google + Apple Sign-In) | Cross-platform, pairs with FCM, eliminates custom JWT |
| Social features | Friend system | Friends, match history, head-to-head records |
| Push notifications | Firebase Cloud Messaging (FCM) | Free, Flutter-native, shares Firebase project |
| Opponent linking | Claim token + auto email match | Supports unregistered opponents, links on registration |

---

## 1. System Overview

```
┌─────────────────────────────────────────────────────────────┐
│                        CLIENTS                               │
│                                                              │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────┐   │
│  │ Apple Watch   │  │ Wear OS      │  │ Phone App        │   │
│  │ Swift/SwiftUI │  │ Flutter      │  │ Flutter          │   │
│  │ + UniFFI      │  │ + dart:ffi   │  │ (Pure API Client)│   │
│  │              │  │              │  │                  │   │
│  │ Rust Engine  │  │ Rust Engine  │  │  No Rust Engine  │   │
│  │ (local score)│  │ (local score)│  │  (view/social)   │   │
│  └──────┬───────┘  └──────┬───────┘  └────────┬─────────┘   │
│         │                 │                    │              │
└─────────┼─────────────────┼────────────────────┼──────────────┘
          │ HTTPS           │ HTTPS              │ HTTPS
          ▼                 ▼                    ▼
┌─────────────────────────────────────────────────────────────┐
│                     CLOUD BACKEND                            │
│                                                              │
│  ┌─────────────────────────────────────────────────────┐    │
│  │              Rust Axum REST API                      │    │
│  │  ┌────────┐ ┌────────┐ ┌────────┐ ┌─────────────┐  │    │
│  │  │ Auth   │ │Matches │ │ Stats  │ │  Social     │  │    │
│  │  │ Module │ │ Module │ │ Module │ │  Module     │  │    │
│  │  └────────┘ └────────┘ └────────┘ └─────────────┘  │    │
│  └──────────────────────┬──────────────────────────────┘    │
│                         │                                    │
│              ┌──────────┼──────────┐                        │
│              ▼          ▼          ▼                        │
│         ┌────────┐ ┌────────┐ ┌────────┐                   │
│         │PostgreSQL│ │  FCM   │ │ Rust   │                   │
│         │ Database │ │ Service│ │ Engine │                   │
│         └────────┘ └────────┘ └────────┘                   │
│                                (server-side                 │
│                                 analysis)                    │
└─────────────────────────────────────────────────────────────┘
```

### Core Design Principles

- **Watch is the scoring device**: Embeds Rust engine, offline-first, syncs to cloud after match
- **Phone is the viewing device**: Pure API client, no Rust engine, handles stats/history/social
- **Backend is the data center**: Storage, analysis computation (for phone), social, push notifications

---

## 2. Frontend Architecture - Watch Apps

```
┌─────────────────────────────────────────────────────────────┐
│                    APPLE WATCH APP                            │
│                    (Swift/SwiftUI)                            │
│                                                              │
│  ┌─────────────────────────────────────────────────────┐    │
│  │                   UI Layer                           │    │
│  │  ┌─────────────┐ ┌──────────────┐ ┌──────────────┐  │    │
│  │  │ ContentView │ │MatchHistory  │ │  AuthView    │  │    │
│  │  │ (scoring)   │ │  View        │ │              │  │    │
│  │  └──────┬──────┘ └──────┬───────┘ └──────┬───────┘  │    │
│  └─────────┼───────────────┼────────────────┼───────────┘    │
│            │               │                │                │
│  ┌─────────▼───────────────▼────────────────▼───────────┐    │
│  │              Service Layer                            │    │
│  │  ┌─────────────┐ ┌──────────────┐ ┌──────────────┐  │    │
│  │  │TennisMatch  │ │ SyncService  │ │ Speech       │  │    │
│  │  │(UniFFI wrap) │ │ (HTTP sync)  │ │ Recognizer   │  │    │
│  │  └──────┬──────┘ └──────┬───────┘ └──────────────┘  │    │
│  └─────────┼───────────────┼────────────────────────────┘    │
│            │               │                                 │
│  ┌─────────▼───────────┐   │  ┌──────────────────────┐      │
│  │   Rust Engine       │   │  │   SwiftData          │      │
│  │   (via UniFFI)      │   └──▶   (local persistence)│      │
│  │  ┌───────────────┐  │      │  ┌────────────────┐  │      │
│  │  │ Scoring       │  │      │  │ MatchRecord    │  │      │
│  │  │ Analysis      │  │      │  │ EventRecord    │  │      │
│  │  │ Replay        │  │      │  └────────────────┘  │      │
│  │  └───────────────┘  │      └──────────────────────┘      │
│  └─────────────────────┘                                     │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│                   WEAR OS WATCH APP                          │
│                   (Flutter)                                   │
│                                                              │
│  ┌─────────────────────────────────────────────────────┐    │
│  │                   UI Layer (Flutter Widgets)          │    │
│  │  ┌─────────────┐ ┌──────────────┐ ┌──────────────┐  │    │
│  │  │ ScoreScreen │ │HistoryScreen │ │ AuthScreen   │  │    │
│  │  └──────┬──────┘ └──────┬───────┘ └──────┬───────┘  │    │
│  └─────────┼───────────────┼────────────────┼───────────┘    │
│            │               │                │                │
│  ┌─────────▼───────────────▼────────────────▼───────────┐    │
│  │              Service Layer                            │    │
│  │  ┌─────────────┐ ┌──────────────┐ ┌──────────────┐  │    │
│  │  │TennisMatch  │ │ SyncService  │ │ Speech       │  │    │
│  │  │(dart:ffi)   │ │ (HTTP sync)  │ │ Service      │  │    │
│  │  └──────┬──────┘ └──────┬───────┘ └──────────────┘  │    │
│  └─────────┼───────────────┼────────────────────────────┘    │
│            │               │                                 │
│  ┌─────────▼───────────┐   │  ┌──────────────────────┐      │
│  │   Rust Engine       │   │  │   SQLite / Hive      │      │
│  │   (via dart:ffi)    │   └──▶   (local persistence)│      │
│  │  ┌───────────────┐  │      │  ┌────────────────┐  │      │
│  │  │ Scoring       │  │      │  │ MatchRecord    │  │      │
│  │  │ Analysis      │  │      │  │ EventRecord    │  │      │
│  │  └───────────────┘  │      │  └────────────────┘  │      │
│  └─────────────────────┘      └──────────────────────┘      │
└─────────────────────────────────────────────────────────────┘
```

### Key Design Points

- **Symmetric architecture**: Both watch apps have 3 layers (UI → Service → Engine + Storage)
- **Different Rust bindings**: Apple Watch uses UniFFI (auto-generated Swift), Wear OS uses dart:ffi (via `flutter_rust_bridge`)
- **Different local storage**: Apple Watch uses SwiftData, Wear OS uses SQLite or Hive
- **Voice input**: Apple Watch uses native Speech Framework, Wear OS uses Flutter speech-to-text plugin
- **Consistent sync logic**: Both are offline-first + HTTP POST to cloud when online, using `client_id` for idempotency

---

## 3. Frontend Architecture - Phone App (Flutter)

```
┌──────────────────────────────────────────────────────────────┐
│                    PHONE APP (Flutter)                         │
│                    Android / iOS                              │
│                                                               │
│  ┌──────────────────────────────────────────────────────┐    │
│  │                  UI Layer (Flutter Widgets)            │    │
│  │                                                       │    │
│  │  ┌───────────┐ ┌───────────┐ ┌───────────┐          │    │
│  │  │ Dashboard  │ │ Match     │ │ Social    │          │    │
│  │  │ Screen    │ │ Detail    │ │ Screen    │          │    │
│  │  │ (overview)│ │ Screen    │ │(friends)  │          │    │
│  │  └───────────┘ └───────────┘ └───────────┘          │    │
│  │  ┌───────────┐ ┌───────────┐ ┌───────────┐          │    │
│  │  │ History   │ │ Analysis  │ │ Settings  │          │    │
│  │  │ Screen    │ │ Screen    │ │ Screen    │          │    │
│  │  │(match list)│ │(charts)  │ │(account)  │          │    │
│  │  └───────────┘ └───────────┘ └───────────┘          │    │
│  └────────────────────────┬─────────────────────────────┘    │
│                           │                                   │
│  ┌────────────────────────▼─────────────────────────────┐    │
│  │              State Management (Riverpod)              │    │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌────────┐  │    │
│  │  │ AuthState│ │MatchState│ │StatsState│ │Social  │  │    │
│  │  │ Provider │ │ Provider │ │ Provider │ │Provider│  │    │
│  │  └──────────┘ └──────────┘ └──────────┘ └────────┘  │    │
│  └────────────────────────┬─────────────────────────────┘    │
│                           │                                   │
│  ┌────────────────────────▼─────────────────────────────┐    │
│  │              Repository Layer                         │    │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌────────┐  │    │
│  │  │ AuthRepo │ │MatchRepo │ │ StatsRepo│ │Social  │  │    │
│  │  │          │ │          │ │          │ │Repo    │  │    │
│  │  └──────────┘ └──────────┘ └──────────┘ └────────┘  │    │
│  └────────────────────────┬─────────────────────────────┘    │
│                           │                                   │
│  ┌────────────────────────▼─────────────────────────────┐    │
│  │              Network Layer                            │    │
│  │  ┌───────────────────────────────────────────────┐   │    │
│  │  │  API Client (dio / http)                       │   │    │
│  │  │  - Base URL config                             │   │    │
│  │  │  - Firebase token interceptor (auto attach)    │   │    │
│  │  │  - Error handling                              │   │    │
│  │  └───────────────────────────────────────────────┘   │    │
│  └──────────────────────────────────────────────────────┘    │
│                                                               │
│  ┌──────────────────────────────────────────────────────┐    │
│  │  FCM Integration (firebase_messaging)                 │    │
│  │  - Friend request notifications                       │    │
│  │  - Friend match result notifications                  │    │
│  │  - Match claim notifications                          │    │
│  └──────────────────────────────────────────────────────┘    │
└──────────────────────────────────────────────────────────────┘
```

### Key Design Points

- **4-layer architecture**: UI → State Management → Repository → Network
- **Riverpod** for state management: Type-safe, testable, Flutter community recommended
- **Repository abstraction**: Isolates API details, easy to add local cache later without changing upper layers
- **Firebase token interceptor**: Automatically attaches Firebase ID token to requests
- **No Rust engine**: Phone relies entirely on cloud API, keeping it simple

---

## 4. Backend Architecture (Internal)

```
┌──────────────────────────────────────────────────────────────────┐
│                     RUST AXUM BACKEND                             │
│                                                                   │
│  ┌────────────────────────────────────────────────────────────┐  │
│  │                    Middleware Pipeline                       │  │
│  │  ┌─────────┐ ┌──────────┐ ┌───────────┐ ┌──────────────┐  │  │
│  │  │  CORS   │ │ Logging  │ │Rate Limit │ │ Firebase     │  │  │
│  │  │         │ │(tracing) │ │(tower)    │ │ Token Verify │  │  │
│  │  └─────────┘ └──────────┘ └───────────┘ └──────────────┘  │  │
│  └────────────────────────────┬───────────────────────────────┘  │
│                               │                                   │
│  ┌────────────────────────────▼───────────────────────────────┐  │
│  │                     Router Layer                             │  │
│  │                                                              │  │
│  │  /api/auth/*          → Auth Handlers                       │  │
│  │  /api/matches/*       → Match Handlers                      │  │
│  │  /api/stats/*         → Stats Handlers                      │  │
│  │  /api/social/*        → Social Handlers        [NEW]        │  │
│  │  /api/notifications/* → Notification Handlers  [NEW]        │  │
│  │  /api/health          → Health Check                        │  │
│  └────────────────────────────┬───────────────────────────────┘  │
│                               │                                   │
│  ┌────────────────────────────▼───────────────────────────────┐  │
│  │                   Service Layer (Business Logic)              │  │
│  │                                                              │  │
│  │  ┌────────────┐ ┌────────────┐ ┌─────────────────────────┐ │  │
│  │  │ AuthService│ │MatchService│ │ StatsService            │ │  │
│  │  │ - get/     │ │ - create   │ │ - summary               │ │  │
│  │  │   create   │ │ - list     │ │ - match_analysis        │ │  │
│  │  │   user     │ │ - get      │ │ - momentum              │ │  │
│  │  │            │ │ - delete   │ │ - pace                  │ │  │
│  │  │            │ │ - claim    │ │                         │ │  │
│  │  └────────────┘ └────────────┘ └─────────────────────────┘ │  │
│  │                                                              │  │
│  │  ┌─────────────────────┐ ┌─────────────────────────────┐   │  │
│  │  │ SocialService [NEW] │ │ NotificationService [NEW]   │   │  │
│  │  │ - send_request      │ │ - register_device_token     │   │  │
│  │  │ - accept/reject     │ │ - send_push (via FCM API)   │   │  │
│  │  │ - list_friends      │ │ - notify_friend_request     │   │  │
│  │  │ - friend_matches    │ │ - notify_match_result       │   │  │
│  │  │ - head_to_head      │ │                             │   │  │
│  │  └─────────────────────┘ └─────────────────────────────┘   │  │
│  └────────────────────────────┬───────────────────────────────┘  │
│                               │                                   │
│  ┌────────────────────────────▼───────────────────────────────┐  │
│  │                   Data Access Layer (sqlx + PostgreSQL)       │  │
│  └────────────────────────────┬───────────────────────────────┘  │
│                               │                                   │
│  ┌────────────────────────────▼───────────────────────────────┐  │
│  │                   Core Engine (tennis-scorer crate)           │  │
│  │  replay_with_context() | compute_statistics()                │  │
│  │  compute_momentum()    | compute_pace()                      │  │
│  └────────────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────────────┘
```

### Key Design Points

- **4-layer architecture**: Middleware → Router → Service → Data Access, plus Core Engine
- **New Service Layer**: Extract business logic from handlers (currently handlers access DB directly)
- **New modules**: Social (friend system) and Notification (FCM push)
- **Shared Core Engine**: Backend depends on `tennis-scorer` crate for analysis
- **Firebase Token Verify**: Replaces custom JWT middleware, validates Firebase ID tokens

---

## 5. Database Schema

```
┌─────────────────────── Core Tables ─────────────────────────┐
│                                                              │
│  ┌──────────────┐         ┌──────────────────────┐          │
│  │    users     │         │      matches          │          │
│  │──────────────│    1:N  │──────────────────────│          │
│  │ id (PK)      │◄────────│ user_id (FK)         │          │
│  │ firebase_uid │ [CHG]   │ id (PK)              │          │
│  │ email        │         │ client_id (UNIQUE)   │          │
│  │ display_name │ [NEW]   │ match_type           │          │
│  │ avatar_url   │ [NEW]   │ config (JSONB)       │          │
│  │ created_at   │         │ winner               │          │
│  └──────────────┘         │ player1_sets         │          │
│                           │ player2_sets         │          │
│                           │ opponent_user_id [NEW]│          │
│                           │ opponent_email   [NEW]│          │
│                           │ opponent_name   [NEW] │          │
│                           │ opponent_claim_  [NEW]│          │
│                           │   token               │          │
│                           │ started_at           │          │
│                           │ ended_at             │          │
│                           └───────────┬──────────┘          │
│                                       │ 1:N                 │
│                           ┌───────────▼──────────┐          │
│                           │   match_events       │          │
│                           │ id | match_id | ...  │          │
│                           └──────────────────────┘          │
└──────────────────────────────────────────────────────────────┘

┌─────────────────────── Social Tables [NEW] ─────────────────┐
│                                                              │
│  ┌──────────────────────┐  ┌──────────────────────────┐     │
│  │   friend_requests    │  │     friendships           │     │
│  │──────────────────────│  │──────────────────────────│     │
│  │ id (PK)              │  │ id (PK)                  │     │
│  │ from_user_id (FK)    │  │ user_id (FK)             │     │
│  │ to_user_id (FK)      │  │ friend_id (FK)           │     │
│  │ status (pending/     │  │ created_at               │     │
│  │  accepted/rejected)  │  │ UNIQUE(user_id,friend_id)│     │
│  │ created_at           │  └──────────────────────────┘     │
│  └──────────────────────┘                                    │
│  * friendships stored bidirectionally (A→B and B→A)          │
└──────────────────────────────────────────────────────────────┘

┌─────────────────────── Push Table [NEW] ────────────────────┐
│                                                              │
│  ┌──────────────────────┐                                    │
│  │    device_tokens     │                                    │
│  │ id | user_id (FK)    │                                    │
│  │ fcm_token | device_  │                                    │
│  │ type (watch_apple/   │                                    │
│  │ watch_wearos/        │                                    │
│  │ phone_android/       │                                    │
│  │ phone_ios)           │                                    │
│  │ created_at |         │                                    │
│  │ updated_at           │                                    │
│  └──────────────────────┘                                    │
└──────────────────────────────────────────────────────────────┘
```

---

## 6. Complete Data Flow Architecture

```
                    ┌───────────────┐
                    │   Firebase    │
                    │  ┌─────────┐  │
                    │  │  Auth   │  │
                    │  │(Google/ │  │
                    │  │ Apple)  │  │
                    │  └────┬────┘  │
                    │       │       │
                    │  ┌────▼────┐  │
                    │  │  FCM    │  │
                    │  │ (Push)  │  │
                    │  └────┬────┘  │
                    └───────┼───────┘
                       ▲  ▲ │ Push
          ID Token ────┘  │ │ Notifications
          verify          │ │
                          │ ▼
  ┌────────────┐    ┌─────┴──────────────────────────────────┐
  │            │    │         RUST AXUM BACKEND               │
  │  Apple     │    │                                         │
  │  Watch     │    │  ┌───────────────────────────────────┐  │
  │ ┌────────┐ │    │  │        Middleware Pipeline         │  │
  │ │ Rust   │ │    │  │  CORS → Logging → Rate Limit      │  │
  │ │ Engine │ │    │  │  → Firebase Token Verify           │  │
  │ │(UniFFI)│ │    │  └──────────────┬────────────────────┘  │
  │ ├────────┤ │    │                 │                        │
  │ │SwiftDa-│ │    │  ┌──────────────▼────────────────────┐  │
  │ │ta (DB) │ │    │  │         Router Layer               │  │
  │ └────────┘ │    │  │                                    │  │
  │            │    │  │  /auth/me         GET              │  │
  │  HTTPS ────────►│  │  /matches         POST/GET         │  │
  │  ① sync    │    │  │  /matches/{id}    GET/DELETE       │  │
  │            │    │  │  /matches/claim   POST              │  │
  └────────────┘    │  │  /stats/*         GET              │  │
                    │  │  /social/*        POST/GET         │  │
  ┌────────────┐    │  │  /notifications/* POST/PUT         │  │
  │            │    │  └──────────────┬────────────────────┘  │
  │  Wear OS   │    │                 │                        │
  │  Watch     │    │  ┌──────────────▼────────────────────┐  │
  │ ┌────────┐ │    │  │        Service Layer               │  │
  │ │ Rust   │ │    │  │  Auth | Match | Stats             │  │
  │ │ Engine │ │    │  │  Social | Notification            │  │
  │ │(dart:  │ │    │  └──────────────┬────────────────────┘  │
  │ │ ffi)   │ │    │                 │                        │
  │ ├────────┤ │    │  ┌──────────────▼────────────────────┐  │
  │ │SQLite  │ │    │  │  Data Access (sqlx + PostgreSQL)   │  │
  │ └────────┘ │    │  └──────────────┬────────────────────┘  │
  │            │    │                 │                        │
  │  HTTPS ────────►│  ┌──────────────▼────────────────────┐  │
  │  ① sync    │    │  │  Core Engine (tennis-scorer crate) │  │
  │            │    │  └───────────────────────────────────┘  │
  └────────────┘    └─────────────────────────────────────────┘
                                  ▲
  ┌────────────┐                  │
  │  Phone     │    HTTPS         │
  │  App       │──────────────────┘
  │ (Flutter)  │  ② view stats
  │            │  ③ social
  │  ◄── FCM ──── ④ push
  └────────────┘
```

### Data Flow Reference

| # | Flow | Timing | Data | Notes |
|---|------|--------|------|-------|
| ① | Watch → Server (match sync) | After match / when online | POST /api/matches { config, events[], opponent_info } | offline-first, client_id idempotency |
| ② | Phone → Server (view stats) | User opens app | GET /api/matches, GET /api/stats/* | Read-only, server computes with Rust Engine |
| ③ | Phone → Server (social) | Friend actions | POST/GET /api/social/* | Triggers FCM push to recipient |
| ④ | Server → Phone (push) | Friend request / match result / claim | FCM Push → firebase_messaging | Server calls FCM HTTP API |
| ⑤ | Server ↔ Firebase Auth | Every API request | Firebase ID Token verification | Stateless, verified per request |

### Authentication Flow

```
Client → Firebase (Google/Apple Sign-In) → Firebase ID Token
Client → Server (Authorization: Bearer <firebase_token>)
Server → Firebase Admin (verify token) → { uid, email }
Server → Find or create user by firebase_uid
```

### Opponent Claim Flow

```
Match creation:  opponent_user_id: NULL, claim_token: "abc123"
                 Share claim URL to opponent

Scenario A (active claim):
  Opponent registers → POST /api/matches/claim { token }
  → Sets opponent_user_id, clears claim_token

Scenario B (auto-match):
  Opponent registers with same email
  → System auto-links matches where opponent_email = user.email
```

---

## 7. API Endpoints

| Category | Endpoint | Method | Description |
|----------|----------|--------|-------------|
| **Auth** | `/api/auth/me` | GET | Get/create current user (Firebase token) |
| **Matches** | `/api/matches` | POST | Create match (with optional opponent info) |
| | `/api/matches` | GET | List matches (paginated) |
| | `/api/matches/{id}` | GET | Match detail with events |
| | `/api/matches/{id}` | DELETE | Delete match |
| | `/api/matches/claim` | POST | Claim match as opponent |
| **Stats** | `/api/stats/summary` | GET | Win/loss summary, streak, recent form |
| | `/api/stats/match/{id}/analysis` | GET | Full match analysis |
| | `/api/stats/match/{id}/momentum` | GET | Momentum data |
| | `/api/stats/match/{id}/pace` | GET | Pace/timing data |
| **Social** | `/api/social/friend-request` | POST | Send friend request |
| | `/api/social/friend-request/{id}` | POST | Accept/reject request |
| | `/api/social/friends` | GET | List friends |
| | `/api/social/friends/{id}/matches` | GET | Friend's matches |
| | `/api/social/head-to-head/{id}` | GET | Head-to-head record |
| **Notification** | `/api/notifications/register` | POST | Register device FCM token |
| | `/api/notifications/settings` | PUT | Notification preferences |

---

## 8. Deployment & CI/CD Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      GitHub Repository                       │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌────────────┐  │
│  │ crates/  │  │ WatchApp/│  │ flutter/ │  │ .github/   │  │
│  │ (Rust)   │  │ (Swift)  │  │ (Dart)   │  │ workflows/ │  │
│  └──────────┘  └──────────┘  └──────────┘  └─────┬──────┘  │
└──────────────────────────────────────────────────┬┘          │
                                                   │ Push/PR   │
                              ┌────────────────────┘           │
                              ▼                                │
┌─────────────────────────────────────────────────────────┐   │
│                   GitHub Actions                         │   │
│                                                          │   │
│  rust.yml (every push/PR)                                │   │
│  ├─ cargo fmt --check                                    │   │
│  ├─ cargo clippy -- -D warnings                          │   │
│  ├─ cargo test                                           │   │
│  └─ cargo build --release                                │   │
│                                                          │   │
│  deploy.yml (master push)                                │   │
│  ├─ cargo test -p tennis-scorer-api                      │   │
│  └─ railway up --detach (Dockerfile multi-stage)         │   │
│                                                          │   │
│  watchos.yml (WatchApp changes)                          │   │
│  ├─ build-watchos.sh (UniFFI → Swift binding)            │   │
│  └─ xcodebuild test                                     │   │
│                                                          │   │
│  flutter.yml [NEW] (flutter/ changes)                    │   │
│  ├─ flutter analyze                                      │   │
│  ├─ flutter test                                         │   │
│  ├─ flutter build apk                                    │   │
│  └─ flutter build ios                                    │   │
└─────────────────────────────────────────────────────────┘   │
                              │ Deploy                         │
                              ▼                                │
┌─────────────────────────────────────────────────────────┐   │
│                Production Environment                    │   │
│                                                          │   │
│  ┌──────────────────┐  ┌─────────────────────────────┐  │   │
│  │     Railway      │  │        Firebase              │  │   │
│  │  ┌────────────┐  │  │  ┌───────────────────────┐  │  │   │
│  │  │ Rust API   │  │  │  │ Auth (Google/Apple)    │  │  │   │
│  │  │ (Docker)   │  │  │  └───────────────────────┘  │  │   │
│  │  │ Port 8000  │  │  │  ┌───────────────────────┐  │  │   │
│  │  └────────────┘  │  │  │ Cloud Messaging (FCM)  │  │  │   │
│  │  ┌────────────┐  │  │  └───────────────────────┘  │  │   │
│  │  │ PostgreSQL │  │  └─────────────────────────────┘  │   │
│  │  │ (Managed)  │  │                                    │   │
│  │  └────────────┘  │  ┌─────────────────────────────┐  │   │
│  └──────────────────┘  │     App Distribution         │  │   │
│                        │  Google Play (Flutter phone)  │  │   │
│                        │  App Store (Flutter phone)    │  │   │
│                        │  App Store (watchOS)          │  │   │
│                        │  Google Play (Wear OS)        │  │   │
│                        └─────────────────────────────┘  │   │
└─────────────────────────────────────────────────────────┘   │
└──────────────────────────────────────────────────────────────┘
```

### Environment Variables

```
Railway:
  DATABASE_URL          → Railway managed PostgreSQL
  FIREBASE_PROJECT_ID   → Firebase project ID
  HOST                  → 0.0.0.0
  PORT                  → 8000

GitHub Secrets:
  RAILWAY_TOKEN         → Railway deploy token
  FIREBASE_SERVICE_KEY  → Firebase Admin SDK key
```

---

## 9. Migration Path from Current State

### Phase 1: Backend Evolution
1. Replace Argon2 + custom JWT with Firebase Auth token verification
2. Add opponent fields to matches table
3. Extract Service Layer from handlers
4. Add match claim endpoint

### Phase 2: Social & Notifications
5. Add friend_requests, friendships, device_tokens tables
6. Implement Social module (friend CRUD, head-to-head)
7. Integrate FCM for push notifications
8. Implement Notification module

### Phase 3: Cross-Platform Frontend
9. Create Flutter phone app (pure API client)
10. Create Flutter Wear OS watch app (with Rust engine via dart:ffi)
11. Add flutter.yml CI pipeline
12. Set up app distribution (Google Play, App Store)
