total_lines=0

for fl in $(find tree_walk_interpreter/lox/src/ -type f); do
	wc -l $fl
	lines_in_file=$(wc -l $fl | awk '{ print $1 }')
	total_lines=$((total_lines + $lines_in_file))
done

for fl in $(find bytecode_virtual_machine/lox/src/ -type f); do
	wc -l $fl
	lines_in_file=$(wc -l $fl | awk '{ print $1 }')
	total_lines=$((total_lines + $lines_in_file))
done

echo "Total lines of code: $total_lines"
