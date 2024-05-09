#[doc = "Register `RXDMA_CTRL_STATE` reader"]
pub type R = crate::R<RXDMA_CTRL_STATE_SPEC>;
#[doc = "Register `RXDMA_CTRL_STATE` writer"]
pub type W = crate::W<RXDMA_CTRL_STATE_SPEC>;
#[doc = "Field `RXDMA_WATER_LEVEL` reader - "]
pub type RXDMA_WATER_LEVEL_R = crate::FieldReader;
#[doc = "Field `RXDMA_WATER_LEVEL` writer - "]
pub type RXDMA_WATER_LEVEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RXDMA_STATE` reader - "]
pub type RXDMA_STATE_R = crate::FieldReader;
#[doc = "Field `RXDMA_STATE` writer - "]
pub type RXDMA_STATE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RXDMA_APPEND_LQI_OFFSET` reader - "]
pub type RXDMA_APPEND_LQI_OFFSET_R = crate::BitReader;
#[doc = "Field `RXDMA_APPEND_LQI_OFFSET` writer - "]
pub type RXDMA_APPEND_LQI_OFFSET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDMA_APPEND_FREQ_OFFSET` reader - "]
pub type RXDMA_APPEND_FREQ_OFFSET_R = crate::BitReader;
#[doc = "Field `RXDMA_APPEND_FREQ_OFFSET` writer - "]
pub type RXDMA_APPEND_FREQ_OFFSET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn rxdma_water_level(&self) -> RXDMA_WATER_LEVEL_R {
        RXDMA_WATER_LEVEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn rxdma_state(&self) -> RXDMA_STATE_R {
        RXDMA_STATE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn rxdma_append_lqi_offset(&self) -> RXDMA_APPEND_LQI_OFFSET_R {
        RXDMA_APPEND_LQI_OFFSET_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn rxdma_append_freq_offset(&self) -> RXDMA_APPEND_FREQ_OFFSET_R {
        RXDMA_APPEND_FREQ_OFFSET_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXDMA_CTRL_STATE")
            .field(
                "rxdma_water_level",
                &format_args!("{}", self.rxdma_water_level().bits()),
            )
            .field(
                "rxdma_state",
                &format_args!("{}", self.rxdma_state().bits()),
            )
            .field(
                "rxdma_append_lqi_offset",
                &format_args!("{}", self.rxdma_append_lqi_offset().bit()),
            )
            .field(
                "rxdma_append_freq_offset",
                &format_args!("{}", self.rxdma_append_freq_offset().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RXDMA_CTRL_STATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn rxdma_water_level(&mut self) -> RXDMA_WATER_LEVEL_W<RXDMA_CTRL_STATE_SPEC> {
        RXDMA_WATER_LEVEL_W::new(self, 0)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    #[must_use]
    pub fn rxdma_state(&mut self) -> RXDMA_STATE_W<RXDMA_CTRL_STATE_SPEC> {
        RXDMA_STATE_W::new(self, 16)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn rxdma_append_lqi_offset(&mut self) -> RXDMA_APPEND_LQI_OFFSET_W<RXDMA_CTRL_STATE_SPEC> {
        RXDMA_APPEND_LQI_OFFSET_W::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn rxdma_append_freq_offset(
        &mut self,
    ) -> RXDMA_APPEND_FREQ_OFFSET_W<RXDMA_CTRL_STATE_SPEC> {
        RXDMA_APPEND_FREQ_OFFSET_W::new(self, 25)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdma_ctrl_state::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxdma_ctrl_state::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXDMA_CTRL_STATE_SPEC;
impl crate::RegisterSpec for RXDMA_CTRL_STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdma_ctrl_state::R`](R) reader structure"]
impl crate::Readable for RXDMA_CTRL_STATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxdma_ctrl_state::W`](W) writer structure"]
impl crate::Writable for RXDMA_CTRL_STATE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXDMA_CTRL_STATE to value 0"]
impl crate::Resettable for RXDMA_CTRL_STATE_SPEC {
    const RESET_VALUE: u32 = 0;
}
