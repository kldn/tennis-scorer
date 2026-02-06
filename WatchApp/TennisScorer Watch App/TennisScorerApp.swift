//
//  TennisScorerApp.swift
//  TennisScorer Watch App
//
//  Created by 大維 on 2026/2/5.
//

import SwiftUI
import SwiftData

@main
struct TennisScorer_Watch_AppApp: App {
    var body: some Scene {
        WindowGroup {
            ContentView()
        }
        .modelContainer(for: [MatchRecord.self, MatchEventRecord.self])
    }
}
