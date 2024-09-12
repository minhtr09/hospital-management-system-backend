-- Insert initial roles
INSERT INTO role (role_name, role_type, description, permission) VALUES
('Doctor', 'doctor', 'Medical practitioner responsible for patient care', 'Patient Data Access'),
('Nurse', 'nurse', 'Healthcare professional assisting in patient care', 'Patient Data Access'),
('Administrator', 'admin', 'Manages hospital operations and staff', 'Administrative Access'),
('Pharmacist', 'staff', 'Manages medication dispensing and inventory', 'Pharmacy Access'),
('Patient', 'patient', 'Receives medical care and treatment', 'Read Only');