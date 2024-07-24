# Clocky

Clocky is a small CLI utility that allows for easy time tracking. Clocky
only tracks a single activity, e.g work, and answers two questions:
1. How much time did I work today?
2. How much time did I work this week?

A session is started at with `clocky start` and ended with `clocky end`.
A new session can't be started when a session is still running.
When you forgot to end a session, you can provide an end date with
`clocky end -d 2024-07-25T10:53:00+12:00`.

You can view your time logged for the day with `clocky today` and 
the time logged for the week with `clocky week`.
