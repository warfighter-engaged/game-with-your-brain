REM call "C:\Program Files (x86)\Microsoft Visual Studio\2019\Community\VC\Auxiliary\Build\vcvars64"
cd build
cmake ..
devenv /rebuild Debug calibration.sln
cd Debug
".\calibration.exe"
cd ..\..