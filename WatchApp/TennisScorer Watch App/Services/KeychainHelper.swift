import Foundation
import Security

enum KeychainHelper {
    private static let service = "com.tennisscorer.api"

    static func save(key: String, value: String) {
        guard let data = value.data(using: .utf8) else { return }

        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrService as String: service,
            kSecAttrAccount as String: key,
        ]

        // Delete existing
        SecItemDelete(query as CFDictionary)

        // Add new
        var addQuery = query
        addQuery[kSecValueData as String] = data
        SecItemAdd(addQuery as CFDictionary, nil)
    }

    static func read(key: String) -> String? {
        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrService as String: service,
            kSecAttrAccount as String: key,
            kSecReturnData as String: true,
            kSecMatchLimit as String: kSecMatchLimitOne,
        ]

        var result: AnyObject?
        let status = SecItemCopyMatching(query as CFDictionary, &result)

        guard status == errSecSuccess, let data = result as? Data else {
            return nil
        }

        return String(data: data, encoding: .utf8)
    }

    static func delete(key: String) {
        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrService as String: service,
            kSecAttrAccount as String: key,
        ]
        SecItemDelete(query as CFDictionary)
    }

    // Convenience for token management
    static var accessToken: String? {
        get { read(key: "access_token") }
        set {
            if let value = newValue {
                save(key: "access_token", value: value)
            } else {
                delete(key: "access_token")
            }
        }
    }

    static var refreshToken: String? {
        get { read(key: "refresh_token") }
        set {
            if let value = newValue {
                save(key: "refresh_token", value: value)
            } else {
                delete(key: "refresh_token")
            }
        }
    }
}
