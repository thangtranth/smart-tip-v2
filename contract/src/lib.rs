use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, Balance};
use near_sdk::{json_types::U128, near_bindgen, AccountId, Promise};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct TaskId(pub u64);

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum TaskStatus {
    PENDING,
    COMPLETE,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Contribution {
    activity_point: U128,
}

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Project {
    members: UnorderedMap<AccountId, Contribution>,
    amount_to_allocate: Balance,
    total_activity_point: U128,
    // Should it only inlude outstanding tasks?
    tasks: UnorderedMap<TaskId, TaskStatus>,
}

impl Default for Project {
    fn default() -> Self {
        env::panic_str("Smart tip should be initialized before usage")
    }
}
#[near_bindgen]
impl Project {
    // Initiate the contract
    #[init]
    pub fn new(member_list: Vec<AccountId>) -> Self {
        let mut project = Self {
            members: UnorderedMap::new(b"c".to_vec()),
            amount_to_allocate: 0,
            total_activity_point: 0.into(),
            tasks: UnorderedMap::new(b"s".to_vec()),
        };

        let initial_activity_point = Contribution {
            activity_point: 0.into(),
        };

        for member in member_list {
            project.members.insert(&member, &initial_activity_point);
        }

        project
    }

    // Add task
    pub fn add_task(&mut self, id: u64) {
        self.tasks.insert(&TaskId(id), &TaskStatus::PENDING);
    }

    // Update point when a task is completed.
    pub fn complete_activitiy(&mut self, task_id: u64) {
        assert!(
            self.tasks
                .keys_as_vector()
                .to_vec()
                .contains(&TaskId(task_id)),
            "The task is not existed"
        );
        // TO DO: check if task is already completed, do not add.
        let account_id = env::predecessor_account_id();
        self.total_activity_point.0 += 1;
        self.tasks.insert(&TaskId(task_id), &TaskStatus::COMPLETE);
        let contribution = self.members.get(&account_id).unwrap_or(Contribution {
            activity_point: 0.into(),
        });

        let activity_point_update = contribution.activity_point.0 + 1;
        self.members.insert(
            &account_id,
            &Contribution {
                activity_point: (activity_point_update).into(),
            },
        );
    }

    // Send Tip to the Smarttip
    #[payable]
    pub fn tip(&mut self) -> u128 {
        let account_id = env::predecessor_account_id().to_string();
        let amount = env::attached_deposit();
        self.amount_to_allocate += amount;
        env::log_str(&format!(
            "Account {} tipped the project {}",
            account_id, amount
        ));

        amount
    }

    #[payable]
    // Pay all contributors of the project after the tips.
    pub fn pay_all_contributors(&mut self) {
        for member in self.members.keys() {
            // self.pay_tip(member);
            let tip_amount = self.allocate_tip(&member);
            Promise::new(member).transfer(tip_amount);
        }

        self.amount_to_allocate = 0;
    }

    // Send tips from smart contracts to near account.

    // pub fn pay_tip(&self, account_id: AccountId) -> Promise {
    //     let tip_amount = self.allocate_tip(&account_id);
    //     Promise::new(account_id).transfer(tip_amount)
    // }

    // Amount to allocate increase
    fn allocate_tip(&self, account_id: &AccountId) -> u128 {
        let contributor = self.members.get(account_id).unwrap();
        let allocation = self.amount_to_allocate * u128::from(contributor.activity_point)
            / u128::from(self.total_activity_point);
        println!(
            "activity_point: {} ",
            u128::from(contributor.activity_point)
        );
        println!(
            "total_activity_point: {} ",
            u128::from(self.total_activity_point)
        );
        println!(
            "amount_to_allocate: {} ",
            u128::from(self.amount_to_allocate)
        );
        println!("allocation: {} ", u128::from(allocation));
        env::log_str(&format!(
            "Contributor {} is allocated {} ",
            account_id, allocation
        ));
        allocation
    }

    // Get functions
    pub fn get_total_amount_to_allocate(&self) -> u128 {
        return self.amount_to_allocate;
    }

    pub fn get_total_activity_point(&self) -> u128 {
        return self.total_activity_point.0;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;

    #[test]
    fn test_initiate() {
        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(accounts(0))
            .attached_deposit(10)
            .build());
        let project = Project::new(vec![accounts(0), accounts(1)]);
        assert_eq!(project.members.len(), 2);
        assert_eq!(project.amount_to_allocate, 0);
    }
    #[test]
    fn test_add_task() {
        let mut project = Project::new(vec![accounts(0)]);
        project.add_task(1);
        project.add_task(2);
        assert_eq!(project.tasks.len(), 2);
    }

    #[test]
    fn test_complete_task() {
        let mut project = Project::new(vec![accounts(0)]);
        project.add_task(1);
        project.complete_activitiy(1);
        println!(
            "Task status issss: {:#?}",
            project.tasks.get(&TaskId { 0: 1 }).unwrap(),
        );
        assert_eq!(
            project.tasks.get(&TaskId { 0: 1 }).unwrap(),
            TaskStatus::COMPLETE
        );
    }

    #[test]
    #[should_panic]
    fn test_non_existed_task() {
        let mut project = Project::new(vec![accounts(0)]);
        project.add_task(1);
        project.complete_activitiy(2);
        assert_eq!(
            project.tasks.get(&TaskId { 0: 1 }).unwrap(),
            TaskStatus::COMPLETE
        );
    }

    #[test]
    fn test_tip() {
        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(accounts(1))
            .attached_deposit(10)
            .build());
        let mut project = Project::new(vec![accounts(0)]);
        project.tip();
        assert_eq!(project.amount_to_allocate, 10)
    }

    #[test]
    fn test_allocate_tip() {
        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(accounts(0))
            .build());
        let mut project = Project::new(vec![accounts(0)]);
        project.add_task(1);
        project.complete_activitiy(1);
        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(accounts(1))
            .attached_deposit(10)
            .build());
        project.tip();
        let allocated_amount = project.allocate_tip(&accounts(0));
        assert_eq!(allocated_amount, 10);
    }

    #[test]
    fn test_allocate_tip_2_accounts() {
        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(accounts(0))
            .build());
        let mut project = Project::new(vec![accounts(0)]);
        project.add_task(1);
        project.add_task(2);
        project.add_task(3);
        project.complete_activitiy(1);
        // New contributor completed 2 tasks
        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(accounts(1))
            .build());
        project.complete_activitiy(2);
        project.complete_activitiy(3);

        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(accounts(2))
            .attached_deposit(30)
            .build());
        project.tip();
        let allocated_amount_account_0 = project.allocate_tip(&accounts(0));
        assert_eq!(allocated_amount_account_0, 10);
        let allocated_amount_account_1 = project.allocate_tip(&accounts(1));
        assert_eq!(allocated_amount_account_1, 20);
    }

    #[test]
    fn test_pay_all_contributors() {
        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(accounts(0))
            .build());
        let mut project = Project::new(vec![accounts(0)]);
        project.add_task(1);
        project.complete_activitiy(1);
        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(accounts(1))
            .attached_deposit(10)
            .build());
        project.tip();
        let allocated_amount = project.allocate_tip(&accounts(0));
        assert_eq!(allocated_amount, 10);
        project.pay_all_contributors();
        assert_eq!(project.amount_to_allocate, 0);
    }
}
