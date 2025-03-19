// Each entry will be its own file so only one password can be
// displayed at a time.
// As a result the name of each entry must fit into one Block512
// The user name and password will be used to encrypt the names
// but the actual data in each entry can have its own password
// file structure will end up being:
// ./files/<USERHASH>/<ENCRYPTEDENTRYNAME>
// Whenever and entry is updated a backup of the old entry should be still in
// the same file and accessible just incase

// File layout for an entry:
// <timestamp>\n<SMsg>(\n\n<timestamp>\n<SMsg>)?
// Every entry will consist of a timestamp followed by the message
// then an empty line will denote a backup message

// checklist:
// [] get user info
// [] have a way to display all entry names
// [] have a way to display an entry and clear the display after
// [] add a new entry
// [] add a new entry with a random password
// [] edit an entry
// [] entry backup
// [] viewable backups
// [] restore backup
// [] view all entry names
// [] remove an entry


fn main() {
}
