//! Build tools and compilers.
//!
//! Provides Program implementations for make, gcc, configure, ant, and mvn.

use std::sync::Arc;

use super::common;
use crate::category::Category;
use crate::colors::SemanticColor;
use crate::program::SimpleProgram;
use crate::rule::Rule;

// =============================================================================
// MAKE
// =============================================================================

fn make_rules() -> Vec<Rule> {
    vec![
        // Error markers
        Rule::new(r"\*\*\*\s+")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"^make(\[\d+\])?:\s+\*\*\*")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\bError\s+\d+")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .build(),
        Rule::new(r"\bStop\.")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
        // File:line references
        Rule::new(r"^[\w\./\-]+:\d+:")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        // Entering/leaving directory
        Rule::new(r"make(\[\d+\])?: Entering directory")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"make(\[\d+\])?: Leaving directory")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        // Target names
        Rule::new(r"make(\[\d+\])?: Nothing to be done for")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        Rule::new(r"make(\[\d+\])?: `\S+' is up to date")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        // Recipe execution
        Rule::new(r"^\t")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        common::number_rule(),
    ]
}

pub fn make_program() -> Arc<SimpleProgram> {
    Arc::new(
        SimpleProgram::new(
            "dev.make",
            "make",
            "Make build output",
            Category::Dev,
            make_rules(),
        )
        .with_detect_patterns(vec!["make", "gmake", "cmake"]),
    )
}

// =============================================================================
// GCC / G++ (Compilers)
// =============================================================================

fn gcc_rules() -> Vec<Rule> {
    let mut rules = vec![];

    // File:line:col location
    rules.push(common::compiler_location_rule());

    // Error levels
    rules.extend([
        Rule::new(r"\berror:")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\bwarning:")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .bold()
            .build(),
        Rule::new(r"\bnote:")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\bfatal error:")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
    ]);

    // Error codes
    rules.push(
        Rule::new(r"\[-W[\w\-]+\]")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    );

    // Source code caret indicator
    rules.extend([
        Rule::new(r"^\s*\^~*$")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .build(),
        Rule::new(r"^\s*\|")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    ]);

    // Line numbers
    rules.push(
        Rule::new(r"^\s*\d+\s*\|")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    );

    // In file included from
    rules.push(
        Rule::new(r"^In file included from")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
    );

    // Function context
    rules.push(
        Rule::new(r"^In (function|member function|constructor|destructor)")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
    );

    // Required from
    rules.push(
        Rule::new(r"^\s*required from")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    );

    // Undefined reference (linker errors)
    rules.extend([
        Rule::new(r"\bundefined reference to\b")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
        Rule::new(r"\bmultiple definition of\b")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
    ]);

    // Build status
    rules.extend(common::build_status_rules());

    // Type names (common patterns)
    rules.push(
        Rule::new(r"'[^']+::[^']+'")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
    );
    rules.push(
        Rule::new(r"'[^']+'")
            .unwrap()
            .semantic(SemanticColor::String)
            .build(),
    );

    rules.push(common::number_rule());
    rules
}

pub fn gcc_program() -> Arc<SimpleProgram> {
    Arc::new(
        SimpleProgram::new(
            "dev.gcc",
            "gcc",
            "GCC/G++/Clang compiler output",
            Category::Dev,
            gcc_rules(),
        )
        .with_detect_patterns(vec!["gcc", "g++", "clang", "clang++", "cc", "c++"]),
    )
}

// =============================================================================
// CONFIGURE (Autoconf)
// =============================================================================

fn configure_rules() -> Vec<Rule> {
    vec![
        // Check results
        Rule::new(r"^checking .*\.\.\. yes$")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"^checking .*\.\.\. no$")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .build(),
        Rule::new(r"^checking .*\.\.\. ")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        // Configure status
        Rule::new(r"^configure: creating ")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"^configure: error:")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"^configure: warning:")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .bold()
            .build(),
        // Config status
        Rule::new(r"^config\.status: creating ")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"^config\.status: error:")
            .unwrap()
            .semantic(SemanticColor::Error)
            .build(),
        // Version/found messages
        Rule::new(r"found \S+ version")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"not found")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .build(),
        // File paths
        Rule::new(r"/usr/\S+")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"/opt/\S+")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        // Feature enables/disables
        Rule::new(r"\b(enabled|disabled)\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        // Version numbers
        Rule::new(r"\b\d+\.\d+(\.\d+)?\b")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
        common::number_rule(),
    ]
}

pub fn configure_program() -> Arc<SimpleProgram> {
    Arc::new(
        SimpleProgram::new(
            "dev.configure",
            "configure",
            "Autoconf configure script output",
            Category::Dev,
            configure_rules(),
        )
        .with_detect_patterns(vec!["./configure", "autoreconf", "autoconf"]),
    )
}

// =============================================================================
// ANT (Apache Ant)
// =============================================================================

fn ant_rules() -> Vec<Rule> {
    vec![
        // Target execution
        Rule::new(r"^[\w\-]+:$")
            .unwrap()
            .semantic(SemanticColor::Key)
            .bold()
            .build(),
        // Build status
        Rule::new(r"^BUILD SUCCESSFUL$")
            .unwrap()
            .semantic(SemanticColor::Success)
            .bold()
            .build(),
        Rule::new(r"^BUILD FAILED$")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
        // Task output prefixes
        Rule::new(r"^\s*\[(javac|java|copy|delete|mkdir|jar|war|echo|exec|junit|get)\]")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
        // Compile messages
        Rule::new(r"\bCompiling \d+ source files?")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\bCreated dir:")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\bDeleting:")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\bCopying \d+ files?")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        // Errors and warnings
        Rule::new(r"\berror:")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\bwarning:")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        // File paths
        Rule::new(r"\b[\w\-/]+\.java:\d+:")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        // Build timing
        Rule::new(r"Total time: \d+ \w+")
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
        // Buildfile location
        Rule::new(r"^Buildfile: \S+")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        common::number_rule(),
    ]
}

pub fn ant_program() -> Arc<SimpleProgram> {
    Arc::new(
        SimpleProgram::new(
            "dev.ant",
            "ant",
            "Apache Ant build output",
            Category::Dev,
            ant_rules(),
        )
        .with_detect_patterns(vec!["ant"]),
    )
}

// =============================================================================
// MVN (Apache Maven)
// =============================================================================

fn mvn_rules() -> Vec<Rule> {
    vec![
        // Build phases
        Rule::new(r"^\[INFO\] ---.*---$")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"^\[INFO\] Building ")
            .unwrap()
            .semantic(SemanticColor::Info)
            .bold()
            .build(),
        // Build status
        Rule::new(r"^\[INFO\] BUILD SUCCESS$")
            .unwrap()
            .semantic(SemanticColor::Success)
            .bold()
            .build(),
        Rule::new(r"^\[INFO\] BUILD FAILURE$")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
        Rule::new(r"^\[INFO\] BUILD ERROR$")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        // Log levels
        Rule::new(r"^\[INFO\]")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"^\[WARNING\]")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"^\[ERROR\]")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"^\[DEBUG\]")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        // Plugin execution
        Rule::new(r"\bmaven-\w+-plugin:\d+\.\d+(\.\d+)?:\w+")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        // Artifact coordinates
        Rule::new(r"\b[\w\.\-]+:[\w\.\-]+:[\w\.\-]+:\d+\.\d+(\.\d+)?")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
        // Download progress
        Rule::new(r"Downloading from \w+:")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        Rule::new(r"Downloaded from \w+:")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        // Test results
        Rule::new(r"Tests run: \d+, Failures: 0, Errors: 0")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"Tests run: \d+, Failures: [1-9]\d*")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
        Rule::new(r"Tests run: \d+, Failures: \d+, Errors: [1-9]\d*")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        // Total time
        Rule::new(r"Total time:\s+[\d:\.]+\s*(s|min)")
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
        // Reactor summary
        Rule::new(r"Reactor Summary")
            .unwrap()
            .semantic(SemanticColor::Key)
            .bold()
            .build(),
        Rule::new(r"\bSUCCESS\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"\bFAILURE\b")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
        Rule::new(r"\bSKIPPED\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        common::size_rule(),
        common::number_rule(),
    ]
}

pub fn mvn_program() -> Arc<SimpleProgram> {
    Arc::new(
        SimpleProgram::new(
            "dev.mvn",
            "mvn",
            "Apache Maven build output",
            Category::Dev,
            mvn_rules(),
        )
        .with_detect_patterns(vec!["mvn", "maven", "mvnw"]),
    )
}
