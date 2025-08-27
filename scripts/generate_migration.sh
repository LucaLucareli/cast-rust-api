#!/bin/bash

# Diretório de migrações
MIGRATIONS_DIR="./migrations"

# Lista migrações existentes
if [ -d "$MIGRATIONS_DIR" ]; then
  echo "Migrações existentes:"
  ls "$MIGRATIONS_DIR" | sort
else
  echo "Nenhuma migração existente."
  mkdir -p "$MIGRATIONS_DIR"
fi

echo

# Se não passar argumento, solicita o nome interativamente
if [ -z "$1" ]; then
  echo "Digite o nome da nova migração:"
  read migration_input
else
  migration_input="$1"
fi

# Remove espaços e caracteres problemáticos (só letras, números, hífen e underscore)
migration_input=$(echo "$migration_input" | tr '[:upper:]' '[:lower:]' | sed 's/[^a-z0-9_-]/-/g')

# Gera timestamp no formato Prisma (Ex: 20250724070328)
timestamp=$(date +"%Y%m%d%H%M%S")

# Define o nome completo da pasta
migration_name="${timestamp}-${migration_input}"

# Caminho da pasta de migração
migration_dir="$MIGRATIONS_DIR/$migration_name"

# Cria a pasta e arquivos
mkdir -p "$migration_dir"
touch "$migration_dir/migration.sql"

# Cria um README.md explicativo
echo "# Migration: $migration_input" > "$migration_dir/README.md"
echo "Criada em: $(date)" >> "$migration_dir/README.md"

echo
echo "✅ Migração criada em: $migration_dir"
