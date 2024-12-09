@echo off

if not exist artifacts mkdir artifacts
copy /y target\release\wincron.exe artifacts\wincron-release.exe
copy /y target\debug\wincron.exe artifacts\wincron-debug.exe
