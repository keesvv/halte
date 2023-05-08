//
//  HalteApp.swift
//  Halte
//
//  Created by Kees van Voorthuizen on 08/05/2023.
//

import SwiftUI

@main
struct HalteApp: App {
    var departure: Date = Date().advanced(by: (60 * 5) + 1)
    var canonicalDeparture: String {
        return RelativeDateTimeFormatter().localizedString(
            fromTimeInterval: departure.timeIntervalSinceNow)
    }

    var body: some Scene {
        MenuBarExtra("Halte", systemImage: "tram.fill") {
            VStack {
                Text("Next departure in \(canonicalDeparture)")
            }
        }
    }
}
