#!/bin/bash

# Este script é projetado para ser executado EXCLUSIVAMENTE DENTRO do contêiner Docker do SQL Server.
# Ele espera que as variáveis de ambiente e os arquivos de migração estejam presentes internamente.

set -e  # Para o script se qualquer comando falhar

echo "🚀 Iniciando processo de migração dentro do contêiner..."

# Caminho para o arquivo .env DENTRO do contêiner (onde foi copiado pelo script externo)
ENV_FILE="/tmp/migration_setup/.env"

# Verifica se o arquivo .env existe dentro do contêiner
if [ ! -f "$ENV_FILE" ]; then
    echo "❌ Erro: Arquivo .env não encontrado dentro do contêiner em '$ENV_FILE'."
    exit 1
fi

# Exporta as variáveis do .env para o ambiente do script dentro do contêiner
echo "📋 Carregando variáveis de ambiente de '$ENV_FILE'..."
set -a  # Exporta todas as variáveis definidas
source "$ENV_FILE"
set +a  # Para de exportar automaticamente

# --- Configurações de Conexão SQL Server (A partir de DENTRO do Contêiner) ---
# Quando o script roda DENTRO do contêiner SQL Server, ele se conecta a si mesmo.
# Portanto, o host é 'localhost' ou '127.0.0.1' e a porta é a porta INTERNA padrão do SQL Server (1433).
SERVER="localhost,1433"
USER="${DB_USER:-sa}"
PASSWORD="${DB_PASSWORD}"
DATABASE="${DB_NAME:-master}"

# Verificação de variáveis de ambiente essenciais carregadas
if [ -z "$PASSWORD" ]; then
    echo "❌ Erro: Variável de ambiente DB_PASSWORD não definida APÓS carregar o .env dentro do contêiner."
    echo "Verifique o conteúdo do seu .env."
    exit 1
fi

echo "🔧 Configuração de conexão:"
echo "   Server: $SERVER"
echo "   User: $USER"
echo "   Database: $DATABASE"

# Pasta onde os arquivos .sql foram copiados DENTRO do contêiner
MIGRATIONS_DIR="/tmp/migration_setup/migrations"

# Verifica se o diretório de migrações existe dentro do contêiner
if [ ! -d "$MIGRATIONS_DIR" ]; then
    echo "❌ Erro: Diretório de migrações '$MIGRATIONS_DIR' não encontrado dentro do contêiner."
    exit 1
fi

# Verifica se o sqlcmd está instalado dentro do contêiner
if ! command -v sqlcmd &> /dev/null; then
    echo "❌ Erro: 'sqlcmd' não está instalado ou não está no PATH DENTRO do contêiner."
    echo "Certifique-se de que a imagem Docker 'rust-cast-sql-sever' inclui o sqlcmd (geralmente 'mssql-tools')."
    exit 1
fi

echo "🔌 Conectando ao SQL Server em '$SERVER', Banco: '$DATABASE', Usuário: '$USER'..."

# Testa a conexão primeiro
echo "🧪 Testando conexão com o banco..."
if ! sqlcmd -S "$SERVER" -U "$USER" -P "$PASSWORD" -d "$DATABASE" -Q "SELECT 1" -b; then
    echo "❌ Erro: Não foi possível conectar ao SQL Server."
    echo "Verifique se:"
    echo "   - O SQL Server está rodando"
    echo "   - As credenciais estão corretas"
    echo "   - O banco '$DATABASE' existe"
    exit 1
fi

echo "✅ Conexão estabelecida com sucesso!"

# Lista todos os arquivos de migração
echo "📁 Arquivos de migração encontrados:"
ls -la "$MIGRATIONS_DIR"/*.sql 2>/dev/null || {
    echo "❌ Nenhum arquivo .sql encontrado em '$MIGRATIONS_DIR'"
    exit 1
}

# Executa os arquivos de migração ordenadamente
echo "🚀 Executando migrações..."
for FILE in $(ls "$MIGRATIONS_DIR"/*.sql | sort); do
    echo "-> Executando migração: $(basename "$FILE")"
    
    # Conecta ao SQL Server usando as credenciais e o banco de dados
    if sqlcmd -S "$SERVER" -U "$USER" -P "$PASSWORD" -d "$DATABASE" -i "$FILE" -b; then
        echo "✅ Migração $(basename "$FILE") aplicada com sucesso."
    else
        echo "❌ Erro ao executar $(basename "$FILE") no banco de dados."
        echo "Verifique a saída acima para detalhes."
        exit 1
    fi
done

echo "🎉 Todas as migrações foram aplicadas com sucesso dentro do contêiner!"