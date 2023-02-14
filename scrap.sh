files=$(find . -iname "*.rs" -exec ls $1 {} \; | grep level)

input='    let input = ;
'

for f in ${files[@]}; do
    # level=$(cat ${f} |grep level |cut -f2 -d '"')
    # sed -i "s|\"\./{}\.txt\",|\"\./src/quizzes/{}/{}\.txt\", for_export().level, |" ${f};
    
    #n=$(cat ${f} | wc -l )
    #start=$(echo "$n-7" |bc)
    #end=$(echo "$n-5" |bc)    
    #echo "$start $end"
    #sed -i "$start,$end d" $f
    #sed -i "14 d" $f

    #sed -i "7s|^|${input//$'\n'/\\$'\n'}|" ${f}

    sed -i 's|    let input = "";||' ${f}

done