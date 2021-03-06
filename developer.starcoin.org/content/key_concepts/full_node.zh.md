---
title: 全节点
weight: 6
---

Starcoin区块链的客户端创建交易并提交给一个全节点。然后全节点根据一定的规则决定交易的执行顺序。一个全节点包含以下逻辑组件：

<!--more-->

**交易池**

- 交易池是一个缓存区，用来存放 "等待"执行的交易
- 当一个新的交易被添加到一个节点的交易池，这个节点的交易会给其他节点同步这个交易

**共识**

- 共识组件负责对区块进行排序，并通过共识协议最终与网络中的其他所有节点达成一致

**链**

- 链维护系统的内部状态，为其他组件的正常运行提供上下文

**执行器**

- 执行器使用虚拟机（VM）来执行交易

**虚拟机(VM)**

- 交易池使用VM验证交易
- VM用于运行交易中包含的程序并计算出结果

**挖矿**

- 通过一定的算法计算出符合一定规则的哈希

**存储**

- 存储组件用于保存交易、区块和状态等数据
