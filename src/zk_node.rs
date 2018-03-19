use std::fmt;

use zookeeper::{Acl, CreateMode, ZooKeeper, ZkResult, Stat};

pub struct ZkNode<'a> {
  owner: &'a ZooKeeper,
  path: String,
  data: Vec<u8>,
  acl: Vec<Acl>,
  stat: Stat
}

impl<'a> fmt::Debug for ZkNode<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "ZkNode {{ path: {}, data: {:?}, acl: {:?}, stat: {:?} }}", self.path, self.data, self.acl, self.stat)
  }
}

impl<'a> ZkNode<'a> {
  pub fn read_node(owner: &'a ZooKeeper, path: &str) -> ZkNode<'a> {
    info!("read_node(zk, {})", path);
    let (data, stat) = owner.get_data(path, false).unwrap();

    ZkNode {
      owner: owner,
      path: path.to_owned(),
      data: data,
      acl: owner.get_acl(path).unwrap().0,
      stat: stat
    }
  }

  fn create(&self, target: &'a ZooKeeper) -> ZkResult<String> {
    self.create_unless_exists(target)
      .and_then(|_| {
        target.set_acl(&self.path, self.acl.clone(), None)
      })
      .and_then(|_| {
        target.set_data(&self.path, self.data.clone(), None)
      })
      .map(|_| "done".to_owned())
  }

  fn create_unless_exists(&self, target: &'a ZooKeeper) -> ZkResult<String> {
    if target.exists(&self.path, false)?.is_none() {
        let mode = if self.stat.is_ephemeral() {
          CreateMode::Ephemeral
        } else {
          CreateMode::Persistent
        };
        target.create(&self.path, self.data.clone(), self.acl.clone(), mode)
    } else {
      Ok("present".to_owned())
    }
  }

  pub fn sync(&self, target: &'a ZooKeeper) -> ZkResult<String> {
    self.create(target).and_then(|ans| {
      if self.stat.num_children > 0 {
        self.owner.get_children(&self.path, false).unwrap()
          .iter()
          .fold(Ok("zero".to_owned()), |res, child_path| res.and_then(|_| {
            ZkNode::read_node(self.owner, &format!("{}/{}", self.path, child_path)).sync(target)
          }))
      } else {
        Ok(ans)
      }
    })
  }
}
