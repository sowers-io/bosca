#!/bin/bash

set -e

echo "Initializing Databases..."

psql -v ON_ERROR_STOP=1 --username bosca --dbname bosca <<-EOSQL
	CREATE DATABASE temporal owner bosca;
	CREATE DATABASE mlflow owner bosca;
	CREATE DATABASE spicedb owner bosca;
	CREATE DATABASE hydra owner bosca;
	CREATE DATABASE kratos owner bosca;
	CREATE DATABASE boscaprofiles owner bosca;
	CREATE DATABASE boscacontent owner bosca;
	CREATE DATABASE boscaworkflow owner bosca;
	CREATE DATABASE boscasecurity owner bosca;
	CREATE DATABASE boscatest owner bosca;
EOSQL

echo "...Done"
