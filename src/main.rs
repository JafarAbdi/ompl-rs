// Use all the autocxx types which might be handy.
use autocxx::prelude::*;

// Fails because autocxx doesn't support arrays yet. https://github.com/google/autocxx/issues/266
// And std::function https://github.com/google/autocxx/issues/1279
//
// Example for exporting a package
// https://github.com/google/autocxx/blob/main/examples/llvm/src/lib.rs.

include_cpp! {
    #include "ompl/base/Cost.h"
    #include "ompl/base/Planner.h"
    #include "ompl/base/PlannerData.h"
    #include "ompl/base/ProblemDefinition.h"
    #include "ompl/base/ValidStateSampler.h"
    #include "ompl/base/SpaceInformation.h"
    // #include "ompl/geometric/SimpleSetup.h"
    safety!(unsafe)
    generate!("ompl::base::Cost")
    // generate!("ompl::base::Planner")
    // generate!("ompl::base::PlannerData")
    // generate!("ompl::base::SpaceInformation")
    // generate!("ompl::base::ProblemDefinition")
    // generate!("ompl::geometric::SimpleSetup")
}

fn main() {
    // let c = ffi::ompl::base::Cost::new(10.0).within_unique_ptr();
    // moveit! { let c = ffi::ompl::base::Cost::new(2.0) };
    // let c = ffi::ompl::base::Cost::new(1.0).within_box();
    let c = Box::emplace(ffi::ompl::base::Cost::new(3.0));
    println!("Cost is {}", c.value());
}
