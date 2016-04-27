use countdown::Countdown;
use aliases::models::Alias;

pub struct ExecutionWorkflow {
    alias: Alias,
}

impl ExecutionWorkflow {

    pub fn new(alias: Alias) -> Self {
        ExecutionWorkflow { alias: alias }
    }

    pub fn execute(&self) {
        if self.conditional_passes() {
            if self.user_confirmation_successful() {
                self.allow_for_backout();
                self.execute_command();
            }
        } else {
            // TODO alert the user
        }
    }

    //------------- private -----------//

    fn conditional_passes(&self) -> bool {
        self.alias.conditional.execute()
    }

    fn user_confirmation_successful(&self) -> bool {
        self.alias.user_confirmation.execute()
    }

    fn allow_for_backout(&self) {
        if self.alias.delayed_backout > 0 {
            println!("Executing '{}' in {} seconds", self.alias.command(), self.alias.delayed_backout);
            println!("Press ctrl + c to cancel execution.");
            Countdown::new(self.alias.delayed_backout.clone()).start();
        }
    }

    fn execute_command(&self) {
        self.alias.execute();
    }
}