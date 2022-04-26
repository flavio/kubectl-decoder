> ⚠️ ⚠️ **WARNING:** this plugin is an experimental POC meant to demonstrate
> how kubectl plugins can be written using WebAssembly and WASI.

This is a kubectl plugin that decodes the data kept inside of a Kubernetes
Secret. When a x509 certificates is found, its inner details are printed.

This plugin is written using WebAssembly and WASI, it can be used via the
[krew-wasm](https://github.com/flavio/krew-wasm) plugin manager.

## Prerequisites

* A working kubeconfig file
* A generic version of `kubectl`
* Latest version of `krew-wasm` installed

## Installation

Simply perform the following command:

```console
krew-wasm pull ghcr.io/flavio/krew-wasm-plugins/decoder:latest
```

As reported by `krew-wasm pull`, make sure to add `$HOME/.krew-wasm/bin` to your
`$PATH` so that `kubectl` can find the `kubectl-decoder` plugin.

## Usage

Like any other regular kubectl plugin, just invoke:

```console
kubectl decoder --help
```

### Secrets

The plugin base64-decodes the data stored inside of a Secret and print it
to the standard output.

Let's create a simple secret:

```
$ kubectl create secret generic db-user-pass \
  --from-literal=username=devuser \
  --from-literal=password='secret!'
```

Now, let's decode it:

```
$ kubectl decoder secret db-user-pass
┌──────────────┬──────────────┐
│ Name:        │ db-user-pass │
├──────────────┼──────────────┤
│ Namespace:   │ default      │
├──────────────┼──────────────┤
│ Labels:      │ <none>       │
├──────────────┼──────────────┤
│ Annotations: │ <none>       │
├──────────────┼──────────────┤
│ Type:        │ Opaque       │
└──────────────┴──────────────┘

Data:

password:
secret!

username:
devuser
```

The plugin prints detailed information about the x509 certificates that are
found inside of the secret:

```
$ kubectl decoder secret  k3s-serving -n kube-system
┌──────────────┬──────────────────────────────────────────────────────────────────────────────────────────────────┐
│ Name:        │ k3s-serving                                                                                      │
├──────────────┼──────────────────────────────────────────────────────────────────────────────────────────────────┤
│ Namespace:   │ kube-system                                                                                      │
├──────────────┼──────────────────────────────────────────────────────────────────────────────────────────────────┤
│ Labels:      │ <none>                                                                                           │
├──────────────┼──────────────────────────────────────────────────────────────────────────────────────────────────┤
│ Annotations: │ listener.cattle.io/cn-10.43.0.1: 10.43.0.1                                                       │
│              │ listener.cattle.io/cn-127.0.0.1: 127.0.0.1                                                       │
│              │ listener.cattle.io/cn-192.168.50.2: 192.168.50.2                                                 │
│              │ listener.cattle.io/cn-192.168.50.3: 192.168.50.3                                                 │
│              │ listener.cattle.io/cn-192.168.50.4: 192.168.50.4                                                 │
│              │ listener.cattle.io/cn-jolly.svc.lan: jolly.svc.lan                                               │
│              │ listener.cattle.io/cn-kube.svc.lan: kube.svc.lan                                                 │
│              │ listener.cattle.io/cn-kubernetes: kubernetes                                                     │
│              │ listener.cattle.io/cn-kubernetes.default: kubernetes.default                                     │
│              │ listener.cattle.io/cn-kubernetes.default.svc.cluster.local: kubernetes.default.svc.cluster.local │
│              │ listener.cattle.io/cn-localhost: localhost                                                       │
│              │ listener.cattle.io/fingerprint: SHA1=B304C2A51C2EA7A93817E5BEA9116C666C9E14E0                    │
├──────────────┼──────────────────────────────────────────────────────────────────────────────────────────────────┤
│ Type:        │ kubernetes.io/tls                                                                                │
└──────────────┴──────────────────────────────────────────────────────────────────────────────────────────────────┘

Data:

tls.crt:
  Version: V3
  Serial: 52:34:07:33:b2:7f:e8:3e
  Subject: O=k3s, CN=k3s
  Issuer: CN=k3s-server-ca@1620311249
  Validity:
    NotBefore: Thu, 06 May 2021 14:27:29 +0000
    NotAfter:  Sun, 05 Feb 2023 18:08:25 +0000
    is_valid:  true
  Subject Public Key Info:
    Public Key Algorithm:
      Oid: id-ecPublicKey
      Parameter: <PRESENT> prime256v1
    EC Public Key: (256 bit)
        04:66:4a:9f:d8:70:93:ad:ce:3f:d9:fe:75:36:c0:5b:
        c1:53:cd:ca:c4:cd:b6:23:60:d7:11:fd:b7:4a:04:de:
        87:d3:0a:e5:a1:51:6c:ab:31:b6:55:96:78:72:26:1c:
        e4:df:11:52:91:99:eb:1c:10:70:b0:2f:03:53:25:57:
        d2:
  Signature Algorithm:
    Oid: ecdsa-with-SHA256
    Parameter: <ABSENT>
      30:45:02:21:00:cc:09:ea:30:90:15:02:ac:0b:f9:19:
      5e:13:23:a1:77:4c:b6:13:85:67:e6:3a:5a:5d:ea:51:
      4c:4c:5f:68:06:02:20:3a:84:f6:d0:de:8d:f2:b9:7a:
      a6:ac:9c:c3:82:9b:25:68:2e:ae:d3:4e:63:a3:7d:73:
      ff:de:d2:da:d8:e8:0b:
  Extensions:
    [crit:true l:4] keyUsage: 
      X509v3 Key Usage: Digital Signature, Key Encipherment
    [crit:false l:12] extendedKeyUsage: 
      ExtendedKeyUsage(ExtendedKeyUsage { any: false, server_auth: true, client_auth: false, code_signing: false, email_protection: false, time_stamping: false, ocsp_signing: false, other: [] })
    [crit:false l:24] authorityKeyIdentifier: 
      X509v3 Authority Key Identifier
        Key Identifier: 36:b8:41:75:9d:bb:b8:22:09:86:da:c1:b6:ac:8f:c6:33:4b:8b:c5
    [crit:false l:143] subjectAltName: 
      X509v3 SAN: DNS:jolly.svc.lan
      X509v3 SAN: DNS:kube.svc.lan
      X509v3 SAN: DNS:kubernetes
      X509v3 SAN: DNS:kubernetes.default
      X509v3 SAN: DNS:kubernetes.default.svc.cluster.local
      X509v3 SAN: DNS:localhost
      X509v3 SAN: IP Address:10.43.0.1
      X509v3 SAN: IP Address:127.0.0.1
      X509v3 SAN: IP Address:192.168.50.2
      X509v3 SAN: IP Address:192.168.50.3
      X509v3 SAN: IP Address:192.168.50.4

Structure validation status: Unknown (feature 'validate' not enabled)
```
