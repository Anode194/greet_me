# Greet_Me

Simple greeting script that gathers my todos from joplin and prints out the urgent ones and allows me to view them all quickly and easily.

To set up the random quote, sign up for an account on [rapidApi](https://rapidapi.com/andruxnet/api/random-famous-quotes). you'll have 50 free requests a month if you go over that then they will start to charge you. (a penny per request I believe). Once signed up go the the get endpoint and go over to the code snipped section in the lower right hand side of the screen, click the drop down and select shell and then curl. then copy the snippet into your command line with a "> ~/.config/greet_me/quotes.json" at the end. replace the count=10 with a count=1. and then run it. copy that whole command and enter it into your crontab. type crontab -e and put 0 0 * * * your_command_here) and it will run the call and you'll have a new quote every day at midnight!
