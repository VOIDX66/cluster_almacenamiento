# cluster_almacenamiento

Instalar tanto gluster server como client


```bash
sudo apt update
sudo apt install -y glusterfs-server glusterfs-client
```

iniciar los servicios 

```bash
sudo systemctl enable --now glusterd
```
