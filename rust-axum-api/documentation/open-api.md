# API Documentation

This project includes comprehensive API documentation using OpenAPI/Swagger. The documentation is automatically generated from code annotations and is accessible through a web interface.

## Accessing the Documentation

- **Swagger UI**: Available at `http://your-server/swagger-ui`
- **Raw OpenAPI Spec**: Available at `http://your-server/api-docs/openapi.json`

## Authentication

All API endpoints are protected with API key authentication. To access the API:

1. Include the `x-api-key` header in your requests
2. Provide your API key as the header value

Example:
```bash
curl -H "x-api-key: your-api-key" http://your-server/users
```

## Development Guide

### Adding New Endpoints

1. Create your handler function with the appropriate route
2. Add OpenAPI documentation using the `#[utoipa::path]` attribute
3. Include the handler in the `ApiDoc` struct's path list
4. Add any new models to the components section

Example:
```rust
#[utoipa::path(
    post,
    path = "/your-endpoint",
    request_body = YourRequestType,
    responses(
        (status = 200, description = "Success", body = YourResponseType),
        (status = 400, description = "Bad Request", body = ErrorResponse)
    ),
    security(
        ("api_key" = [])
    ),
    tag = "your-tag"
)]
async fn your_handler() {
    // Implementation
}
```

### Adding New Models

1. Define your struct
2. Derive the `ToSchema` trait
3. Add examples using schema attributes
4. Include the model in the `ApiDoc` components

Example:
```rust
#[derive(ToSchema)]
struct YourModel {
    #[schema(example = "example value")]
    field: String
}
```

## Dependencies

Make sure these dependencies are in your `Cargo.toml`:

```toml
[dependencies]
utoipa = { version = "4", features = ["axum_extras"] }
utoipa-swagger-ui = { version = "5", features = ["axum"] }
```

## Testing the Documentation

1. Start your server locally
2. Navigate to `http://localhost:your-port/swagger-ui`
3. Try out the endpoints directly from the Swagger UI
4. Verify that all parameters, request bodies, and responses are correctly documented

## Common Issues

1. **Missing Schema**: Ensure all types used in requests/responses derive `ToSchema`
2. **Authentication Errors**: Verify API key is correctly set in the Swagger UI
3. **Outdated Docs**: Remember to rebuild after updating documentation annotations

## Best Practices

1. Include meaningful examples for all fields
2. Provide clear descriptions for endpoints and parameters
3. Document all possible response codes
4. Group related endpoints under appropriate tags
5. Keep documentation in sync with implementation

## Future Improvements

Consider implementing:
- OAuth2 authentication
- Rate limiting documentation
- Request/response examples
- Detailed error codes and messages
- Versioning information

For more detailed information about the OpenAPI specification, visit [OpenAPI Documentation](https://swagger.io/specification/).
