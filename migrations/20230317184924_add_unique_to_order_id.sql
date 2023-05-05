ALTER TABLE alpaca_order drop CONSTRAINT if exists alpaca_order_id_unique;
ALTER TABLE alpaca_order ADD CONSTRAINT alpaca_order_id_unique UNIQUE (id) ;
