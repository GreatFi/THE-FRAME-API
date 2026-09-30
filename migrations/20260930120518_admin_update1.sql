-- Add migration script here
UPDATE users
SET role = 'admin'
WHERE email = 'hero2@gmail.com';
UPDATE users
SET role = 'customer'
WHERE email = 'greatemmanuel923@gmail.com';
