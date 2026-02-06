import SwiftUI

struct AuthView: View {
    @Binding var isLoggedIn: Bool
    @State private var email = ""
    @State private var password = ""
    @State private var isRegistering = false
    @State private var errorMessage: String?
    @State private var isLoading = false

    var body: some View {
        ScrollView {
            VStack(spacing: 12) {
                Text(isRegistering ? "註冊" : "登入")
                    .font(.headline)

                TextField("Email", text: $email)
                    .textContentType(.emailAddress)

                SecureField("密碼", text: $password)
                    .textContentType(isRegistering ? .newPassword : .password)

                if let error = errorMessage {
                    Text(error)
                        .font(.caption2)
                        .foregroundColor(.red)
                        .multilineTextAlignment(.center)
                }

                Button(isRegistering ? "註冊" : "登入") {
                    Task { await authenticate() }
                }
                .buttonStyle(.borderedProminent)
                .disabled(isLoading || email.isEmpty || password.isEmpty)

                Button(isRegistering ? "已有帳號？登入" : "沒有帳號？註冊") {
                    isRegistering.toggle()
                    errorMessage = nil
                }
                .font(.caption2)
                .foregroundColor(.secondary)
            }
            .padding()
        }
    }

    private func authenticate() async {
        isLoading = true
        errorMessage = nil
        defer { isLoading = false }

        do {
            if isRegistering {
                let _ = try await APIClient.shared.register(email: email, password: password)
            } else {
                let _ = try await APIClient.shared.login(email: email, password: password)
            }
            isLoggedIn = true
        } catch {
            errorMessage = isRegistering ? "註冊失敗，請重試" : "登入失敗，請檢查帳號密碼"
        }
    }
}
