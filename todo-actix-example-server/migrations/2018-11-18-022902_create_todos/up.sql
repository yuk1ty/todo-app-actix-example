create table todos (
  id INT  AUTO_INCREMENT PRIMARY KEY,
  title VARCHAR(255) NOT NULL,
  body VARCHAR(1024),
  status BOOLEAN NOT NULL
)