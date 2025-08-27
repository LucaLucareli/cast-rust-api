#!/bin/bash

# Este script orquestra a execução das migrações DENTRO do contêiner Docker do SQL Server.
# Ele roda no seu host, mas transfere os arquivos e executa o processo no contêiner.

set -e  # Para o script se qualquer comando falhar

# Nome do contêiner SQL Server, conforme visto no 'docker ps'
CONTAINER_NAME="rust-cast-sql-sever"

# Caminho da pasta de migrações e do .env no SEU HOST
HOST_MIGRATIONS_DIR="./migrations"
HOST_ENV_FILE=".env"
# Este é o script que será COPIADO e EXECUTADO dentro do Docker.
# Certifique-se de que ele exista na raiz do seu projeto e esteja atualizado (veja o Passo 2).
HOST_INTERNAL_RUN_SCRIPT="./run_migrations_internal.sh"

# --- Verificações Iniciais no Host ---

echo "🔍 Verificando pré-requisitos..."

if [ ! -f "$HOST_ENV_FILE" ]; then
    echo "❌ Erro: Arquivo .env não encontrado no host em '$HOST_ENV_FILE'."
    echo "Crie-o com as variáveis de ambiente necessárias para a conexão do SQL Server (DB_USER, DB_PASSWORD, DB_NAME)."
    exit 1
fi

if [ ! -d "$HOST_MIGRATIONS_DIR" ]; then
    echo "❌ Erro: Diretório de migrações '$HOST_MIGRATIONS_DIR' não encontrado no host."
    echo "Certifique-se de que a pasta 'migrations' existe e contém seus arquivos .sql."
    exit 1
fi

if [ ! -f "$HOST_INTERNAL_RUN_SCRIPT" ]; then
    echo "❌ Erro: Script interno de execução de migrações '$HOST_INTERNAL_RUN_SCRIPT' não encontrado no host."
    echo "Crie o arquivo '$HOST_INTERNAL_RUN_SCRIPT' conforme as instruções do Passo 2."
    exit 1
fi

echo "✅ Todos os pré-requisitos verificados com sucesso!"

# --- Verificação do Docker ---

echo "🐳 Verificando se o Docker está rodando..."
if ! docker info >/dev/null 2>&1; then
    echo "❌ Erro: Docker não está rodando ou não está acessível."
    echo "Inicie o Docker e tente novamente."
    exit 1
fi

# --- Verificação e Status do Contêiner ---

echo "🔍 Verificando o status do contêiner '$CONTAINER_NAME'..."

# Verifica se o contêiner existe
if ! docker ps -a --format "table {{.Names}}" | grep -q "^$CONTAINER_NAME$"; then
    echo "❌ Erro: Contêiner '$CONTAINER_NAME' não encontrado."
    echo "Verifique se o docker-compose está rodando ou se o nome do contêiner está correto."
    exit 1
fi

# Verifica se o contêiner está rodando
if ! docker ps --format "table {{.Names}}" | grep -q "^$CONTAINER_NAME$"; then
    echo "❌ Erro: Contêiner '$CONTAINER_NAME' não está rodando."
    echo "Inicie o contêiner com 'docker-compose up -d' ou 'docker start $CONTAINER_NAME'"
    exit 1
fi

# Espera até que o contêiner esteja "healthy" (saudável) ou atinja o tempo limite
echo "⏳ Aguardando o contêiner ficar saudável..."
for i in $(seq 1 12); do # Tenta por até 12 * 5 segundos = 60 segundos
    CONTAINER_HEALTH=$(docker inspect --format='{{.State.Health.Status}}' "$CONTAINER_NAME" 2>/dev/null || echo "not_found")
    
    case "$CONTAINER_HEALTH" in
        "healthy")
            echo "✅ Contêiner '$CONTAINER_NAME' está saudável."
            break
            ;;
        "starting")
            echo "⏳ Contêiner '$CONTAINER_NAME' está iniciando. Aguardando ($i/12)..."
            ;;
        "unhealthy")
            echo "⚠️  Contêiner '$CONTAINER_NAME' está com problemas de saúde."
            echo "Verifique os logs com 'docker logs $CONTAINER_NAME'"
            ;;
        "not_found")
            echo "❌ Erro: Contêiner '$CONTAINER_NAME' não encontrado. Verifique se ele está rodando com 'docker ps'."
            exit 1
            ;;
        *)
            echo "⚠️  Contêiner '$CONTAINER_NAME' em estado inesperado: '$CONTAINER_HEALTH'."
            echo "Tentando prosseguir, mas pode haver falhas."
            break
            ;;
    esac
    
    if [ $i -eq 12 ]; then
        echo "⚠️  Aviso: O contêiner '$CONTAINER_NAME' não atingiu o estado 'healthy' em 60 segundos."
        echo "Conexões podem falhar, mas tentaremos prosseguir."
    fi
    
    sleep 5
done

# --- Preparação e Execução no Contêiner ---

echo "🚀 Copiando arquivos de migração para o contêiner '$CONTAINER_NAME'..."

# Caminho temporário dentro do contêiner para os arquivos de migração e o script
CONTAINER_TEMP_MIGRATIONS_PATH="/tmp/migration_setup"

# Cria o diretório temporário no contêiner
docker exec "$CONTAINER_NAME" mkdir -p "$CONTAINER_TEMP_MIGRATIONS_PATH"

# Copia os arquivos e pastas do host para dentro do contêiner
echo "📁 Copiando arquivos..."
docker cp "$HOST_ENV_FILE" "$CONTAINER_NAME":"$CONTAINER_TEMP_MIGRATIONS_PATH/.env"
docker cp "$HOST_INTERNAL_RUN_SCRIPT" "$CONTAINER_NAME":"$CONTAINER_TEMP_MIGRATIONS_PATH/run_migrations_internal.sh"
docker cp "$HOST_MIGRATIONS_DIR" "$CONTAINER_NAME":"$CONTAINER_TEMP_MIGRATIONS_PATH/migrations"

echo "✅ Arquivos copiados com sucesso para '$CONTAINER_TEMP_MIGRATIONS_PATH' no contêiner."

echo "⚙️  Executando o script de migração DENTRO do contêiner..."

# Executa o script de migração dentro do contêiner
# O script interno (run_migrations_internal.sh) será responsável por carregar o .env e usar o sqlcmd
docker exec "$CONTAINER_NAME" bash -c "chmod +x $CONTAINER_TEMP_MIGRATIONS_PATH/run_migrations_internal.sh && $CONTAINER_TEMP_MIGRATIONS_PATH/run_migrations_internal.sh"

EXECUTION_STATUS=$? # Captura o código de saída do comando docker exec

if [ $EXECUTION_STATUS -eq 0 ]; then
    echo "🎉 Migrações concluídas com sucesso no SQL Server do Docker!"
else
    echo "❌ Erro ao executar migrações no contêiner. Verifique os logs acima para detalhes."
fi

# Opcional: Limpeza dos arquivos temporários no contêiner após a execução
echo "🧹 Limpando arquivos temporários no contêiner..."
docker exec "$CONTAINER_NAME" rm -rf "$CONTAINER_TEMP_MIGRATIONS_PATH"
echo "✅ Limpeza concluída."

exit $EXECUTION_STATUS