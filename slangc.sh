# !/bin/sh

# Remove existing .spv files
spvs=$(find . -name '*.spv')
for spv in $spvs; do
  rm "$spv"
done

function compile {
  local file_path="$1"

  local dir=$(dirname "$file_path")
  local filename=$(basename "$file_path")
  local basename=${filename%.*}

  slangc $file_path          \
    -target spirv            \
    -profile spirv_1_4       \
    -emit-spirv-directly     \
    -fvk-use-entrypoint-name \
    -entry vertMain          \
    -entry fragMain          \
    -o "$dir/$basename.spv"

  if [ $? -eq 0 ]; then
    echo "Compiled $file_path to $dir/$basename.spv"
  fi
}

# Compile .slang files to .spv
shaders=$(find . -name '*.slang')
for shader in $shaders; do
  compile $shader
done
