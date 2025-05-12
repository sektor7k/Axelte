CREATE DATABASE IF NOT EXISTS axelte;
USE axelte;

CREATE TABLE users (
    id CHAR(36) PRIMARY KEY,                           -- UUID kullanımı
    username VARCHAR(100) NOT NULL UNIQUE,             -- kullanıcı adı
    email VARCHAR(255) NOT NULL UNIQUE,                -- e-posta adresi (benzersiz)
    password VARCHAR(255) NOT NULL,                    -- bcrypt ile hashlenmiş şifre
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,    -- kayıt tarihi
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);