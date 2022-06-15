select * from items
where priority between 256 / 3 and 256 / 3 * 2
order by random()
limit 5
