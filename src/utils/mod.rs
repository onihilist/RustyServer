use colored::Colorize;

pub mod Logs {
    use std::fmt::DebugStruct;
    use std::task::Context;
    use colored::Colorize;

    #[derive(Clone, Copy)]
    pub struct UtilsData {
        context: Option<&'static str>,
        message: &'static str,
        debug_status: Option<bool>
    }

    #[warn(non_snake_case)]
    pub fn initLog(ctx: Option<&'static str>, msg: &'static str, debug: Option<bool>) -> UtilsData {

        let mut utils: UtilsData = UtilsData {
            context: None,
            message: "",
            debug_status: None,
        };

        if ctx != None && debug == None {
            utils = UtilsData {
                context: ctx,
                message: msg,
                debug_status: Some(false)
            };
        } else if ctx != None && debug != None  {
            utils = UtilsData {
                context: ctx,
                message: msg,
                debug_status: debug
            };
        } else if ctx == None && debug != None {
            utils = UtilsData {
                context: Some(""),
                message: msg,
                debug_status: debug
            };
        } else {
            utils = UtilsData {
                context: Some(""),
                message: msg,
                debug_status: Some(false)
            };
        }
        return utils;
    }

    fn cryptoError(msg: &str) {
        println!("{} {} {}{}{}{}", "[", "CryptoChan".italic().bright_red(), "] ", "error".red(), " -> ", msg);
    }

    pub fn error(utils_data: UtilsData) {
        if utils_data.context != Option::from("") {
            cryptoError("Impossible to set a custom log status, use `clog` function.");
        } else {
            println!("{}{}{}{}", "[", "ERROR".red(), "] -> ", utils_data.message);
        }
    }

    pub fn info(utils_data: UtilsData) {
        if utils_data.context != Option::from("") {
            cryptoError("Impossible to set a custom log status, use `clog` function.");
        } else {
            println!("{}{}{}{}", "[", "INFO".cyan(), "] -> ", utils_data.message);
        }
    }

    pub fn debug(utils_data: UtilsData) {
        if utils_data.context != Option::from("") {
            cryptoError("Impossible to set a custom log status, use `clog` function.");
        } else if utils_data.debug_status == None {
            cryptoError("Enable the debug mode into UtilsData object.");
        }
        else {
            println!("{}{}{}{}", "[", "DEBUG".magenta(), "] -> ", utils_data.message);
        }
    }

    pub fn warning(utils_data: UtilsData) {
        if utils_data.context != Option::from("") {
            cryptoError("Impossible to set a custom log status, use `clog` function.");
        } else {
            println!("{}{}{}{}", "[", "WARNING".yellow(), "] -> ", utils_data.message);
        }
    }

    pub fn success(utils_data: UtilsData) {
        if utils_data.context != Option::from("") {
            cryptoError("Impossible to set a custom log status, use `clog` function.");
        } else {
            println!("{}{}{}{}", "[", "SUCCESS".green(), "] -> ", utils_data.message);
        }
    }

    pub fn clog(utils_data: UtilsData, color: Option<[u8; 3]>) {
        if utils_data.context == Option::from("") {
            cryptoError("Context is not specified.")
        } else if color != None {
            let colors = color.unwrap();
            println!("{}{}{}{}", "[", utils_data.context.unwrap().on_truecolor(colors[0], colors[1], colors[2]), "] -> ", utils_data.message);
        } else {
            println!("{}{}{}{}", "[", utils_data.context.unwrap(), "] -> ", utils_data.message);
        }
    }
}
