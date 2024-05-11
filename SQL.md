## The list of planned to implement SQL-features

Mostly, derived from SQLite Database System Design and Implementation Book

### Data definition language, DDL:
- Creation of table, index, view;
- Deletion of table, index, view;
- ALTER TABLE (rename table and add column);
- UNIQUE, NOT NULL constraints;
- Foreign key constraint;
- Autoincrement

### Data manipulation language, DML:
- INSERT, DELETE, UPDATE, and SELECT;
- Subqueries including correlated subqueries;
- group by, order by, offset-limit;
- INNER JOIN, LEFT OUTER JOIN, NATURAL JOIN;
- UNION, UNION ALL, INTERSECT, EXCEPT;

### Transactional commands:
- BEGIN;
- COMMIT;
- ROLLBACK;

### ByteSwapDB commands:
- reindex;
- attach, detach;
- explain;
