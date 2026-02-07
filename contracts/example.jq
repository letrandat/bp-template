def die($msg): error($msg);

if .ok != true then die("expected .ok == true") else . end
| if .version != 1 then die("expected .version == 1") else . end
