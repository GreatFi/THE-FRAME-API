-- Add migration script here
UPDATE users
SET role = 'admin'
WHERE email = 'greatemmanuel923@gmail.com';