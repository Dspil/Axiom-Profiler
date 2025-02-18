use std::str::Split;

use super::LogParser;

/// Original Z3 log parser. Works with Z3 v.4.12.1, should work with other versions
/// as long as the log format is the same for the important line cases.
/// Compare with the log files in the `logs/` folder to see if this is the case.
/// 
/// Tries to get around borrowing issues with indirection
/// (saving ID strings to avoid managing actual references/pointers between terms/other items) and map lookups.
pub mod z3parser;
pub mod dump;

/// WORK IN PROGRESS, currently not functional.
/// 
/// A Z3 log parser using `Rc` and `RefCell` (interior mutability)
/// so that objects can be 'held' by any number of other objects without creating ownership/borrowing problems.
// pub mod z3parser_rc;

impl<T: Z3LogParser + Default> LogParser for T {
    fn process_line(&mut self, line: &str, line_no: usize) -> bool {
        let mut split = line.split(' ');
        let parse = match split.next().unwrap() {
            // match the line case
            "[tool-version]" => {
                self.version_info(split)
            }
            "[mk-quant]" | "[mk-lambda]" => {
                self.mk_quant(split)
            }
            "[mk-var]" => {
                self.mk_var(split)
            }
            "[mk-proof]" | "[mk-app]" => {
                self.mk_proof_app(split)
            }
            "[attach-meaning]" => {
                self.attach_meaning(split)
            }
            "[attach-var-names]" => {
                self.attach_vars(split, &line)
            }
            "[attach-enode]" => {
                self.attach_enode(split)
            }
            "[eq-expl]" => {
                self.eq_expl(split)
            }
            "[new-match]" => {
                self.new_match(split, line_no)
            }
            "[inst-discovered]" => {
                self.inst_discovered(split, line_no, &line)
            }
            "[instance]" => {
                self.instance(split, line_no)
            }
            "[end-of-instance]" => {
                self.end_of_instance();
                Some(())
            }
            "[decide-and-or]" => {
                self.decide_and_or(split)
            }
            "[decide]" => {
                self.decide(split)
            }
            "[assign]" => {
                self.assign(split)
            }
            "[push]" => {
                self.push(split)
            }
            "[pop]" => {
                self.pop(split)
            }
            "[begin-check]" => {
                self.begin_check(split)
            }
            "[query-done]" => {
                self.query_done(split)
            }
            "[eof]" => {
                return false;
            }
            "[resolve-process]" => {
                self.resolve_process(split)
            }
            "[resolve-lit]" => {
                self.resolve_lit(split)
            }
            "[conflict]" => {
                self.conflict(split)
            }
            _ => None,
        };
        parse.unwrap_or_else(|| println!("Error parsing line: {line}"));
        true
    }
}

pub trait Z3LogParser {
    /* Methods to handle each line case of Z3 logs.
     `l` is a line split with spaces as delimiters,
     and `l0` is the raw line (used only when )
    */
    fn version_info(&mut self, l: Split<'_, char>) -> Option<()>;
    fn mk_quant(&mut self, l: Split<'_, char>) -> Option<()>;
    fn mk_var(&mut self, l: Split<'_, char>) -> Option<()>;
    fn mk_proof_app(&mut self, l: Split<'_, char>) -> Option<()>;
    fn attach_meaning(&mut self, l: Split<'_, char>) -> Option<()>;
    fn attach_vars(&mut self, l: Split<'_, char>, l0: &str) -> Option<()>;
    fn attach_enode(&mut self, l: Split<'_, char>) -> Option<()>;
    fn eq_expl(&mut self, l: Split<'_, char>) -> Option<()>;
    fn new_match(&mut self, l: Split<'_, char>, line_no: usize) -> Option<()>;
    fn inst_discovered(&mut self, l: Split<'_, char>, line_no: usize, l0: &str) -> Option<()>;
    fn instance(&mut self, l: Split<'_, char>, line_no: usize) -> Option<()>;
    fn end_of_instance(&mut self);

    // unused in original parser
    fn decide_and_or(&mut self, _l: Split<'_, char>) -> Option<()> { Some(()) }
    fn decide(&mut self, _l: Split<'_, char>) -> Option<()> { Some(()) }
    fn assign(&mut self, _l: Split<'_, char>) -> Option<()> { Some(()) }
    fn push(&mut self, _l: Split<'_, char>) -> Option<()> { Some(()) }
    fn pop(&mut self, _l: Split<'_, char>) -> Option<()> { Some(()) }
    fn begin_check(&mut self, _l: Split<'_, char>) -> Option<()> { Some(()) }
    fn query_done(&mut self, _l: Split<'_, char>) -> Option<()> { Some(()) }
    fn resolve_process(&mut self, _l: Split<'_, char>) -> Option<()> { Some(()) }
    fn resolve_lit(&mut self, _l: Split<'_, char>) -> Option<()> { Some(()) }
    fn conflict(&mut self, _l: Split<'_, char>) -> Option<()> { Some(()) }
}

/// An identifier for a Z3 quantifier instantiation (called "fingerprint" in the original Axiom Profiler).
/// Represented as a 16-digit hexadecimal number in log files.
type Z3Fingerprint = u64;

/// Type of solver and version number
#[derive(Default)]
pub struct VersionInfo {
    solver: String,
    version: String,
}

