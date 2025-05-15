CREATE DATABASE IF NOT EXISTS axelte;
USE axelte;

CREATE TABLE users (
    id CHAR(36) PRIMARY KEY,                          
    username VARCHAR(100) NOT NULL UNIQUE,            
    email VARCHAR(255) NOT NULL UNIQUE,                
    password VARCHAR(255) NOT NULL,   
    avatar VARCHAR(255) DEFAULT 'https://avatars.githubusercontent.com/u/124599?v=4',
    role VARCHAR(255) DEFAULT 'user',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,    
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);