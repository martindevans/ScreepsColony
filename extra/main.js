console.log("Loading...");
try
{
    const instance = require("screeps_colony");
    instance.initialize_instance();
    module.exports.loop = instance.main;
}
catch (err)
{
    console.log("Loading Error!");
    console.log(err);
}
console.log("...Loaded");
