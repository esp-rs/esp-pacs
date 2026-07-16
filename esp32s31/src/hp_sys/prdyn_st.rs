#[doc = "Register `PRDYN_ST` reader"]
pub type R = crate::R<PRDYN_ST_SPEC>;
#[doc = "Field `ROM_PRDYN` reader - record memory power status"]
pub type ROM_PRDYN_R = crate::BitReader;
#[doc = "Field `CPU_MEM_PRDYN` reader - record memory power status"]
pub type CPU_MEM_PRDYN_R = crate::BitReader;
#[doc = "Field `BITSCRAMBLER_MEM_PRDYN` reader - record memory power status"]
pub type BITSCRAMBLER_MEM_PRDYN_R = crate::BitReader;
#[doc = "Field `USB_MEM_PRDYN` reader - record memory power status"]
pub type USB_MEM_PRDYN_R = crate::BitReader;
#[doc = "Field `RMT_MEM_PRDYN` reader - record memory power status"]
pub type RMT_MEM_PRDYN_R = crate::BitReader;
#[doc = "Field `LEDC0_MEM_PRDYN` reader - record memory power status"]
pub type LEDC0_MEM_PRDYN_R = crate::BitReader;
#[doc = "Field `KEY_MANAGER_MEM_PRDYN` reader - record memory power status"]
pub type KEY_MANAGER_MEM_PRDYN_R = crate::BitReader;
#[doc = "Field `VPU_MEM_PRDYN` reader - record memory power status"]
pub type VPU_MEM_PRDYN_R = crate::BitReader;
#[doc = "Field `GDMA_MEM_PRDYN` reader - record memory power status"]
pub type GDMA_MEM_PRDYN_R = crate::BitReader;
#[doc = "Field `UART_MEM_PRDYN` reader - record memory power status"]
pub type UART_MEM_PRDYN_R = crate::BitReader;
#[doc = "Field `CRYPTO_MEM_PRDYN` reader - record memory power status"]
pub type CRYPTO_MEM_PRDYN_R = crate::BitReader;
#[doc = "Field `FLASH_MSPI_MEM_PRDYN` reader - record memory power status"]
pub type FLASH_MSPI_MEM_PRDYN_R = crate::BitReader;
#[doc = "Field `PSRAM_MSPI_MEM_PRDYN` reader - record memory power status"]
pub type PSRAM_MSPI_MEM_PRDYN_R = crate::BitReader;
#[doc = "Field `MODEM_MEM_PRDYN` reader - record memory power status"]
pub type MODEM_MEM_PRDYN_R = crate::BitReader;
#[doc = "Field `LEDC1_MEM_PRDYN` reader - record memory power status"]
pub type LEDC1_MEM_PRDYN_R = crate::BitReader;
impl R {
    #[doc = "Bit 1 - record memory power status"]
    #[inline(always)]
    pub fn rom_prdyn(&self) -> ROM_PRDYN_R {
        ROM_PRDYN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - record memory power status"]
    #[inline(always)]
    pub fn cpu_mem_prdyn(&self) -> CPU_MEM_PRDYN_R {
        CPU_MEM_PRDYN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - record memory power status"]
    #[inline(always)]
    pub fn bitscrambler_mem_prdyn(&self) -> BITSCRAMBLER_MEM_PRDYN_R {
        BITSCRAMBLER_MEM_PRDYN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - record memory power status"]
    #[inline(always)]
    pub fn usb_mem_prdyn(&self) -> USB_MEM_PRDYN_R {
        USB_MEM_PRDYN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - record memory power status"]
    #[inline(always)]
    pub fn rmt_mem_prdyn(&self) -> RMT_MEM_PRDYN_R {
        RMT_MEM_PRDYN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - record memory power status"]
    #[inline(always)]
    pub fn ledc0_mem_prdyn(&self) -> LEDC0_MEM_PRDYN_R {
        LEDC0_MEM_PRDYN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - record memory power status"]
    #[inline(always)]
    pub fn key_manager_mem_prdyn(&self) -> KEY_MANAGER_MEM_PRDYN_R {
        KEY_MANAGER_MEM_PRDYN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - record memory power status"]
    #[inline(always)]
    pub fn vpu_mem_prdyn(&self) -> VPU_MEM_PRDYN_R {
        VPU_MEM_PRDYN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - record memory power status"]
    #[inline(always)]
    pub fn gdma_mem_prdyn(&self) -> GDMA_MEM_PRDYN_R {
        GDMA_MEM_PRDYN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - record memory power status"]
    #[inline(always)]
    pub fn uart_mem_prdyn(&self) -> UART_MEM_PRDYN_R {
        UART_MEM_PRDYN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - record memory power status"]
    #[inline(always)]
    pub fn crypto_mem_prdyn(&self) -> CRYPTO_MEM_PRDYN_R {
        CRYPTO_MEM_PRDYN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - record memory power status"]
    #[inline(always)]
    pub fn flash_mspi_mem_prdyn(&self) -> FLASH_MSPI_MEM_PRDYN_R {
        FLASH_MSPI_MEM_PRDYN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - record memory power status"]
    #[inline(always)]
    pub fn psram_mspi_mem_prdyn(&self) -> PSRAM_MSPI_MEM_PRDYN_R {
        PSRAM_MSPI_MEM_PRDYN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - record memory power status"]
    #[inline(always)]
    pub fn modem_mem_prdyn(&self) -> MODEM_MEM_PRDYN_R {
        MODEM_MEM_PRDYN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - record memory power status"]
    #[inline(always)]
    pub fn ledc1_mem_prdyn(&self) -> LEDC1_MEM_PRDYN_R {
        LEDC1_MEM_PRDYN_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRDYN_ST")
            .field("rom_prdyn", &self.rom_prdyn())
            .field("cpu_mem_prdyn", &self.cpu_mem_prdyn())
            .field("bitscrambler_mem_prdyn", &self.bitscrambler_mem_prdyn())
            .field("usb_mem_prdyn", &self.usb_mem_prdyn())
            .field("rmt_mem_prdyn", &self.rmt_mem_prdyn())
            .field("ledc0_mem_prdyn", &self.ledc0_mem_prdyn())
            .field("key_manager_mem_prdyn", &self.key_manager_mem_prdyn())
            .field("vpu_mem_prdyn", &self.vpu_mem_prdyn())
            .field("gdma_mem_prdyn", &self.gdma_mem_prdyn())
            .field("uart_mem_prdyn", &self.uart_mem_prdyn())
            .field("crypto_mem_prdyn", &self.crypto_mem_prdyn())
            .field("flash_mspi_mem_prdyn", &self.flash_mspi_mem_prdyn())
            .field("psram_mspi_mem_prdyn", &self.psram_mspi_mem_prdyn())
            .field("modem_mem_prdyn", &self.modem_mem_prdyn())
            .field("ledc1_mem_prdyn", &self.ledc1_mem_prdyn())
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
