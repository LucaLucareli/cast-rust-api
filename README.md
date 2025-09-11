# API Rust Monorepo - Sistema de Streaming com JWT, Cache e Banco Real

Este é um projeto monorepo em Rust que implementa um sistema de streaming de vídeos similar ao Netflix, com múltiplas APIs especializadas, autenticação JWT, cache Redis, banco de dados real e sistema de migrações:

- **Auth API** (`/auth`)
- **Admin API** (`/admin`)
- **Viewer API** (`/viewer`)

## Funcionalidades Implementadas

### **Autenticação JWT Completa**
- **Access Token**: Validade de 1 hora (configurável via .env)
- **Refresh Token**: Validade de 7 dias (configurável via .env)
- **Middleware de autenticação**: Protege rotas privadas
- **Middleware de admin**: Verifica permissões de administrador

### **Sistema de Banco de Dados Real**
- **PostgreSQL**: Banco de dados principal
- **Repositórios funcionais**
- **Hash de senhas**: Bcrypt para segurança

### **Sistema de Cache**
- **Cache Redis**: Implementado com TTL configurável
- **Cache automático**: Para endpoints de leitura (ex: catálogo de vídeos)
- **Funções de cache**: SET, GET, DELETE, EXISTS, INCREMENT

### **Repositórios de Banco de Dados**
- **Padrão Repository**: Uma pasta para cada tabela
- **Operações customizadas**: Busca por filtros, contagem, etc.
- **Estrutura organizada**: Fácil manutenção e extensão

### **Logging Automático**
- **Logs de requisições**: Método, URI, status, latência, User-Agent
- **Formato estruturado**: Fácil de ler e processar

## Como Executar

### 1. Pré-requisitos

- Rust 1.70+
- Docker e Docker Compose
- PostgreSQL (via Docker)
- Redis (via Docker)
- Azurite (Azure Storage Emulator via Docker)

### 2. Configuração

1. Copie o arquivo de ambiente:
   ```bash
   cp env.example .env
   ```

2. Edite o arquivo `.env` com suas configurações

### 3. Iniciar Serviços

```bash
docker-compose up -d
```

### 4. Executar as APIs

#### Executar todas as APIs individualmente ou juntas
```bash
# Terminal 1 - Auth API
cargo make --no-workspace start-auth-api

# Terminal 2 - Admin API  
cargo make --no-workspace start-admin-api

# Terminal 3 - Viewer API
cargo make --no-workspace start-viewer-api

# Terminal 4 - API Principal
cargo make --no-workspace start-all
```
