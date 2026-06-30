#[doc = "Register `PRDYN_ST` reader"]
pub type R = crate::R<PRDYN_ST_SPEC>;
#[doc = "Field `HP_ROM_PRDYN` reader - record memory power status"]
pub type HP_ROM_PRDYN_R = crate::BitReader;
#[doc = "Field `HP_CPU_MEM_PRDYN` reader - record memory power status"]
pub type HP_CPU_MEM_PRDYN_R = crate::BitReader;
#[doc = "Field `HP_BITSCRAMBLER_MEM_PRDYN` reader - record memory power status"]
pub type HP_BITSCRAMBLER_MEM_PRDYN_R = crate::BitReader;
#[doc = "Field `HP_USB_MEM_PRDYN` reader - record memory power status"]
pub type HP_USB_MEM_PRDYN_R = crate::BitReader;
#[doc = "Field `HP_RMT_MEM_PRDYN` reader - record memory power status"]
pub type HP_RMT_MEM_PRDYN_R = crate::BitReader;
#[doc = "Field `HP_LEDC0_MEM_PRDYN` reader - record memory power status"]
pub type HP_LEDC0_MEM_PRDYN_R = crate::BitReader;
#[doc = "Field `HP_KEY_MANAGER_MEM_PRDYN` reader - record memory power status"]
pub type HP_KEY_MANAGER_MEM_PRDYN_R = crate::BitReader;
#[doc = "Field `HP_VPU_MEM_PRDYN` reader - record memory power status"]
pub type HP_VPU_MEM_PRDYN_R = crate::BitReader;
#[doc = "Field `HP_GDMA_MEM_PRDYN` reader - record memory power status"]
pub type HP_GDMA_MEM_PRDYN_R = crate::BitReader;
#[doc = "Field `HP_UART_MEM_PRDYN` reader - record memory power status"]
pub type HP_UART_MEM_PRDYN_R = crate::BitReader;
#[doc = "Field `HP_CRYPTO_MEM_PRDYN` reader - record memory power status"]
pub type HP_CRYPTO_MEM_PRDYN_R = crate::BitReader;
#[doc = "Field `HP_FLASH_MSPI_MEM_PRDYN` reader - record memory power status"]
pub type HP_FLASH_MSPI_MEM_PRDYN_R = crate::BitReader;
#[doc = "Field `HP_PSRAM_MSPI_MEM_PRDYN` reader - record memory power status"]
pub type HP_PSRAM_MSPI_MEM_PRDYN_R = crate::BitReader;
#[doc = "Field `HP_MODEM_MEM_PRDYN` reader - record memory power status"]
pub type HP_MODEM_MEM_PRDYN_R = crate::BitReader;
#[doc = "Field `HP_LEDC1_MEM_PRDYN` reader - record memory power status"]
pub type HP_LEDC1_MEM_PRDYN_R = crate::BitReader;
impl R {
    #[doc = "Bit 1 - record memory power status"]
    #[inline(always)]
    pub fn hp_rom_prdyn(&self) -> HP_ROM_PRDYN_R {
        HP_ROM_PRDYN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - record memory power status"]
    #[inline(always)]
    pub fn hp_cpu_mem_prdyn(&self) -> HP_CPU_MEM_PRDYN_R {
        HP_CPU_MEM_PRDYN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - record memory power status"]
    #[inline(always)]
    pub fn hp_bitscrambler_mem_prdyn(&self) -> HP_BITSCRAMBLER_MEM_PRDYN_R {
        HP_BITSCRAMBLER_MEM_PRDYN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - record memory power status"]
    #[inline(always)]
    pub fn hp_usb_mem_prdyn(&self) -> HP_USB_MEM_PRDYN_R {
        HP_USB_MEM_PRDYN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - record memory power status"]
    #[inline(always)]
    pub fn hp_rmt_mem_prdyn(&self) -> HP_RMT_MEM_PRDYN_R {
        HP_RMT_MEM_PRDYN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - record memory power status"]
    #[inline(always)]
    pub fn hp_ledc0_mem_prdyn(&self) -> HP_LEDC0_MEM_PRDYN_R {
        HP_LEDC0_MEM_PRDYN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - record memory power status"]
    #[inline(always)]
    pub fn hp_key_manager_mem_prdyn(&self) -> HP_KEY_MANAGER_MEM_PRDYN_R {
        HP_KEY_MANAGER_MEM_PRDYN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - record memory power status"]
    #[inline(always)]
    pub fn hp_vpu_mem_prdyn(&self) -> HP_VPU_MEM_PRDYN_R {
        HP_VPU_MEM_PRDYN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - record memory power status"]
    #[inline(always)]
    pub fn hp_gdma_mem_prdyn(&self) -> HP_GDMA_MEM_PRDYN_R {
        HP_GDMA_MEM_PRDYN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - record memory power status"]
    #[inline(always)]
    pub fn hp_uart_mem_prdyn(&self) -> HP_UART_MEM_PRDYN_R {
        HP_UART_MEM_PRDYN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - record memory power status"]
    #[inline(always)]
    pub fn hp_crypto_mem_prdyn(&self) -> HP_CRYPTO_MEM_PRDYN_R {
        HP_CRYPTO_MEM_PRDYN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - record memory power status"]
    #[inline(always)]
    pub fn hp_flash_mspi_mem_prdyn(&self) -> HP_FLASH_MSPI_MEM_PRDYN_R {
        HP_FLASH_MSPI_MEM_PRDYN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - record memory power status"]
    #[inline(always)]
    pub fn hp_psram_mspi_mem_prdyn(&self) -> HP_PSRAM_MSPI_MEM_PRDYN_R {
        HP_PSRAM_MSPI_MEM_PRDYN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - record memory power status"]
    #[inline(always)]
    pub fn hp_modem_mem_prdyn(&self) -> HP_MODEM_MEM_PRDYN_R {
        HP_MODEM_MEM_PRDYN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - record memory power status"]
    #[inline(always)]
    pub fn hp_ledc1_mem_prdyn(&self) -> HP_LEDC1_MEM_PRDYN_R {
        HP_LEDC1_MEM_PRDYN_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRDYN_ST")
            .field("hp_rom_prdyn", &self.hp_rom_prdyn())
            .field("hp_cpu_mem_prdyn", &self.hp_cpu_mem_prdyn())
            .field(
                "hp_bitscrambler_mem_prdyn",
                &self.hp_bitscrambler_mem_prdyn(),
            )
            .field("hp_usb_mem_prdyn", &self.hp_usb_mem_prdyn())
            .field("hp_rmt_mem_prdyn", &self.hp_rmt_mem_prdyn())
            .field("hp_ledc0_mem_prdyn", &self.hp_ledc0_mem_prdyn())
            .field("hp_key_manager_mem_prdyn", &self.hp_key_manager_mem_prdyn())
            .field("hp_vpu_mem_prdyn", &self.hp_vpu_mem_prdyn())
            .field("hp_gdma_mem_prdyn", &self.hp_gdma_mem_prdyn())
            .field("hp_uart_mem_prdyn", &self.hp_uart_mem_prdyn())
            .field("hp_crypto_mem_prdyn", &self.hp_crypto_mem_prdyn())
            .field("hp_flash_mspi_mem_prdyn", &self.hp_flash_mspi_mem_prdyn())
            .field("hp_psram_mspi_mem_prdyn", &self.hp_psram_mspi_mem_prdyn())
            .field("hp_modem_mem_prdyn", &self.hp_modem_mem_prdyn())
            .field("hp_ledc1_mem_prdyn", &self.hp_ledc1_mem_prdyn())
            .finish()
    }
}
#[doc = "hp system prdyn status register\n\nYou can [`read`](crate::Reg::read) this register and get [`prdyn_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRDYN_ST_SPEC;
impl crate::RegisterSpec for PRDYN_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prdyn_st::R`](R) reader structure"]
impl crate::Readable for PRDYN_ST_SPEC {}
#[doc = "`reset()` method sets PRDYN_ST to value 0x000f_d7da"]
impl crate::Resettable for PRDYN_ST_SPEC {
    const RESET_VALUE: u32 = 0x000f_d7da;
}
