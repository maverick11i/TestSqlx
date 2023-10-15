DO $$
DECLARE
    r RECORD;
BEGIN
    -- テーブル一覧を取得
    FOR r IN (SELECT tablename FROM pg_tables WHERE schemaname = 'public')
    LOOP
        -- テーブルを削除
        EXECUTE 'DROP TABLE IF EXISTS ' || r.tablename || ' CASCADE';
    END LOOP;
END $$;
