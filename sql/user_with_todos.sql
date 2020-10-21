select *, u.id as user_user_id, t.id as todo_id from public.users u 
left join public.todos t 
on t.user_id = u.id 
where u.id = '{}'