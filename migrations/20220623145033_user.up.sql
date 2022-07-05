-- Active: 1657341104989@@127.0.0.1@3306
-- Add up migration script here
CREATE TABLE `short_links` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `url` varchar(255) NOT NULL DEFAULT '',
  PRIMARY KEY (`id`)
);