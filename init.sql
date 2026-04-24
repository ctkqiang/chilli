-- Grant all privileges on chilli database to chilli user from any host
-- MySQL 8.0+ syntax: use GRANT without IDENTIFIED BY
GRANT ALL PRIVILEGES ON chilli.* TO 'chilli'@'%';
FLUSH PRIVILEGES;
