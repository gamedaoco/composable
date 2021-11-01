#!/bin/sh
mkdir ~/.ssh
echo $VAULT_PASS > /vault_password_file.txt
ansible-vault decrypt \
--vault-password-file=/vault_password_file.txt \
id_rsa-encrypted \
--output=~/.ssh/id_rsa
chmod 0600 ~/.ssh/id_rsa
ansible-playbook --vault-password-file=/vault_password_file.txt \
-i ansible/hosts \
ansible/deploy.yml