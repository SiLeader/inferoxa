# Inferoxa

[English](README.md) | 日本語

Inferoxa は、主にバックエンド間の通信を想定した、LLM 推論のための gRPC プロトコルです。

このプロジェクトの主な成果物は [`proto/`](proto/) 以下の gRPC 定義です。あわせて、既存の推論バックエンドの上にこのプロトコルを実装したサーバーを、リファレンス実装として開発しています。

> [!WARNING]
> Inferoxa は開発初期の段階にあります。パッケージ名は `inferoxa.v1` ですが、proto 定義には今後も破壊的変更が入る可能性があります。

## 目標

- **プロバイダー非依存**: llama.cpp や OpenAI API のような異なる推論バックエンドを、同じインターフェースで扱えます。プロバイダーごとの対応状況の違いは、モデルのケイパビリティや、未対応のオプションに対する `INVALID_ARGUMENT` エラーといった形で、プロトコル自体の中で表現します。
- **型付きスキーマ**: リクエストとレスポンスは Protocol Buffers で定義しています。メッセージのロール、コンテンツの種類、ストリームのイベントは、構造の緩い JSON ではなく `oneof` フィールドとして表現し、クライアントとサーバーのコードは定義から生成します。
- **バックエンド間通信向け**: ブラウザーやエンドユーザーのクライアントではなく、サーバー間の通信を対象としています。逐次的な出力には gRPC のサーバーストリーミングを使います。

## サービス

すべての定義は `inferoxa.v1` パッケージにあります。

| サービス | RPC | 説明 |
| --- | --- | --- |
| [`ModelsService`](proto/inferoxa/v1/models.proto) | `ListModels` | プロバイダーで利用できるモデルの一覧を取得します。ケイパビリティで絞り込めます。 |
| [`EmbeddingsService`](proto/inferoxa/v1/embeddings.proto) | `CreateEmbeddings` | 入力テキストの埋め込みを生成します。 |
| [`CompletionsService`](proto/inferoxa/v1/completions.proto) | `CreateChatCompletion` | チャットの会話に続くアシスタントのメッセージを生成します。 |
| | `CreateChatCompletionStream` | `CreateChatCompletion` と同じ内容を、生成され次第逐次返します。 |
| | `CreateTextCompletion` | 生のテキストプロンプトの続きを生成します。チャットテンプレートは適用しません。 |
| | `CreateTextCompletionStream` | `CreateTextCompletion` と同じ内容を、生成され次第逐次返します。 |

チャット補完は、ツール呼び出し、推論 (reasoning) コンテンツ、画像などのファイル入力に対応しています。各フィールドの仕様は proto ファイル内のコメントに記述しています。

## プロトコルを使う

### proto ファイル

[`proto/`](proto/) ディレクトリを、コピーや git submodule などでプロジェクトに取り込み、使用する言語の gRPC ツールでコードを生成してください。インポートパスには `proto/` を指定します。

```bash
protoc -I proto --python_out=gen proto/inferoxa/v1/*.proto
```

定義は [Buf Schema Registry](https://buf.build/) にも公開する予定です。

### Rust

[`inferoxa-proto`](crates/inferoxa-proto) は、[tonic](https://github.com/hyperium/tonic) で生成したコードを提供するクレートです。crates.io に公開する予定です。

生成する内容は feature で選択します。デフォルトではどれも有効になっていません。

| feature | 説明 |
| --- | --- |
| `models` | `models.proto` のメッセージとサービス |
| `embeddings` | `embeddings.proto` のメッセージとサービス |
| `completions` | `completions.proto` のメッセージとサービス |
| `server` | サーバーのコードと、ファイルディスクリプタセット (`FILE_DESCRIPTOR_SET`、gRPC リフレクションに利用可能) を生成します |
| `client` | クライアントのコードを生成します |

生成されたコードは `inferoxa_proto::inferoxa::v1` にあります。`tonic`、`prost`、`prost_types` はクレートから再エクスポートしています。

## リファレンス実装

| クレート | バックエンド | 状況 |
| --- | --- | --- |
| [`inferoxa-llama-server`](crates/inferoxa-llama-server) | [llama.cpp](https://github.com/ggml-org/llama.cpp) | 開発中 |
| `inferoxa-openai-api` | OpenAI API | 開発予定 |

どちらも Inferoxa の gRPC サーバーです。`inferoxa-llama-server` は llama.cpp でモデルをローカルで実行し、`inferoxa-openai-api` はリクエストを OpenAI API に転送します。

## リポジトリ構成

```text
proto/inferoxa/v1/             gRPC 定義 (プロトコル本体)
crates/inferoxa-proto/         定義から生成した Rust のコード
crates/inferoxa-llama-server/  llama.cpp によるリファレンス実装
```

## ライセンス

[Apache License 2.0](LICENSE)
