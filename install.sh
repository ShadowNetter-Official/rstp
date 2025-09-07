#!/bin/bash

echo
echo "rstp"
echo "by ShadowNetter"
echo
echo "cloning into repo..."
git clone https://github.com/ShadowNetter-Official/rstp
cd rstp
echo "done"
echo "installing..."
chmod +x rstp
sudo cp rstp /bin/
echo "done"
echo
echo "to uninstall do: "
echo "sudo rm /bin/rstp"
echo
