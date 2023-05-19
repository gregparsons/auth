# https://help.dropbox.com/sync/ignored-files
# ignore
# xattr -w com.dropbox.ignored 1 frontend/target
# xattr -w com.dropbox.ignored 1 backend/target
xattr -w com.dropbox.ignored 1 ./target

#unignore
# xattr -d com.dropbox.ignored /Users/yourname/Dropbox\ \(Personal\)/YourFileName.pdf
