# RS-Swag-Demo

A simple Rust API demo showcasing Actix Web with OpenAPI/Swagger UI integration.

![Rust](https://img.shields.io/badge/Rust-black?style=for-the-badge&logo=rust&logoColor=#E57324)
![Actix](https://img.shields.io/badge/Actix-148EFF?style=for-the-badge&logo=rust&logoColor=white)
![Swagger](https://img.shields.io/badge/Swagger-85EA2D?style=for-the-badge&logo=swagger&logoColor=black)

## 📝 Description

RS-Swag-Demo demonstrates how to build a RESTful API in Rust using the Actix Web framework with auto-generated OpenAPI documentation via Utoipa and Swagger UI. This project serves as a minimal but functional template for creating well-documented Rust web services.

## ✨ Features

- RESTful API endpoints using Actix Web
- Automatic OpenAPI documentation generation with Utoipa
- Interactive API testing through Swagger UI
- Proper type safety with Serde serialization/deserialization
- JSON schema definition with JsonSchema

## 🚀 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.68 or later)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) (comes with Rust)

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/RGGH/rs-swag-demo.git
   cd rs-swag-demo
   ```

2. Build the project:
   ```bash
   cargo build
   ```

3. Run the server:
   ```bash
   cargo run
   ```

4. Access the API at:
   - API Endpoint: [http://127.0.0.1:8080/hello](http://127.0.0.1:8080/hello)
   - Swagger UI: [http://127.0.0.1:8080/swagger-ui/](http://127.0.0.1:8080/swagger-ui/)

## 🧪 Testing

Run the automated tests:
```bash
cargo test
```

### Manual Testing

Test the API endpoint:
```bash
curl http://127.0.0.1:8080/hello
```

Expected response:
```json
{"message":"Hello from module!"}
```

## 🔧 Project Structure

```
rs-swag-demo/
├── src/
│   ├── api.rs        # API endpoints and response models
│   ├── lib.rs        # Library exports for testing
│   └── main.rs       # Server configuration and startup
├── tests/
│   └── api_test.rs   # API integration tests
└── Cargo.toml        # Project dependencies
```

## 📚 Learn More

- [Actix Web documentation](https://actix.rs/)
- [Utoipa documentation](https://docs.rs/utoipa/latest/utoipa/)
- [OpenAPI specification](https://swagger.io/specification/)

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🤝 Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request
