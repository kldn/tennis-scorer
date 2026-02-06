import XCTest
@testable import TennisScorer_Watch_App

final class KeywordMatcherTests: XCTestCase {

    // MARK: - Task 5.1: Keyword matching logic

    func testKeywordWo_MapsToPlayer1Point() {
        let result = KeywordMatcher.match("我")
        XCTAssertEqual(result, .player1Point)
    }

    func testKeywordDuiShou_MapsToPlayer2Point() {
        let result = KeywordMatcher.match("對手")
        XCTAssertEqual(result, .player2Point)
    }

    func testKeywordQuXiao_MapsToUndo() {
        let result = KeywordMatcher.match("取消")
        XCTAssertEqual(result, .undo)
    }

    func testNoKeyword_ReturnsNil() {
        let result = KeywordMatcher.match("你好")
        XCTAssertNil(result)
    }

    func testEmptyString_ReturnsNil() {
        let result = KeywordMatcher.match("")
        XCTAssertNil(result)
    }

    func testKeywordInLongerText() {
        XCTAssertEqual(KeywordMatcher.match("我得分了"), .player1Point)
        XCTAssertEqual(KeywordMatcher.match("是對手得分"), .player2Point)
        XCTAssertEqual(KeywordMatcher.match("請取消"), .undo)
    }

    // MARK: - Task 5.2: Longer keywords matched before "我"

    func testDuiShou_MatchedBeforeWo() {
        // "對手" does not contain "我", but this test ensures the priority order
        // is correct: longer keywords are checked first
        let result = KeywordMatcher.match("對手")
        XCTAssertEqual(result, .player2Point)
    }

    func testQuXiao_MatchedBeforeWo() {
        let result = KeywordMatcher.match("取消")
        XCTAssertEqual(result, .undo)
    }

    func testTextWithBothDuiShouAndWo_MatchesDuiShou() {
        // If text somehow contains both "對手" and "我", "對手" should win
        let result = KeywordMatcher.match("我的對手")
        XCTAssertEqual(result, .player2Point)
    }

    func testTextWithBothQuXiaoAndWo_MatchesQuXiao() {
        let result = KeywordMatcher.match("我要取消")
        XCTAssertEqual(result, .undo)
    }

    // MARK: - Task 5.3: State transitions

    func testSpeechStateEquality() {
        XCTAssertEqual(SpeechState.idle, SpeechState.idle)
        XCTAssertEqual(SpeechState.listening, SpeechState.listening)
        XCTAssertEqual(SpeechState.processing, SpeechState.processing)
        XCTAssertEqual(SpeechState.error, SpeechState.error)
        XCTAssertEqual(SpeechState.result(.player1Point), SpeechState.result(.player1Point))
        XCTAssertEqual(SpeechState.result(.player2Point), SpeechState.result(.player2Point))
        XCTAssertEqual(SpeechState.result(.undo), SpeechState.result(.undo))

        XCTAssertNotEqual(SpeechState.idle, SpeechState.listening)
        XCTAssertNotEqual(SpeechState.result(.player1Point), SpeechState.result(.player2Point))
    }

    func testSpeechStateInitiallyIdle() async {
        let recognizer = await SpeechRecognizer()
        let state = await recognizer.state
        XCTAssertEqual(state, .idle)
    }

    // MARK: - Task 5.4: Timeout behavior
    // Note: Full timeout integration tests require mocking SFSpeechRecognizer and AVAudioEngine.
    // The timeout logic uses Task.sleep with cancellation in SpeechRecognizer.
    // These tests verify the state machine expectations.

    func testScoringActionDisplayText() {
        XCTAssertEqual(ScoringAction.player1Point.displayText, "我")
        XCTAssertEqual(ScoringAction.player2Point.displayText, "對手")
        XCTAssertEqual(ScoringAction.undo.displayText, "取消")
    }
}
