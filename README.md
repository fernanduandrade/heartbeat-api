# Heartbeat API

API simples em Rust para expor o status de saude do servico.

## Endpoint

`GET /health/`

Exemplo de resposta:

```json
{
  "status": "healthy",
  "service": "heartbeat-api",
  "version": "0.1.0",
  "hostname": "heartbeat-api-7df8d6f7d8-n9x2k",
  "pod_name": "heartbeat-api-7df8d6f7d8-n9x2k",
  "pod_namespace": "default",
  "pod_ip": "10.1.2.3",
  "node_name": "worker-01",
  "timestamp": "2026-08-23T12:00:00Z"
}
```

## Rodando localmente

```bash
cargo run
curl http://localhost:8080/health/
```

## Docker

```bash
docker build -t heartbeat-api .
docker run --rm -p 8080:8080 heartbeat-api
```

## CI/CD

O workflow `.github/workflows/ci-cd.yml` roda a cada push na branch `main` quando houver alteracao no codigo, dependencias, Dockerfile ou no proprio workflow.

Ele executa:

1. `cargo fmt --check`
2. `cargo check --locked`
3. build e push da imagem no Docker Hub
4. commit no repositorio `fernanduandrade/heartbeat-iac` com a nova tag da imagem

Configure estes secrets no GitHub deste repositorio:

| Secret | Uso |
| --- | --- |
| `DOCKERHUB_USERNAME` | Usuario/namespace do Docker Hub |
| `DOCKERHUB_TOKEN` | Token de acesso do Docker Hub |
| `IAC_REPO_TOKEN` | PAT do GitHub com permissao de escrita em `fernanduandrade/heartbeat-iac` |

As imagens sao publicadas como:

```text
DOCKERHUB_USERNAME/heartbeat-api:<short-sha>
DOCKERHUB_USERNAME/heartbeat-api:latest
```

## Kubernetes

Configure as variaveis com Downward API para preencher os dados do pod:

```yaml
env:
  - name: POD_NAME
    valueFrom:
      fieldRef:
        fieldPath: metadata.name
  - name: POD_NAMESPACE
    valueFrom:
      fieldRef:
        fieldPath: metadata.namespace
  - name: POD_IP
    valueFrom:
      fieldRef:
        fieldPath: status.podIP
  - name: NODE_NAME
    valueFrom:
      fieldRef:
        fieldPath: spec.nodeName
```
