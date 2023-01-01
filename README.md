# Rusty-Notes
A cli note taking and organizing app.

## Usage
When first run, Rusty-Notes will create a notes directory in your home directory. Inside the notes dir it will create a dir for the current year, then inside that, one for the current month. The actual note will be taken in a text file named for the day of the month the note is taken on. This creates a directory tree that looks like "~/notes/<year>/<month>/<day>". The individual note will then be stamped with the time and day inside the file.
I find that this creates a good amount of organization.
You can also create a note in any file by using -o <FILE>.

The actual note taken depends on if you supplied a file name or a string. If a file name is supplied, the note will contain the absolute path to the file as well as the first 5 lines in that file. If it is a string it will simply contain that string (note that a string will need to be wrapped in quotes if it has any spaces).

-h or --help will give basic usage.
