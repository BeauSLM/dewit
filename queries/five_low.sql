select * from items
where priority > 256 / 3 * 2
order by random()
limit 5
