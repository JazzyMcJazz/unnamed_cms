BEGIN TRANSACTION;

CREATE system_resource CONTENT {
    "name": $name,
    "display_name": $display_name,
    "description": $description,
    "resource_type": $type,
};

DEFINE TABLE {$name} SCHEMAFULL;

COMMIT TRANSACTION;