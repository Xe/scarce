# Project Scarce

## Abstract
An automated uploading tool for Philomena boorus. This software will scrape other image boorus and then upload images to Furbooru after applying tag transformations.

## Dependencies

- [ ] https://github.com/Xe/furbooru/pull/5
- [ ] https://github.com/furbooru/philomena/pull/12
- [ ] Rescue the xe621 package from the withinbot-old repo
    - [ ] Update xe621 to the new API for e621
- [ ] Tag translation
- [ ] Command system over a magic forum thread in the meta board
- [ ] State in a SQLite database
- [ ] A PostgreSQL database connection for a backup of Derpibooru's data (for faster hash checking)
- [ ] Evaluate a higher rate limit for the Scarce bot's IP address

## Tag Translation
Various image boorus have separate tagging standards. Scarce will need to translate that into the Furbooru tagging system. Most of the booru apis offer a lot of the information we would need to translate this, but the rating system can make things a bit interesting. It is proposed to have a few tag transformations as well as manually adding a "rating review required" tag in order to stay on the safe side. If all else fails, fail to the higher rating values to be safe.

### e621 Tag Translation
For e621, translate all underscores into spaces. This should cover 99% of the tags that are required, if making things a bit verbose. Additionally, translate the rating tags as follows:

| E621 rating | Furbooru rating |
|:----------- |:--------------- |
| Safe | Safe |
| Questionable | Questionable |
| Explicit | Explicit |

### Derpibooru Tag Translation
Derpibooru is where things get even more fun. Species are usually not tagged on derpibooru. However, when the species is not tagged, this usually means that the image is of a pony. Ratings should map 1:1.

Explicitly ignore the following tags:
* nazi
* oc:aryanne
* foalcon

## Forum-Based Command System
The most natural place to control this bot is via a forum thread. A thread in the meta board will be created with the OP of the post explaining how to give the bot commands. Initially, commands should be restricted to moderators and above. As things clear up, it will be safer to open up the tag scraping support to everyone.

The following commands are proposed:
* !scarce stop - instantly stops scraping and waits for further instructions
* !scarce start - continues scraping where it left off
* !scarce daily_limit `<number>` - sets Scarce's daily limit of the number of images it can post
* !scarce add_tag `<source> <tag>` - Tells scarce to additionally monitor a given tag by name from a given source. Valid sources are e621 and derpibooru (can be shortened as derpi)
* !scarce del_tag `<source> <tag>` - Tells scarce to stop monitoring a given tag from a given source
* !scarce ignore_tag `<source> <tag>` - Tells scarce to explicitly ignore a given tag, meaning any images with that tag will not be queued for upload
* !scarce unignore_tag `<source> <tag>` - Tells scarce to unignore a given tag, allowing uploads of it to happen again
* !scarce print_tags - Tells scarce to reply to the forum thread with a list of tags it is currently monitoring
* !scarce add_rule `<source> <from> <to>` - Tells scarce to interpret a given tag from a source as another tag on Furbooru. If the tag is "DROP", the tag will be dropped when preparing an upload.
* !scarce del_rule `<source> <from> <to>` - Tells scarce to remove a tag translation rule.
* !scarce print_rules - Tells scarce to upload a textual form of its list of rules to $SOMEWHERE and put a permalink to it in the forum thread as a hyperlink.
* !scarce status - Tells scarce to reply to the forum thread with a summary of its current status including:
    * The number of images it has posted in the last 24 hours
    * It's daily limit
    * How many tag translation rules it has
    * How many tags it is monitoring
    * How deep its posting queue is
    * How many images it has ever posted
    * A link to the source code
    * Other information that may be useful when debugging the program

## State in SQLite
Scarce should have the following SQLite tables:
* config - where scarce's volatile config lives
* tag_watchlist - where the watchlist of tags is stored
* images - where metadata about images is stored
* upload_queue - where uploads that will be sent are stored
* tag_translation_list - where the tag translation rules are stored
* stats - where statistics of the software are stored

## PostgreSQL copy of Derpibooru's data [low priority]
This will be used in order to query for images that match a given set of tags. This would be done with a local database in order to scrape these images and get them into the upload queue faster.

## Higher Rate Limit
This bot may end up running into rate limit issues on Furbooru given it needs to do at most three API calls in order to upload an image (once for sha512 searching and for original sha512 searching, once for perceptual matching, once for uploading the image). If any of these calls result in finding a match in the Furbooru database, the image is deleted from the upload queue and a "already found" statistic is incremented.
