# pointless

Just a pointless server to test some stuff...

# Test with K3D

- Create cluster with the command

```bash
k3d cluster create ramen -p "8082:30080"
```

- Deploy the resource located in the kube folder
- Pointless should be available in localhost:8082