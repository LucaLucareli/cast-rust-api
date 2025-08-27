USE master;
GO

IF DB_ID(N'$(DbName)') IS NULL
BEGIN
    CREATE DATABASE $(DbName);
    PRINT '***Database $(DbName) created***'
END
ELSE
    PRINT 'Database $(DbName) already exists, skipping...'
GO

IF DB_ID(N'test') IS NULL
BEGIN
    CREATE DATABASE test;
    PRINT '***Database test created***'
END
ELSE
    PRINT 'Database test already exists, skipping...'
GO

-- Perform additional setup steps below
-- ...
