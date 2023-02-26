files=$(find . -iname "*.rs" -exec ls $1 {} \; | grep level)

input='        let config = config();
'

for f in ${files[@]}; do
    # level=$(cat ${f} |grep level |cut -f2 -d '"')
    # sed -i "s|\"\./{}\.txt\",|\"\./src/quizzes/{}/{}\.txt\", for_export().level, |" ${f};
    
    # n=$(cat ${f} | wc -l )
    # start=$(echo "$n-4" |bc)
    #end=$(echo "$n-5" |bc)    
    #echo "$start $end"
    #sed -i "$start,$end d" $f
    sed -i "11,17 d" $f
    #sed -i "14 d" $f

    # sed -i "${start}s|^|${input//$'\n'/\\$'\n'}|" ${f}

    # sed -i 's|input_from_file()|read_from_input_file(\&config.level, \&config.name)|' ${f}

done