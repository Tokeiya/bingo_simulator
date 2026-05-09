CREATE INDEX ON all_cards USING btree (play_id);
CREATE INDEX ON all_cards USING btree (round);
CREATE INDEX ON all_cards USING btree (hit_count);
CREATE INDEX ON all_cards USING btree (cards);
VACUUM ANALYZE all_cards;