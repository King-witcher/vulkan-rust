for f in shaders/*.slang;
do
  echo "Compiling shaders/$f to shaders/$(basename "$f").spv"
  slangc "$f" -target spirv -profile spirv_1_4 -emit-spirv-directly -fvk-use-entrypoint-name -entry vertMain -entry fragMain -o "shaders/$(basename "$f").spv"
done
