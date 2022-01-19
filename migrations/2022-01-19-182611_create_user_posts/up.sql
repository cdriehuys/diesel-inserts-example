CREATE TABLE "user" (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    email TEXT UNIQUE NOT NULL
);

CREATE TABLE tag (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id uuid REFERENCES "user" ON DELETE CASCADE,
    title TEXT NOT NULL,

    UNIQUE (user_id, title)
);

CREATE TABLE post (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id uuid REFERENCES "user" ON DELETE CASCADE,
    tag_id uuid NOT NULL REFERENCES tag ON DELETE CASCADE,
    title TEXT NOT NULL,
    body TEXT NOT NULL DEFAULT ''
);

CREATE OR REPLACE FUNCTION get_or_create_tag(owner_id uuid,
                                             tag_name text,
                                             OUT _tag_id uuid)
AS
$$
BEGIN
    LOOP
        -- The simplest, and least computationally expensive, case is that the
        -- tag exists and we can select from it.
        SELECT tag.id
        FROM tag
        WHERE user_id = owner_id
          AND title = tag_name
        INTO _tag_id;

        -- If the select found something, we're done.
        EXIT WHEN FOUND;

        -- If the select did not find the tag, try to insert it. This could fail
        -- if the tag was just inserted so in that case, we let the loop
        -- continue and pick up the insert in the next try of the select
        -- statement.
        INSERT INTO tag AS t (user_id, title)
        VALUES (owner_id, tag_name)
        ON CONFLICT (user_id, title) DO NOTHING
        RETURNING t.id INTO _tag_id;

        -- If the insert succeeded, we're done. Otherwise try it all again.
        EXIT WHEN FOUND;
    END LOOP;
END;
$$ LANGUAGE "plpgsql";
