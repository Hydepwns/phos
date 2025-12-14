//! Hardware and system statistics programs.
//!
//! Provides Program implementations for lsmod, lspci, vmstat, iostat, and env.

use std::sync::Arc;

use super::common;
use crate::colors::SemanticColor;
use crate::program::SimpleProgram;
use crate::rule::Rule;

// =============================================================================
// LSMOD (List Kernel Modules)
// =============================================================================

fn lsmod_rules() -> Vec<Rule> {
    vec![
        Rule::new(r"^Module\s+Size\s+Used by")
            .unwrap()
            .semantic(SemanticColor::Key)
            .bold()
            .build(),
        Rule::new(r"^[\w_]+")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"\s+\d+\s+")
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
        Rule::new(r"\b[a-z_]+,?")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        common::number_rule(),
    ]
}

pub fn lsmod_program() -> Arc<SimpleProgram> {
    Arc::new(
        SimpleProgram::new(
            "system.lsmod",
            "lsmod",
            "List kernel modules output",
            "system",
            lsmod_rules(),
        )
        .with_detect_patterns(vec!["lsmod"]),
    )
}

// =============================================================================
// LSPCI (List PCI Devices)
// =============================================================================

fn lspci_rules() -> Vec<Rule> {
    vec![
        Rule::new(r"^[0-9a-f]{2}:[0-9a-f]{2}\.[0-9a-f]")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"\b(VGA compatible controller|Network controller|USB controller|Audio device|SATA controller|PCI bridge|ISA bridge|SMBus|Ethernet controller|Non-Volatile memory controller|Communication controller|Serial controller):")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\b(Intel|AMD|NVIDIA|Realtek|Broadcom|Qualcomm|Samsung|Western Digital|SanDisk|Marvell|ASMedia)\b")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
        Rule::new(r"Subsystem:")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        Rule::new(r"\(rev\s+[0-9a-f]+\)")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
        Rule::new(r"\bMemory at [0-9a-f]+")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        Rule::new(r"\bIRQ\s+\d+")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
        common::number_rule(),
    ]
}

pub fn lspci_program() -> Arc<SimpleProgram> {
    Arc::new(
        SimpleProgram::new(
            "system.lspci",
            "lspci",
            "List PCI devices output",
            "system",
            lspci_rules(),
        )
        .with_detect_patterns(vec!["lspci"]),
    )
}

// =============================================================================
// VMSTAT (Virtual Memory Statistics)
// =============================================================================

fn vmstat_rules() -> Vec<Rule> {
    vec![
        Rule::new(r"^\s*procs\s+-+memory-+\s+-+swap-+\s+-+io-+\s+-+system-+\s+-+cpu-+")
            .unwrap()
            .semantic(SemanticColor::Key)
            .bold()
            .build(),
        Rule::new(r"^\s*r\s+b\s+swpd\s+free\s+buff\s+cache")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
        Rule::new(r"\b(procs|memory|swap|io|system|cpu|r|b|swpd|free|buff|cache|si|so|bi|bo|in|cs|us|sy|id|wa|st)\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\b[1-9]\d*\s+[1-9]\d*\s+\d+\s+\d+")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        common::size_rule(),
        common::percentage_rule(),
        common::number_rule(),
    ]
}

pub fn vmstat_program() -> Arc<SimpleProgram> {
    Arc::new(
        SimpleProgram::new(
            "system.vmstat",
            "vmstat",
            "Virtual memory statistics",
            "system",
            vmstat_rules(),
        )
        .with_detect_patterns(vec!["vmstat"]),
    )
}

// =============================================================================
// IOSTAT (I/O Statistics)
// =============================================================================

fn iostat_rules() -> Vec<Rule> {
    vec![
        Rule::new(r"^Linux\s+.*$")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"^avg-cpu:")
            .unwrap()
            .semantic(SemanticColor::Key)
            .bold()
            .build(),
        Rule::new(r"^Device:")
            .unwrap()
            .semantic(SemanticColor::Key)
            .bold()
            .build(),
        Rule::new(r"\b(%user|%nice|%system|%iowait|%steal|%idle|tps|kB_read/s|kB_wrtn/s|kB_read|kB_wrtn|Device)\b")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
        common::device_name_rule(),
        Rule::new(r"\b([2-9]\d\.\d{2}|100\.00)\s+\d+\.\d{2}\s+\d+\.\d{2}$")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        common::size_rule(),
        common::percentage_rule(),
        common::number_rule(),
    ]
}

pub fn iostat_program() -> Arc<SimpleProgram> {
    Arc::new(
        SimpleProgram::new(
            "system.iostat",
            "iostat",
            "I/O statistics output",
            "system",
            iostat_rules(),
        )
        .with_detect_patterns(vec!["iostat"]),
    )
}

// =============================================================================
// ENV (Environment Variables)
// =============================================================================

fn env_rules() -> Vec<Rule> {
    vec![
        Rule::new(r"^(PATH|HOME|USER|SHELL|LANG|TERM|PWD|EDITOR)=")
            .unwrap()
            .semantic(SemanticColor::Key)
            .bold()
            .build(),
        Rule::new(r"^[A-Z_][A-Z0-9_]*=")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"(/[^:\s]+)+")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"https?://[\w\.\-/]+")
            .unwrap()
            .semantic(SemanticColor::String)
            .build(),
        common::number_rule(),
    ]
}

pub fn env_program() -> Arc<SimpleProgram> {
    Arc::new(
        SimpleProgram::new(
            "system.env",
            "env",
            "Environment variables output",
            "system",
            env_rules(),
        )
        .with_detect_patterns(vec!["env", "printenv"]),
    )
}

// =============================================================================
// BLKID (Block Device Attributes)
// =============================================================================

fn blkid_rules() -> Vec<Rule> {
    vec![
        // Device path
        Rule::new(r"^/dev/[\w\d]+:")
            .unwrap()
            .semantic(SemanticColor::Key)
            .bold()
            .build(),
        // UUID
        Rule::new(r#"UUID="[a-fA-F0-9\-]+""#)
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        // PARTUUID
        Rule::new(r#"PARTUUID="[a-fA-F0-9\-]+""#)
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        // TYPE (filesystem type)
        Rule::new(r#"TYPE="[\w\d]+""#)
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
        // LABEL
        Rule::new(r#"LABEL="[^"]+""#)
            .unwrap()
            .semantic(SemanticColor::String)
            .build(),
        // PARTLABEL
        Rule::new(r#"PARTLABEL="[^"]+""#)
            .unwrap()
            .semantic(SemanticColor::String)
            .build(),
        // SEC_TYPE
        Rule::new(r#"SEC_TYPE="[^"]+""#)
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        // Block size
        Rule::new(r#"BLOCK_SIZE="\d+""#)
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
        common::number_rule(),
    ]
}

pub fn blkid_program() -> Arc<SimpleProgram> {
    Arc::new(
        SimpleProgram::new(
            "system.blkid",
            "blkid",
            "Block device attributes output",
            "system",
            blkid_rules(),
        )
        .with_detect_patterns(vec!["blkid"]),
    )
}

// =============================================================================
// FDISK (Partition Table)
// =============================================================================

fn fdisk_rules() -> Vec<Rule> {
    vec![
        // Disk header
        Rule::new(r"^Disk /dev/[\w\d]+:")
            .unwrap()
            .semantic(SemanticColor::Key)
            .bold()
            .build(),
        // Device boot column
        Rule::new(r"^/dev/[\w\d]+")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        // Boot flag
        Rule::new(r"\s+\*\s+")
            .unwrap()
            .semantic(SemanticColor::Success)
            .bold()
            .build(),
        // Column headers
        Rule::new(r"\b(Device|Boot|Start|End|Sectors|Size|Id|Type)\b")
            .unwrap()
            .semantic(SemanticColor::Label)
            .bold()
            .build(),
        // Partition types
        Rule::new(r"\b(Linux|Linux swap|EFI System|Microsoft|NTFS|FAT32|FAT16|Extended|Empty)\b")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
        // Size values
        common::size_rule(),
        // Disk model
        Rule::new(r"Disk model:")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        // Units
        Rule::new(r"Units:")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        Rule::new(r"Sector size")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        // Disklabel type
        Rule::new(r"Disklabel type:\s+\w+")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
        // Disk identifier
        Rule::new(r"Disk identifier:")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        common::number_rule(),
    ]
}

pub fn fdisk_program() -> Arc<SimpleProgram> {
    Arc::new(
        SimpleProgram::new(
            "system.fdisk",
            "fdisk",
            "Partition table output",
            "system",
            fdisk_rules(),
        )
        .with_detect_patterns(vec!["fdisk", "gdisk", "parted"]),
    )
}

// =============================================================================
// LSBLK (List Block Devices)
// =============================================================================

fn lsblk_rules() -> Vec<Rule> {
    vec![
        // Device names with tree chars
        Rule::new(r"^[├└│]─[\w\d]+")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"^\s*[\w\d]+")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        // Column headers
        Rule::new(r"\b(NAME|MAJ:MIN|RM|SIZE|RO|TYPE|MOUNTPOINT|MOUNTPOINTS|FSTYPE|LABEL|UUID|MODEL)\b")
            .unwrap()
            .semantic(SemanticColor::Label)
            .bold()
            .build(),
        // Device types
        Rule::new(r"\b(disk|part|lvm|crypt|loop|rom|raid\d*)\b")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
        // Mount points
        Rule::new(r"(/[\w\-\.]+)+")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        // Size
        common::size_rule(),
        // Read-only flag
        Rule::new(r"\s+0\s+")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"\s+1\s+")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        // Filesystem types
        common::filesystem_type_rule(),
        common::number_rule(),
    ]
}

pub fn lsblk_program() -> Arc<SimpleProgram> {
    Arc::new(
        SimpleProgram::new(
            "system.lsblk",
            "lsblk",
            "List block devices output",
            "system",
            lsblk_rules(),
        )
        .with_detect_patterns(vec!["lsblk"]),
    )
}
