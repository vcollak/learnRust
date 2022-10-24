/* hosting must be made public in order the ancestors like the root library
//to access it. We're not making te serving accessible from the root
library
*/

//define sub-modules within this module
//due to "pub" keyword, hosting is accessible from this module's parent
//servicing is not
pub mod hosting;
mod serving;
