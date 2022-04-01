-- Your SQL goes here
DROP TABLE IF EXISTS comments;
CREATE TABLE comments(
  coid INT(10) UNSIGNED NOT NULL AUTO_INCREMENT,
  cid INT(10) UNSIGNED,
  created DATETIME NOT NULL,
  authorId INT(10) UNSIGNED NOT NULL,
  ownerId INT(100) UNSIGNED,
  text VARCHAR(255) NOT NULL,
  PRIMARY KEY (coid)
);