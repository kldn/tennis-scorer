import Foundation

actor APIClient {
    static let shared = APIClient()

    // TODO: Make configurable
    private let baseURL = "https://tennis-scorer-api.shuttle.app/api"

    enum APIError: Error {
        case unauthorized
        case network(Error)
        case server(Int, String)
        case invalidResponse
    }

    // MARK: - Auth

    func register(email: String, password: String) async throws -> (accessToken: String, refreshToken: String) {
        let body: [String: String] = ["email": email, "password": password]
        let _ = try await request("POST", path: "/auth/register", body: body, authenticated: false)

        // Auto-login after register
        return try await login(email: email, password: password)
    }

    func login(email: String, password: String) async throws -> (accessToken: String, refreshToken: String) {
        let body: [String: String] = ["email": email, "password": password]
        let data = try await request("POST", path: "/auth/login", body: body, authenticated: false)

        let json = try JSONSerialization.jsonObject(with: data) as? [String: Any]
        guard let accessToken = json?["access_token"] as? String,
              let refreshToken = json?["refresh_token"] as? String else {
            throw APIError.invalidResponse
        }

        KeychainHelper.accessToken = accessToken
        KeychainHelper.refreshToken = refreshToken

        return (accessToken, refreshToken)
    }

    func refreshAccessToken() async throws -> String {
        guard let refreshToken = KeychainHelper.refreshToken else {
            throw APIError.unauthorized
        }

        let body = ["refresh_token": refreshToken]
        let data = try await request("POST", path: "/auth/refresh", body: body, authenticated: false)

        let json = try JSONSerialization.jsonObject(with: data) as? [String: Any]
        guard let newAccessToken = json?["access_token"] as? String else {
            throw APIError.invalidResponse
        }

        KeychainHelper.accessToken = newAccessToken
        return newAccessToken
    }

    // MARK: - Matches

    func uploadMatch(_ payload: [String: Any]) async throws {
        let jsonData = try JSONSerialization.data(withJSONObject: payload)
        let _ = try await requestWithData("POST", path: "/matches", bodyData: jsonData, authenticated: true)
    }

    // MARK: - Internal

    private func request(_ method: String, path: String, body: Any, authenticated: Bool) async throws -> Data {
        let jsonData = try JSONSerialization.data(withJSONObject: body)
        return try await requestWithData(method, path: path, bodyData: jsonData, authenticated: authenticated)
    }

    private func requestWithData(_ method: String, path: String, bodyData: Data, authenticated: Bool) async throws -> Data {
        guard let url = URL(string: baseURL + path) else {
            throw APIError.invalidResponse
        }

        var request = URLRequest(url: url)
        request.httpMethod = method
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")
        request.httpBody = bodyData

        if authenticated, let token = KeychainHelper.accessToken {
            request.setValue("Bearer \(token)", forHTTPHeaderField: "Authorization")
        }

        let (data, response) = try await URLSession.shared.data(for: request)

        guard let httpResponse = response as? HTTPURLResponse else {
            throw APIError.invalidResponse
        }

        switch httpResponse.statusCode {
        case 200...201:
            return data
        case 204:
            return Data()
        case 401:
            if authenticated {
                // Try refresh
                let _ = try await refreshAccessToken()
                // Retry once
                request.setValue("Bearer \(KeychainHelper.accessToken ?? "")", forHTTPHeaderField: "Authorization")
                let (retryData, retryResponse) = try await URLSession.shared.data(for: request)
                guard let retryHttp = retryResponse as? HTTPURLResponse,
                      (200...204).contains(retryHttp.statusCode) else {
                    throw APIError.unauthorized
                }
                return retryData
            }
            throw APIError.unauthorized
        default:
            let message = String(data: data, encoding: .utf8) ?? "Unknown error"
            throw APIError.server(httpResponse.statusCode, message)
        }
    }
}
