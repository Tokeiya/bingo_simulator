CREATE INDEX ON million_sample USING btree (play_id);
CREATE INDEX ON million_sample USING btree (round);
CREATE INDEX ON million_sample USING btree (hit_count);
CREATE INDEX ON million_sample USING btree (cards);
VACUUM ANALYZE million_sample;