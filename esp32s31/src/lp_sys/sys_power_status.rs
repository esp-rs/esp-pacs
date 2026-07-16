#[doc = "Register `SYS_POWER_STATUS` reader"]
pub type R = crate::R<SYS_POWER_STATUS_SPEC>;
#[doc = "Field `LP_TCM_RAM_PRDYN` reader - "]
pub type LP_TCM_RAM_PRDYN_R = crate::BitReader;
#[doc = "Field `LP_TCM_ROM_PRDYN` reader - "]
pub type LP_TCM_ROM_PRDYN_R = crate::BitReader;
#[doc = "Field `LP_HUK_MEM_PRDYN` reader - "]
pub type LP_HUK_MEM_PRDYN_R = crate::BitReader;
#[doc = "Field `LP_EFUSE_MEM_PRDYN` reader - "]
pub type LP_EFUSE_MEM_PRDYN_R = crate::BitReader;
#[doc = "Field `LP_UART_MEM_PRDYN` reader - "]
pub type LP_UART_MEM_PRDYN_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lp_tcm_ram_prdyn(&self) -> LP_TCM_RAM_PRDYN_R {
        LP_TCM_RAM_PRDYN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn lp_tcm_rom_prdyn(&self) -> LP_TCM_ROM_PRDYN_R {
        LP_TCM_ROM_PRDYN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn lp_huk_mem_prdyn(&self) -> LP_HUK_MEM_PRDYN_R {
        LP_HUK_MEM_PRDYN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn lp_efuse_mem_prdyn(&self) -> LP_EFUSE_MEM_PRDYN_R {
        LP_EFUSE_MEM_PRDYN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn lp_uart_mem_prdyn(&self) -> LP_UART_MEM_PRDYN_R {
        LP_UART_MEM_PRDYN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYS_POWER_STATUS")
            .field("lp_tcm_ram_prdyn", &self.lp_tcm_ram_prdyn())
            .field("lp_tcm_rom_prdyn", &self.lp_tcm_rom_prdyn())
            .field("lp_huk_mem_prdyn", &self.lp_huk_mem_prdyn())
            .field("lp_efuse_mem_prdyn", &self.lp_efuse_mem_prdyn())
            .field("lp_uart_mem_prdyn", &self.lp_uart_mem_prdyn())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_power_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_POWER_STATUS_SPEC;
impl crate::RegisterSpec for SYS_POWER_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_power_status::R`](R) reader structure"]
impl crate::Readable for SYS_POWER_STATUS_SPEC {}
#[doc = "`reset()` method sets SYS_POWER_STATUS to value 0x1f"]
impl crate::Resettable for SYS_POWER_STATUS_SPEC {
    const RESET_VALUE: u32 = 0x1f;
}
