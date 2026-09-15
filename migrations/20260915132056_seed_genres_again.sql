-- Add migration script here
INSERT INTO genres (name, category) VALUES
  ('Non-Fiction', 'books'),
  ('Biography', 'books'),
  ('Self-Help', 'books'),
  ('Sci-Fi', 'books'),
  ('History', 'books'),

  ('Action', 'film'),
  ('Drama', 'film'),
  ('Sci-Fi', 'film'),
  ('Thriller', 'film'),
  ('Comedy', 'film'),
  ('Horror', 'film'),
  ('Documentary', 'film'),

  ('Shonen', 'manga'),
  ('Seinen', 'manga'),
  ('Shojo', 'manga'),
  ('Isekai', 'manga'),
  ('Action', 'manga'),
  ('Romance', 'manga'),
  ('Horror', 'manga');