-- このファイルに記述されたSQLコマンドが、マイグレーション時に実行されます。

CREATE INDEX idx_username ON users(username);
CREATE INDEX idx_user_id ON dispatchers(user_id);

create index sessions_session_token_index on sessions (session_token);
-- CREATE INDEX idx_tt_status_area_id ON tow_trucks (status, area_id);
CREATE INDEX idx_tow_trucks_status ON tow_trucks(status);

-- CREATE INDEX idx_node_a_id ON edges (node_a_id);
CREATE INDEX idx_edges_covering ON edges (node_a_id, node_b_id, weight);
-- CREATE INDEX idx_completed_orders_order_id ON completed_orders (order_id);
CREATE INDEX idx_co_covering ON completed_orders (order_id, id, tow_truck_id, completed_time);

CREATE INDEX idx_locations_truck_time ON locations (tow_truck_id, timestamp DESC);