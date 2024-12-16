#[doc = "Register `SUS_STATUS` reader"]
pub type R = crate::R<SUS_STATUS_SPEC>;
#[doc = "Register `SUS_STATUS` writer"]
pub type W = crate::W<SUS_STATUS_SPEC>;
#[doc = "Field `FLASH_SUS` reader - The status of flash suspend. This bit is set when PES command is sent, and cleared when PER is sent. Only used in SPI1."]
pub type FLASH_SUS_R = crate::BitReader;
#[doc = "Field `FLASH_SUS` writer - The status of flash suspend. This bit is set when PES command is sent, and cleared when PER is sent. Only used in SPI1."]
pub type FLASH_SUS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_HPM_DLY_256` reader - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 256) SPI_CLK cycles after HPM command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after HPM command is sent."]
pub type FLASH_HPM_DLY_256_R = crate::BitReader;
#[doc = "Field `FLASH_HPM_DLY_256` writer - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 256) SPI_CLK cycles after HPM command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after HPM command is sent."]
pub type FLASH_HPM_DLY_256_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_RES_DLY_256` reader - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 256) SPI_CLK cycles after RES command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after RES command is sent."]
pub type FLASH_RES_DLY_256_R = crate::BitReader;
#[doc = "Field `FLASH_RES_DLY_256` writer - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 256) SPI_CLK cycles after RES command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after RES command is sent."]
pub type FLASH_RES_DLY_256_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_DP_DLY_256` reader - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 256) SPI_CLK cycles after DP command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after DP command is sent."]
pub type FLASH_DP_DLY_256_R = crate::BitReader;
#[doc = "Field `FLASH_DP_DLY_256` writer - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 256) SPI_CLK cycles after DP command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after DP command is sent."]
pub type FLASH_DP_DLY_256_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_PER_DLY_256` reader - Valid when SPI_MEM_FLASH_PER_WAIT_EN is 1. 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 256) SPI_CLK cycles after PER command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after PER command is sent."]
pub type FLASH_PER_DLY_256_R = crate::BitReader;
#[doc = "Field `FLASH_PER_DLY_256` writer - Valid when SPI_MEM_FLASH_PER_WAIT_EN is 1. 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 256) SPI_CLK cycles after PER command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after PER command is sent."]
pub type FLASH_PER_DLY_256_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_PES_DLY_256` reader - Valid when SPI_MEM_FLASH_PES_WAIT_EN is 1. 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 256) SPI_CLK cycles after PES command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after PES command is sent."]
pub type FLASH_PES_DLY_256_R = crate::BitReader;
#[doc = "Field `FLASH_PES_DLY_256` writer - Valid when SPI_MEM_FLASH_PES_WAIT_EN is 1. 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 256) SPI_CLK cycles after PES command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after PES command is sent."]
pub type FLASH_PES_DLY_256_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The status of flash suspend. This bit is set when PES command is sent, and cleared when PER is sent. Only used in SPI1."]
    #[inline(always)]
    pub fn flash_sus(&self) -> FLASH_SUS_R {
        FLASH_SUS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 256) SPI_CLK cycles after HPM command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after HPM command is sent."]
    #[inline(always)]
    pub fn flash_hpm_dly_256(&self) -> FLASH_HPM_DLY_256_R {
        FLASH_HPM_DLY_256_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 256) SPI_CLK cycles after RES command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after RES command is sent."]
    #[inline(always)]
    pub fn flash_res_dly_256(&self) -> FLASH_RES_DLY_256_R {
        FLASH_RES_DLY_256_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 256) SPI_CLK cycles after DP command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after DP command is sent."]
    #[inline(always)]
    pub fn flash_dp_dly_256(&self) -> FLASH_DP_DLY_256_R {
        FLASH_DP_DLY_256_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Valid when SPI_MEM_FLASH_PER_WAIT_EN is 1. 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 256) SPI_CLK cycles after PER command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after PER command is sent."]
    #[inline(always)]
    pub fn flash_per_dly_256(&self) -> FLASH_PER_DLY_256_R {
        FLASH_PER_DLY_256_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Valid when SPI_MEM_FLASH_PES_WAIT_EN is 1. 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 256) SPI_CLK cycles after PES command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after PES command is sent."]
    #[inline(always)]
    pub fn flash_pes_dly_256(&self) -> FLASH_PES_DLY_256_R {
        FLASH_PES_DLY_256_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SUS_STATUS")
            .field("flash_sus", &self.flash_sus())
            .field("flash_hpm_dly_256", &self.flash_hpm_dly_256())
            .field("flash_res_dly_256", &self.flash_res_dly_256())
            .field("flash_dp_dly_256", &self.flash_dp_dly_256())
            .field("flash_per_dly_256", &self.flash_per_dly_256())
            .field("flash_pes_dly_256", &self.flash_pes_dly_256())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The status of flash suspend. This bit is set when PES command is sent, and cleared when PER is sent. Only used in SPI1."]
    #[inline(always)]
    pub fn flash_sus(&mut self) -> FLASH_SUS_W<SUS_STATUS_SPEC> {
        FLASH_SUS_W::new(self, 0)
    }
    #[doc = "Bit 2 - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 256) SPI_CLK cycles after HPM command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after HPM command is sent."]
    #[inline(always)]
    pub fn flash_hpm_dly_256(&mut self) -> FLASH_HPM_DLY_256_W<SUS_STATUS_SPEC> {
        FLASH_HPM_DLY_256_W::new(self, 2)
    }
    #[doc = "Bit 3 - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 256) SPI_CLK cycles after RES command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after RES command is sent."]
    #[inline(always)]
    pub fn flash_res_dly_256(&mut self) -> FLASH_RES_DLY_256_W<SUS_STATUS_SPEC> {
        FLASH_RES_DLY_256_W::new(self, 3)
    }
    #[doc = "Bit 4 - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 256) SPI_CLK cycles after DP command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after DP command is sent."]
    #[inline(always)]
    pub fn flash_dp_dly_256(&mut self) -> FLASH_DP_DLY_256_W<SUS_STATUS_SPEC> {
        FLASH_DP_DLY_256_W::new(self, 4)
    }
    #[doc = "Bit 5 - Valid when SPI_MEM_FLASH_PER_WAIT_EN is 1. 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 256) SPI_CLK cycles after PER command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after PER command is sent."]
    #[inline(always)]
    pub fn flash_per_dly_256(&mut self) -> FLASH_PER_DLY_256_W<SUS_STATUS_SPEC> {
        FLASH_PER_DLY_256_W::new(self, 5)
    }
    #[doc = "Bit 6 - Valid when SPI_MEM_FLASH_PES_WAIT_EN is 1. 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 256) SPI_CLK cycles after PES command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after PES command is sent."]
    #[inline(always)]
    pub fn flash_pes_dly_256(&mut self) -> FLASH_PES_DLY_256_W<SUS_STATUS_SPEC> {
        FLASH_PES_DLY_256_W::new(self, 6)
    }
}
#[doc = "SPI1 flash suspend status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sus_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sus_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SUS_STATUS_SPEC;
impl crate::RegisterSpec for SUS_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sus_status::R`](R) reader structure"]
impl crate::Readable for SUS_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sus_status::W`](W) writer structure"]
impl crate::Writable for SUS_STATUS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SUS_STATUS to value 0"]
impl crate::Resettable for SUS_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
