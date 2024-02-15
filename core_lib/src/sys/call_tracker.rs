// struct CallTracker<'a, 'b>(&'a str, Option<CallTracker<'b>>);

use std::{rc::Rc, cell::RefCell};

pub struct CallstackTracker {
    stack: Rc<RefCell<Vec<String>>>,
}

impl CallstackTracker {
    pub fn new() -> Self {
        Self { stack: Rc::new(RefCell::new(Vec::new())) }
    }

    pub fn push(&self, function_name: String) {
        self.stack.borrow_mut().push(function_name);
    }

    pub fn pop(&self, expected_function_name: &str) {
        if let Some(actual_function_name) = self.stack.borrow_mut().pop() {
            if actual_function_name != expected_function_name {
                panic!("CallstackTracker::pop() called with unexpected function name. Expected: {}, Actual: {}", expected_function_name, actual_function_name);
            }
        } else {
            panic!("CallstackTracker::pop() called when stack is empty");
        }
    }

    pub fn len(&self) -> usize {
        return self.stack.borrow().len();
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        for (index, function_name) in self.stack.borrow().iter().enumerate() {
            if index > 0 {
                result.push_str(" -> ");
            }
            result.push_str(function_name);
        }
        result
    }
}

pub struct CallstackTrackerScope<'a> {
    tracker: &'a CallstackTracker,
    function_name: String,
}

impl<'a> CallstackTrackerScope<'a> {
    pub fn new(tracker: &'a CallstackTracker, function_name: String) -> Self {
        tracker.push(function_name.clone());
        CallstackTrackerScope {
            tracker,
            function_name: function_name,
        }
    }

    pub fn enter(tracker: &'a CallstackTracker, function_name: &'static str, member_name: &'static str) -> Self {
        let mut function_name = String::from(function_name);
        function_name.push_str("::");
        function_name.push_str(member_name);
        Self::new(tracker, function_name)
    }
}

impl<'a> Drop for CallstackTrackerScope<'a> {
    fn drop(&mut self) {
        self.tracker.pop(self.function_name.as_str());
    }
}