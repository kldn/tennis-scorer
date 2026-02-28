import AuthenticationServices
import SwiftUI

struct AuthView: View {
    @Binding var isLoggedIn: Bool
    @State private var errorMessage: String?
    @State private var isLoading = false

    var body: some View {
        ScrollView {
            VStack(spacing: 16) {
                Text("登入")
                    .font(.headline)

                if let error = errorMessage {
                    Text(error)
                        .font(.caption2)
                        .foregroundColor(.red)
                        .multilineTextAlignment(.center)
                }

                SignInWithAppleButton(.signIn) { request in
                    request.requestedScopes = [.email]
                } onCompletion: { result in
                    Task { await handleAppleSignIn(result) }
                }
                .signInWithAppleButtonStyle(.white)
                .frame(height: 45)
            }
            .padding()
        }
        .disabled(isLoading)
    }

    private func handleAppleSignIn(_ result: Result<ASAuthorization, Error>) async {
        isLoading = true
        errorMessage = nil
        defer { isLoading = false }

        switch result {
        case .success(let authorization):
            guard let credential = authorization.credential as? ASAuthorizationAppleIDCredential,
                  let identityTokenData = credential.identityToken,
                  let identityToken = String(data: identityTokenData, encoding: .utf8)
            else {
                errorMessage = "無法取得 Apple 認證資訊"
                return
            }

            do {
                let _ = try await APIClient.shared.loginWithApple(identityToken: identityToken)
                isLoggedIn = true
            } catch {
                errorMessage = "登入失敗，請重試"
            }

        case .failure(let error):
            let nsError = error as NSError
            if nsError.domain == ASAuthorizationError.errorDomain
                && nsError.code == ASAuthorizationError.canceled.rawValue {
                // User cancelled — do nothing
                return
            }
            errorMessage = "Apple 登入失敗"
        }
    }
}
