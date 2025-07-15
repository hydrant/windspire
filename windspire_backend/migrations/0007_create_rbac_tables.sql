-- Create roles table
CREATE TABLE roles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR UNIQUE NOT NULL,
    description VARCHAR NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Create permissions table
CREATE TABLE permissions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR UNIQUE NOT NULL,
    description VARCHAR NOT NULL,
    resource VARCHAR NOT NULL,
    action VARCHAR NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Create user_roles junction table
CREATE TABLE user_roles (
    user_id UUID NOT NULL,
    role_id UUID NOT NULL,
    assigned_at TIMESTAMPTZ DEFAULT NOW(),
    PRIMARY KEY (user_id, role_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (role_id) REFERENCES roles(id) ON DELETE CASCADE
);

-- Create role_permissions junction table
CREATE TABLE role_permissions (
    role_id UUID NOT NULL,
    permission_id UUID NOT NULL,
    granted_at TIMESTAMPTZ DEFAULT NOW(),
    PRIMARY KEY (role_id, permission_id),
    FOREIGN KEY (role_id) REFERENCES roles(id) ON DELETE CASCADE,
    FOREIGN KEY (permission_id) REFERENCES permissions(id) ON DELETE CASCADE
);

-- Create indexes for better performance
CREATE INDEX idx_user_roles_user_id ON user_roles(user_id);
CREATE INDEX idx_user_roles_role_id ON user_roles(role_id);
CREATE INDEX idx_role_permissions_role_id ON role_permissions(role_id);
CREATE INDEX idx_role_permissions_permission_id ON role_permissions(permission_id);
CREATE INDEX idx_permissions_resource_action ON permissions(resource, action);

-- Insert default roles
INSERT INTO roles (id, name, description) VALUES
('01964081-0000-0000-0000-000000000001', 'admin', 'Full system administrator access'),
('01964081-0000-0000-0000-000000000002', 'moderator', 'Content moderation and user management'),
('01964081-0000-0000-0000-000000000003', 'user', 'Basic user access');

-- Insert default permissions
INSERT INTO permissions (id, name, description, resource, action) VALUES
-- User permissions
('01964081-1000-0000-0000-000000000001', 'users:read', 'Read all users', 'users', 'read'),
('01964081-1000-0000-0000-000000000002', 'users:write', 'Create and update users', 'users', 'write'),
('01964081-1000-0000-0000-000000000003', 'users:delete', 'Delete users', 'users', 'delete'),
('01964081-1000-0000-0000-000000000004', 'users:read_own', 'Read own user profile', 'users', 'read_own'),
('01964081-1000-0000-0000-000000000005', 'users:write_own', 'Update own user profile', 'users', 'write_own'),

-- Country permissions
('01964081-2000-0000-0000-000000000001', 'countries:read', 'Read countries', 'countries', 'read'),
('01964081-2000-0000-0000-000000000002', 'countries:write', 'Create and update countries', 'countries', 'write'),
('01964081-2000-0000-0000-000000000003', 'countries:delete', 'Delete countries', 'countries', 'delete'),

-- Boat permissions
('01964081-3000-0000-0000-000000000001', 'boats:read', 'Read boats', 'boats', 'read'),
('01964081-3000-0000-0000-000000000002', 'boats:write', 'Create and update boats', 'boats', 'write'),
('01964081-3000-0000-0000-000000000003', 'boats:delete', 'Delete boats', 'boats', 'delete');

-- Assign permissions to roles
-- Admin role gets all permissions
INSERT INTO role_permissions (role_id, permission_id) 
SELECT '01964081-0000-0000-0000-000000000001', id FROM permissions;

-- Moderator role gets read/write access to content, read access to users
INSERT INTO role_permissions (role_id, permission_id) VALUES
('01964081-0000-0000-0000-000000000002', '01964081-1000-0000-0000-000000000001'), -- users:read
('01964081-0000-0000-0000-000000000002', '01964081-1000-0000-0000-000000000004'), -- users:read_own
('01964081-0000-0000-0000-000000000002', '01964081-1000-0000-0000-000000000005'), -- users:write_own
('01964081-0000-0000-0000-000000000002', '01964081-2000-0000-0000-000000000001'), -- countries:read
('01964081-0000-0000-0000-000000000002', '01964081-2000-0000-0000-000000000002'), -- countries:write
('01964081-0000-0000-0000-000000000002', '01964081-3000-0000-0000-000000000001'), -- boats:read
('01964081-0000-0000-0000-000000000002', '01964081-3000-0000-0000-000000000002'); -- boats:write

-- User role gets basic read access
INSERT INTO role_permissions (role_id, permission_id) VALUES
('01964081-0000-0000-0000-000000000003', '01964081-1000-0000-0000-000000000004'), -- users:read_own
('01964081-0000-0000-0000-000000000003', '01964081-1000-0000-0000-000000000005'), -- users:write_own
('01964081-0000-0000-0000-000000000003', '01964081-2000-0000-0000-000000000001'), -- countries:read
('01964081-0000-0000-0000-000000000003', '01964081-3000-0000-0000-000000000001'); -- boats:read

-- Assign default user role to existing users
INSERT INTO user_roles (user_id, role_id)
SELECT id, '01964081-0000-0000-0000-000000000003' FROM users;

-- Make the first user (Ove) an admin
UPDATE user_roles 
SET role_id = '01964081-0000-0000-0000-000000000001' 
WHERE user_id = '01964081-4fbf-747a-ae64-d17030fc3dcc';

-- Add comments
COMMENT ON TABLE roles IS 'System roles for RBAC';
COMMENT ON TABLE permissions IS 'System permissions for RBAC';
COMMENT ON TABLE user_roles IS 'User role assignments';
COMMENT ON TABLE role_permissions IS 'Role permission grants';