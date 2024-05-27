#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    external_device_encrypt_decrypt_control: EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL,
    sram_usage_conf: SRAM_USAGE_CONF,
    sec_dpa_conf: SEC_DPA_CONF,
    cpu_peri_timeout_conf: CPU_PERI_TIMEOUT_CONF,
    cpu_peri_timeout_addr: CPU_PERI_TIMEOUT_ADDR,
    cpu_peri_timeout_uid: CPU_PERI_TIMEOUT_UID,
    hp_peri_timeout_conf: HP_PERI_TIMEOUT_CONF,
    hp_peri_timeout_addr: HP_PERI_TIMEOUT_ADDR,
    hp_peri_timeout_uid: HP_PERI_TIMEOUT_UID,
    rom_table_lock: ROM_TABLE_LOCK,
    rom_table: ROM_TABLE,
    mem_test_conf: MEM_TEST_CONF,
    _reserved12: [u8; 0x03b0],
    rnd_eco: RND_ECO,
    rnd_eco_low: RND_ECO_LOW,
    rnd_eco_high: RND_ECO_HIGH,
    _reserved15: [u8; 0x0c],
    clock_gate: CLOCK_GATE,
    date: DATE,
}
impl RegisterBlock {
    ///0x00 - EXTERNAL DEVICE ENCRYPTION/DECRYPTION configuration register
    #[inline(always)]
    pub const fn external_device_encrypt_decrypt_control(
        &self,
    ) -> &EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL {
        &self.external_device_encrypt_decrypt_control
    }
    ///0x04 - HP memory usage configuration register
    #[inline(always)]
    pub const fn sram_usage_conf(&self) -> &SRAM_USAGE_CONF {
        &self.sram_usage_conf
    }
    ///0x08 - HP anti-DPA security configuration register
    #[inline(always)]
    pub const fn sec_dpa_conf(&self) -> &SEC_DPA_CONF {
        &self.sec_dpa_conf
    }
    ///0x0c - CPU_PERI_TIMEOUT configuration register
    #[inline(always)]
    pub const fn cpu_peri_timeout_conf(&self) -> &CPU_PERI_TIMEOUT_CONF {
        &self.cpu_peri_timeout_conf
    }
    ///0x10 - CPU_PERI_TIMEOUT_ADDR register
    #[inline(always)]
    pub const fn cpu_peri_timeout_addr(&self) -> &CPU_PERI_TIMEOUT_ADDR {
        &self.cpu_peri_timeout_addr
    }
    ///0x14 - CPU_PERI_TIMEOUT_UID register
    #[inline(always)]
    pub const fn cpu_peri_timeout_uid(&self) -> &CPU_PERI_TIMEOUT_UID {
        &self.cpu_peri_timeout_uid
    }
    ///0x18 - HP_PERI_TIMEOUT configuration register
    #[inline(always)]
    pub const fn hp_peri_timeout_conf(&self) -> &HP_PERI_TIMEOUT_CONF {
        &self.hp_peri_timeout_conf
    }
    ///0x1c - HP_PERI_TIMEOUT_ADDR register
    #[inline(always)]
    pub const fn hp_peri_timeout_addr(&self) -> &HP_PERI_TIMEOUT_ADDR {
        &self.hp_peri_timeout_addr
    }
    ///0x20 - HP_PERI_TIMEOUT_UID register
    #[inline(always)]
    pub const fn hp_peri_timeout_uid(&self) -> &HP_PERI_TIMEOUT_UID {
        &self.hp_peri_timeout_uid
    }
    ///0x24 - Rom-Table lock register
    #[inline(always)]
    pub const fn rom_table_lock(&self) -> &ROM_TABLE_LOCK {
        &self.rom_table_lock
    }
    ///0x28 - Rom-Table register
    #[inline(always)]
    pub const fn rom_table(&self) -> &ROM_TABLE {
        &self.rom_table
    }
    ///0x2c - MEM_TEST configuration register
    #[inline(always)]
    pub const fn mem_test_conf(&self) -> &MEM_TEST_CONF {
        &self.mem_test_conf
    }
    ///0x3e0 - redcy eco register.
    #[inline(always)]
    pub const fn rnd_eco(&self) -> &RND_ECO {
        &self.rnd_eco
    }
    ///0x3e4 - redcy eco low register.
    #[inline(always)]
    pub const fn rnd_eco_low(&self) -> &RND_ECO_LOW {
        &self.rnd_eco_low
    }
    ///0x3e8 - redcy eco high register.
    #[inline(always)]
    pub const fn rnd_eco_high(&self) -> &RND_ECO_HIGH {
        &self.rnd_eco_high
    }
    ///0x3f8 - HP-SYSTEM clock gating configure register
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    ///0x3fc - Date register.
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
/**EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL (rw) register accessor: EXTERNAL DEVICE ENCRYPTION/DECRYPTION configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`external_device_encrypt_decrypt_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`external_device_encrypt_decrypt_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@external_device_encrypt_decrypt_control`] module*/
pub type EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL = crate::Reg<
    external_device_encrypt_decrypt_control::EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC,
>;
///EXTERNAL DEVICE ENCRYPTION/DECRYPTION configuration register
pub mod external_device_encrypt_decrypt_control;
/**SRAM_USAGE_CONF (rw) register accessor: HP memory usage configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`sram_usage_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_usage_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sram_usage_conf`] module*/
pub type SRAM_USAGE_CONF = crate::Reg<sram_usage_conf::SRAM_USAGE_CONF_SPEC>;
///HP memory usage configuration register
pub mod sram_usage_conf;
/**SEC_DPA_CONF (rw) register accessor: HP anti-DPA security configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`sec_dpa_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sec_dpa_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sec_dpa_conf`] module*/
pub type SEC_DPA_CONF = crate::Reg<sec_dpa_conf::SEC_DPA_CONF_SPEC>;
///HP anti-DPA security configuration register
pub mod sec_dpa_conf;
/**CPU_PERI_TIMEOUT_CONF (rw) register accessor: CPU_PERI_TIMEOUT configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`cpu_peri_timeout_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_peri_timeout_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cpu_peri_timeout_conf`] module*/
pub type CPU_PERI_TIMEOUT_CONF = crate::Reg<cpu_peri_timeout_conf::CPU_PERI_TIMEOUT_CONF_SPEC>;
///CPU_PERI_TIMEOUT configuration register
pub mod cpu_peri_timeout_conf;
/**CPU_PERI_TIMEOUT_ADDR (r) register accessor: CPU_PERI_TIMEOUT_ADDR register

You can [`read`](crate::generic::Reg::read) this register and get [`cpu_peri_timeout_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cpu_peri_timeout_addr`] module*/
pub type CPU_PERI_TIMEOUT_ADDR = crate::Reg<cpu_peri_timeout_addr::CPU_PERI_TIMEOUT_ADDR_SPEC>;
///CPU_PERI_TIMEOUT_ADDR register
pub mod cpu_peri_timeout_addr;
/**CPU_PERI_TIMEOUT_UID (r) register accessor: CPU_PERI_TIMEOUT_UID register

You can [`read`](crate::generic::Reg::read) this register and get [`cpu_peri_timeout_uid::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cpu_peri_timeout_uid`] module*/
pub type CPU_PERI_TIMEOUT_UID = crate::Reg<cpu_peri_timeout_uid::CPU_PERI_TIMEOUT_UID_SPEC>;
///CPU_PERI_TIMEOUT_UID register
pub mod cpu_peri_timeout_uid;
/**HP_PERI_TIMEOUT_CONF (rw) register accessor: HP_PERI_TIMEOUT configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`hp_peri_timeout_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_peri_timeout_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hp_peri_timeout_conf`] module*/
pub type HP_PERI_TIMEOUT_CONF = crate::Reg<hp_peri_timeout_conf::HP_PERI_TIMEOUT_CONF_SPEC>;
///HP_PERI_TIMEOUT configuration register
pub mod hp_peri_timeout_conf;
/**HP_PERI_TIMEOUT_ADDR (r) register accessor: HP_PERI_TIMEOUT_ADDR register

You can [`read`](crate::generic::Reg::read) this register and get [`hp_peri_timeout_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hp_peri_timeout_addr`] module*/
pub type HP_PERI_TIMEOUT_ADDR = crate::Reg<hp_peri_timeout_addr::HP_PERI_TIMEOUT_ADDR_SPEC>;
///HP_PERI_TIMEOUT_ADDR register
pub mod hp_peri_timeout_addr;
/**HP_PERI_TIMEOUT_UID (r) register accessor: HP_PERI_TIMEOUT_UID register

You can [`read`](crate::generic::Reg::read) this register and get [`hp_peri_timeout_uid::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hp_peri_timeout_uid`] module*/
pub type HP_PERI_TIMEOUT_UID = crate::Reg<hp_peri_timeout_uid::HP_PERI_TIMEOUT_UID_SPEC>;
///HP_PERI_TIMEOUT_UID register
pub mod hp_peri_timeout_uid;
/**ROM_TABLE_LOCK (rw) register accessor: Rom-Table lock register

You can [`read`](crate::generic::Reg::read) this register and get [`rom_table_lock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rom_table_lock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rom_table_lock`] module*/
pub type ROM_TABLE_LOCK = crate::Reg<rom_table_lock::ROM_TABLE_LOCK_SPEC>;
///Rom-Table lock register
pub mod rom_table_lock;
/**ROM_TABLE (rw) register accessor: Rom-Table register

You can [`read`](crate::generic::Reg::read) this register and get [`rom_table::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rom_table::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rom_table`] module*/
pub type ROM_TABLE = crate::Reg<rom_table::ROM_TABLE_SPEC>;
///Rom-Table register
pub mod rom_table;
/**MEM_TEST_CONF (rw) register accessor: MEM_TEST configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`mem_test_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_test_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mem_test_conf`] module*/
pub type MEM_TEST_CONF = crate::Reg<mem_test_conf::MEM_TEST_CONF_SPEC>;
///MEM_TEST configuration register
pub mod mem_test_conf;
/**RND_ECO (rw) register accessor: redcy eco register.

You can [`read`](crate::generic::Reg::read) this register and get [`rnd_eco::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rnd_eco::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rnd_eco`] module*/
pub type RND_ECO = crate::Reg<rnd_eco::RND_ECO_SPEC>;
///redcy eco register.
pub mod rnd_eco;
/**RND_ECO_LOW (rw) register accessor: redcy eco low register.

You can [`read`](crate::generic::Reg::read) this register and get [`rnd_eco_low::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rnd_eco_low::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rnd_eco_low`] module*/
pub type RND_ECO_LOW = crate::Reg<rnd_eco_low::RND_ECO_LOW_SPEC>;
///redcy eco low register.
pub mod rnd_eco_low;
/**RND_ECO_HIGH (rw) register accessor: redcy eco high register.

You can [`read`](crate::generic::Reg::read) this register and get [`rnd_eco_high::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rnd_eco_high::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rnd_eco_high`] module*/
pub type RND_ECO_HIGH = crate::Reg<rnd_eco_high::RND_ECO_HIGH_SPEC>;
///redcy eco high register.
pub mod rnd_eco_high;
/**CLOCK_GATE (rw) register accessor: HP-SYSTEM clock gating configure register

You can [`read`](crate::generic::Reg::read) this register and get [`clock_gate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clock_gate`] module*/
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
///HP-SYSTEM clock gating configure register
pub mod clock_gate;
/**DATE (rw) register accessor: Date register.

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@date`] module*/
pub type DATE = crate::Reg<date::DATE_SPEC>;
///Date register.
pub mod date;
