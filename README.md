# Inferoxa

English | [日本語](README_ja.md)

Inferoxa is a gRPC protocol for LLM inference, designed mainly for backend-to-backend communication.

The main deliverable of this project is the set of gRPC definitions under [`proto/`](proto/). Servers that implement the protocol on top of existing inference backends are developed alongside it as reference implementations.

> [!WARNING]
> Inferoxa is in an early stage of development. Although the package is named `inferoxa.v1`, breaking changes may still be made to the proto definitions.

## Goals

- **Provider independent**: Different inference backends, such as llama.cpp and the OpenAI API, are used through the same interface. Differences in what a provider supports are expressed in the protocol itself, for example as model capabilities or as an `INVALID_ARGUMENT` error for an unsupported option.
- **Typed schema**: Requests and responses are defined in Protocol Buffers. Message roles, content types and stream events are modeled as `oneof` fields instead of loosely structured JSON, and clients and servers are generated from the definitions.
- **Made for backend-to-backend communication**: The protocol targets communication between servers rather than browsers or end-user clients, and uses gRPC server streaming for incremental output.

## Services

All definitions are in the `inferoxa.v1` package.

| Service | RPC | Description |
| --- | --- | --- |
| [`ModelsService`](proto/inferoxa/v1/models.proto) | `ListModels` | List the models available on the provider, optionally filtered by capability. |
| [`EmbeddingsService`](proto/inferoxa/v1/embeddings.proto) | `CreateEmbeddings` | Create embeddings of the input texts. |
| [`CompletionsService`](proto/inferoxa/v1/completions.proto) | `CreateChatCompletion` | Generate the next assistant message of a chat conversation. |
| | `CreateChatCompletionStream` | Same as `CreateChatCompletion`, but returned incrementally. |
| | `CreateTextCompletion` | Generate the continuation of a raw text prompt. No chat template is applied. |
| | `CreateTextCompletionStream` | Same as `CreateTextCompletion`, but returned incrementally. |

Chat completions support tool calls, reasoning contents, and file inputs such as images. The comments in the proto files are the specification of each field.

## Using the protocol

### Proto files

Take the [`proto/`](proto/) directory into your project, for example by copying it or as a git submodule, and generate code with the gRPC tooling of your language. Use `proto/` as the import path.

```bash
protoc -I proto --python_out=gen proto/inferoxa/v1/*.proto
```

The definitions are also planned to be published to the [Buf Schema Registry](https://buf.build/).

### Rust

[`inferoxa-proto`](crates/inferoxa-proto) provides code generated with [tonic](https://github.com/hyperium/tonic). It is planned to be published to crates.io.

What is generated is selected with features. Nothing is enabled by default.

| Feature | Description |
| --- | --- |
| `models` | Messages and service of `models.proto` |
| `embeddings` | Messages and service of `embeddings.proto` |
| `completions` | Messages and service of `completions.proto` |
| `server` | Generate server code and the file descriptor set (`FILE_DESCRIPTOR_SET`, usable for gRPC reflection) |
| `client` | Generate client code |

The generated code is in `inferoxa_proto::inferoxa::v1`. `tonic`, `prost` and `prost_types` are re-exported from the crate.

## Reference implementations

| Crate | Backend | Status |
| --- | --- | --- |
| [`inferoxa-llama-server`](crates/inferoxa-llama-server) | [llama.cpp](https://github.com/ggml-org/llama.cpp) | In development |
| `inferoxa-openai-api` | OpenAI API | Planned |

Both are Inferoxa gRPC servers. `inferoxa-llama-server` runs models locally with llama.cpp, and `inferoxa-openai-api` forwards requests to the OpenAI API.

## Repository layout

```text
proto/inferoxa/v1/             gRPC definitions (the protocol itself)
crates/inferoxa-proto/         Rust code generated from the definitions
crates/inferoxa-llama-server/  Reference implementation on llama.cpp
```

## License

[Apache License 2.0](LICENSE)
