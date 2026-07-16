#[doc = "Register `SUS_STATUS` reader"]
pub type R = crate::R<SUS_STATUS_SPEC>;
#[doc = "Register `SUS_STATUS` writer"]
pub type W = crate::W<SUS_STATUS_SPEC>;
#[doc = "Field `FLASH_SUS` reader - "]
pub type FLASH_SUS_R = crate::BitReader;
#[doc = "Field `FLASH_SUS` writer - "]
pub type FLASH_SUS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAIT_PESR_CMD_2B` reader - "]
pub type WAIT_PESR_CMD_2B_R = crate::BitReader;
#[doc = "Field `WAIT_PESR_CMD_2B` writer - "]
pub type WAIT_PESR_CMD_2B_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_HPM_DLY_128` reader - "]
pub type FLASH_HPM_DLY_128_R = crate::BitReader;
#[doc = "Field `FLASH_HPM_DLY_128` writer - "]
pub type FLASH_HPM_DLY_128_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_RES_DLY_128` reader - "]
pub type FLASH_RES_DLY_128_R = crate::BitReader;
#[doc = "Field `FLASH_RES_DLY_128` writer - "]
pub type FLASH_RES_DLY_128_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_DP_DLY_128` reader - "]
pub type FLASH_DP_DLY_128_R = crate::BitReader;
#[doc = "Field `FLASH_DP_DLY_128` writer - "]
pub type FLASH_DP_DLY_128_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_PER_DLY_128` reader - "]
pub type FLASH_PER_DLY_128_R = crate::BitReader;
#[doc = "Field `FLASH_PER_DLY_128` writer - "]
pub type FLASH_PER_DLY_128_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_PES_DLY_128` reader - "]
pub type FLASH_PES_DLY_128_R = crate::BitReader;
#[doc = "Field `FLASH_PES_DLY_128` writer - "]
pub type FLASH_PES_DLY_128_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI0_LOCK_EN` reader - "]
pub type SPI0_LOCK_EN_R = crate::BitReader;
#[doc = "Field `SPI0_LOCK_EN` writer - "]
pub type SPI0_LOCK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_PESR_CMD_2B` reader - "]
pub type FLASH_PESR_CMD_2B_R = crate::BitReader;
#[doc = "Field `FLASH_PESR_CMD_2B` writer - "]
pub type FLASH_PESR_CMD_2B_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_PER_COMMAND` reader - "]
pub type FLASH_PER_COMMAND_R = crate::FieldReader<u16>;
#[doc = "Field `FLASH_PER_COMMAND` writer - "]
pub type FLASH_PER_COMMAND_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn flash_sus(&self) -> FLASH_SUS_R {
        FLASH_SUS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn wait_pesr_cmd_2b(&self) -> WAIT_PESR_CMD_2B_R {
        WAIT_PESR_CMD_2B_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn flash_hpm_dly_128(&self) -> FLASH_HPM_DLY_128_R {
        FLASH_HPM_DLY_128_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn flash_res_dly_128(&self) -> FLASH_RES_DLY_128_R {
        FLASH_RES_DLY_128_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn flash_dp_dly_128(&self) -> FLASH_DP_DLY_128_R {
        FLASH_DP_DLY_128_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn flash_per_dly_128(&self) -> FLASH_PER_DLY_128_R {
        FLASH_PER_DLY_128_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn flash_pes_dly_128(&self) -> FLASH_PES_DLY_128_R {
        FLASH_PES_DLY_128_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn spi0_lock_en(&self) -> SPI0_LOCK_EN_R {
        SPI0_LOCK_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn flash_pesr_cmd_2b(&self) -> FLASH_PESR_CMD_2B_R {
        FLASH_PESR_CMD_2B_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn flash_per_command(&self) -> FLASH_PER_COMMAND_R {
        FLASH_PER_COMMAND_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SUS_STATUS")
            .field("flash_sus", &self.flash_sus())
            .field("wait_pesr_cmd_2b", &self.wait_pesr_cmd_2b())
            .field("flash_hpm_dly_128", &self.flash_hpm_dly_128())
            .field("flash_res_dly_128", &self.flash_res_dly_128())
            .field("flash_dp_dly_128", &self.flash_dp_dly_128())
            .field("flash_per_dly_128", &self.flash_per_dly_128())
            .field("flash_pes_dly_128", &self.flash_pes_dly_128())
            .field("spi0_lock_en", &self.spi0_lock_en())
            .field("flash_pesr_cmd_2b", &self.flash_pesr_cmd_2b())
            .field("flash_per_command", &self.flash_per_command())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn flash_sus(&mut self) -> FLASH_SUS_W<'_, SUS_STATUS_SPEC> {
        FLASH_SUS_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn wait_pesr_cmd_2b(&mut self) -> WAIT_PESR_CMD_2B_W<'_, SUS_STATUS_SPEC> {
        WAIT_PESR_CMD_2B_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn flash_hpm_dly_128(&mut self) -> FLASH_HPM_DLY_128_W<'_, SUS_STATUS_SPEC> {
        FLASH_HPM_DLY_128_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn flash_res_dly_128(&mut self) -> FLASH_RES_DLY_128_W<'_, SUS_STATUS_SPEC> {
        FLASH_RES_DLY_128_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn flash_dp_dly_128(&mut self) -> FLASH_DP_DLY_128_W<'_, SUS_STATUS_SPEC> {
        FLASH_DP_DLY_128_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn flash_per_dly_128(&mut self) -> FLASH_PER_DLY_128_W<'_, SUS_STATUS_SPEC> {
        FLASH_PER_DLY_128_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn flash_pes_dly_128(&mut self) -> FLASH_PES_DLY_128_W<'_, SUS_STATUS_SPEC> {
        FLASH_PES_DLY_128_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn spi0_lock_en(&mut self) -> SPI0_LOCK_EN_W<'_, SUS_STATUS_SPEC> {
        SPI0_LOCK_EN_W::new(self, 7)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn flash_pesr_cmd_2b(&mut self) -> FLASH_PESR_CMD_2B_W<'_, SUS_STATUS_SPEC> {
        FLASH_PESR_CMD_2B_W::new(self, 15)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn flash_per_command(&mut self) -> FLASH_PER_COMMAND_W<'_, SUS_STATUS_SPEC> {
        FLASH_PER_COMMAND_W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sus_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sus_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SUS_STATUS_SPEC;
impl crate::RegisterSpec for SUS_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sus_status::R`](R) reader structure"]
impl crate::Readable for SUS_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sus_status::W`](W) writer structure"]
impl crate::Writable for SUS_STATUS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SUS_STATUS to value 0"]
impl crate::Resettable for SUS_STATUS_SPEC {}
