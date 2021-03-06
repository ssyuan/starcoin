---
title: 去中心化治理
weight: 7
---

Token 的去中心化治理已经成为区块链必不可少的一部分。 在 Starcoin 中，可以用 Move 语言方便的实现 DAO 功能。
标准库中内置了一个 DAO 实现，Starcoin 本身也可以通过该模块对各种链上参数进行投票治理。

<!--more-->



本节主要介绍该模块的功能以及治理流程。

## DAO 功能

一个基本的 DAO 治理流程大概包括：

- 发起人发起提案。
- 用户投票。
- 提案通过以及执行。

![dao](/images/dao.jpg)

Starcoin 的 DAO 实现与以太坊的 DAO 实现一个最大的区别是：Starcoin 中，每种类型的提案都有一个单独的合约模块去控制，由该模块去实现提案的发起和提案的执行。

这是因为，以太坊中，智能合约可以通过动态分发去调用其他合约接口，因此可以做到一个合约去发起所有类别的提案，只需要在合约内部动态调用即可。但 Move 是一个函数静态分发调用的模型（这里不多叙述，感兴趣的读者可以阅读 Move 相关的文档），所有的代码调用都必须在编译时确定，做不到动态分发。因此产生了前述的区别。

提案的投票则由 DAO 模块统一负责，DAO 模块对提案做了抽象（实现上，提案是 DAO 模块的一个范型参数），用 `proposal_id` 去标识某个提案，至于提案是什么内容，它不关心，交给用户自己去判断。
投票时，用户通过 DAPP 去获取某个提案的具体内容，然后直接调用 DAO 模块的接口投票，赞成或者反对。

这样可以做到，不同提案可以实现自己的提案逻辑，但又可以共享 DAO 模块的投票功能。

标准库默认提供了以下几种提案：
- ModifyDaoConfigPorposal: 更改 DAO 投票参数的提案。
- OnChainConfigDao: 更改链上参数的提案。
- UpgradeModuleDaoProposal: 升级合约代码的提案。
- TreasuryWithdrawDaoProposal: 从国库中提款的提案。

在发行自己的 Token 时，如有类似需求，用户可以直接接入标准库中的提案，如果有其他更复杂的需求，也可以编写更多的自定义的提案。

### 用户投票

用户投票时，需要质押自己的 Token，票数和 Token 数成正比，即：一币一票。
在投票期，用户可以多次投票，也可以撤回投票，甚至可以倒戈到对方（由赞成变反对，由反对变赞成）。
投票期过后，用户可以立即提回自己质押的 Token。

### 提案通过和执行

投票期过后，如果投票率通过，并且赞成人数多余反对人数，那提案就通过了。
此时，任何人都可以发送交易将提案标识为 **待执行** 状态，放入到队列中，等待执行。
当执行公示期过后，任何人可以发送交易去执行该提案。
提案执行后，提案发起人才可以删除自己的提案，释放提案占用的链上空间。

提案的一个完整生命周期如下：

![proposal lifetime](/images/proposal_lifetime.jpg)

### 投票状态说明：

```rust

    const PENDING: u8 = 1; //等待公示时期
    const ACTIVE: u8 = 2;  //正在进行投票
    const DEFEATED: u8 = 3; //投票期过后，同意的票数小于等于反对的票数，或者同意的票数小于投票阈值，提案被拒绝 
    const AGREED: u8 = 4; //投票期过后，同意的票数大于反对的票数，提案通过
    const QUEUED: u8 = 5; //投票通过的提案被放入等待执行队列进行公示，当前公示期为 24 小时
    const EXECUTABLE: u8 = 6; //经过公示期后，进入可执行状态。任何人可以触发执行。
    const EXTRACTED: u8 = 7; //提案已经执行

```


