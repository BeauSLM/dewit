select * from items
where priority < 256 / 3
order by random()
limit 5
