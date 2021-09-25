i=1
while [ $i -le 100 ]
do
    echo "buf[$i]='a';"
    i=$(($i+1))
done
