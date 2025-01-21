# Cluster Template

We offer a customizable and flexible template for setting up a Kubernetes cluster and Golem Cloud directly on bare metal or virtual machines on the cloud or on prem.

## Let's Go!
There are 4 stages outlined below for completing this project, make sure you follow the stages in order.

### Stage 1: Machine Preparation
System Requirements

> [!IMPORTANT]
> 1. The included behaviour of Talos is that all nodes are able to run workloads, **including** the controller nodes. **Worker nodes** are therefore **optional** but recommended.
> 2. It is highly recommended to have at least 3 nodes for a highly available control plane.
> 3. Are you planning to run your cluster on Proxmox or another self hosted hypervisor?
>
> Here are some important considerations for etcd:
> - **Quorum Requirement**: Etcd needs at least three master/control plane nodes to maintain quorum.
> - **Performance Characteristics**: It is highly read/write intensive and requires low IOPS/latency.
>
> **Disk Considerations**:
> - When using the same disk for all master nodes, consider the operational characteristics of etcd. Each commit to etcd generates at least three times as many read and write operations on the filesystem due to its internal mechanisms. This means your disk must be capable of handling not only this high-frequency traffic but also additional reads and writes from Golem Cloud and the Workers you plan to deploy. Therefore, it is advisable to attach multiple independent disks to your VMs to ensure optimal performance and reliability.

| Role    | Cores    | Memory        | System Disk               |
|---------|----------|---------------|---------------------------|
| Control | 4 _(6*)_ | 8GB _(24GB*)_ | 120GB _(500GB*)_ SSD/NVMe |
| Worker  | 4 _(6*)_ | 8GB _(24GB*)_ | 120GB _(500GB*)_ SSD/NVMe |
| _\* recommended_ |

1. Head over to the [Talos Linux Image Factory](https://factory.talos.dev) and follow the instructions. Be sure to only choose the **bare-minimum system extensions** as some might require additional configuration and prevent Talos from booting without it. You can always add system extensions after Talos is installed and working.
2. This will eventually lead you to download a Talos Linux ISO file (or for SBCs the RAW file). Make sure to note the **schematic ID** you will need this later on.
3. Flash the Talos ISO or RAW file to a USB drive and boot from it on your nodes.

### Stage 2: Local Workstation
1. Use `git clone` to download **the repo** to your local workstation and `cd` into `kube/oss`.
2. **Install** and **activate** [mise](https://mise.jdx.dev/) following the instructions for your workstation [here](https://mise.jdx.dev/getting-started.html).
3. Use `mise` to install the **required** CLI tools:

   📍 _If `mise` is having trouble compiling Python, try running `mise settings python.compile=0` and try these commands again_

    ```sh
    mise trust
    mise install
    mise run deps
    ```
### Stage 3: Template Configuration

> [!IMPORTANT]
> The [config.sample.yaml](./config.sample.yaml) file contains config that are **vital** to the template process.

1. Generate the `config.yaml` from the [config.sample.yaml](./config.sample.yaml) configuration file:

   📍 _If the below command fails `mise` is either not install or configured incorrectly._

    ```sh
    task init
    ```
2. Fill out the `config.yaml` configuration file using the comments in that file as a guide.

3. Template out all the configuration files:

    ```sh
    task configure
    ```
