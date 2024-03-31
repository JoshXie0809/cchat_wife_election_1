# cchat_wife_election_1

## Cchat 第一屆最婆選舉

*Game1, Game2, Game3, final.txt* are raw data from PTT.


output csv file in command line

```{bash}
sqlite3  ./vote.sqlite
.mode    csv
.header  on
.output  ./vote.csv
select * from content_tbl;
```

