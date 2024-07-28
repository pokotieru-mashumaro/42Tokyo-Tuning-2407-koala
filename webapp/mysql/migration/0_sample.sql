-- このファイルに記述されたSQLコマンドが、マイグレーション時に実行されます。

-- users
CREATE INDEX idx_username ON users(username);

-- sessions
create index sessions_session_token_index on sessions (session_token);

-- orders
CREATE INDEX idx_node_id on orders (node_id);

-- dispatchers
CREATE INDEX idx_user_id ON dispatchers(user_id);

-- completed_orders
CREATE INDEX idx_co_covering ON completed_orders (order_id, id, tow_truck_id, completed_time);

-- edges
CREATE INDEX idx_node_a_id on edges(node_a_id);

-- locations
CREATE INDEX idx_locations_truck_time ON locations (tow_truck_id, timestamp DESC);

-- nodes
CREATE INDEX idx_nodes_area ON nodes (area_id);
