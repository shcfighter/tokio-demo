#### 创建 Topic
create_topic --topic test --replication-factor 1 --partitions 1

#### 列出所有 Topic
list

#### 查看 Topic
describe --topic test

#### 增加 Topic 的 partition 数
alter --topic test --partitions 5

#### 查看 topic 指定分区 offset 的最大值或最小值
get_offset --topic test --partitions 0

#### 删除 Topic
delete_topic --topic test

#### 生产消息
produce --topic test

#### 消费消息
##### 从头开始
consume --topic test --from-beginning 0
##### 从尾部开始
consume --topic test --offset latest --partitions 0
##### 指定 Group
consume --topic test --group test_group --from-beginning 0

#### 删除 Group
delete_group --group test_group