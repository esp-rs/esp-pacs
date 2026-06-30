#[doc = "Register `PDMA_TIMER_CFG` reader"]
pub type R = crate::R<PDMA_TIMER_CFG_SPEC>;
#[doc = "Register `PDMA_TIMER_CFG` writer"]
pub type W = crate::W<PDMA_TIMER_CFG_SPEC>;
#[doc = "Field `PDMA_TIMER_EN` reader - 1:enables using dac timer to reduce output frequency 0:no data output"]
pub type PDMA_TIMER_EN_R = crate::BitReader;
#[doc = "Field `PDMA_TIMER_EN` writer - 1:enables using dac timer to reduce output frequency 0:no data output"]
pub type PDMA_TIMER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_TIMER_TARGET` reader - dac pdma timer wait target"]
pub type PDMA_TIMER_TARGET_R = crate::FieldReader<u32>;
#[doc = "Field `PDMA_TIMER_TARGET` writer - dac pdma timer wait target"]
pub type PDMA_TIMER_TARGET_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - 1:enables using dac timer to reduce output frequency 0:no data output"]
    #[inline(always)]
    pub fn pdma_timer_en(&self) -> PDMA_TIMER_EN_R {
        PDMA_TIMER_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:24 - dac pdma timer wait target"]
    #[inline(always)]
    pub fn pdma_timer_target(&self) -> PDMA_TIMER_TARGET_R {
        PDMA_TIMER_TARGET_R::new((self.bits >> 1) & 0x00ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDMA_TIMER_CFG")
            .field("pdma_timer_en", &self.pdma_timer_en())
            .field("pdma_timer_target", &self.pdma_timer_target())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - 1:enables using dac timer to reduce output frequency 0:no data output"]
    #[inline(always)]
    pub fn pdma_timer_en(&mut self) -> PDMA_TIMER_EN_W<'_, PDMA_TIMER_CFG_SPEC> {
        PDMA_TIMER_EN_W::new(self, 0)
    }
    #[doc = "Bits 1:24 - dac pdma timer wait target"]
    #[inline(always)]
    pub fn pdma_timer_target(&mut self) -> PDMA_TIMER_TARGET_W<'_, PDMA_TIMER_CFG_SPEC> {
        PDMA_TIMER_TARGET_W::new(self, 1)
    }
}
#[doc = "PDMA path timer register\n\nYou can [`read`](crate::Reg::read) this register and get [`pdma_timer_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdma_timer_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDMA_TIMER_CFG_SPEC;
impl crate::RegisterSpec for PDMA_TIMER_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdma_timer_cfg::R`](R) reader structure"]
impl crate::Readable for PDMA_TIMER_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pdma_timer_cfg::W`](W) writer structure"]
impl crate::Writable for PDMA_TIMER_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PDMA_TIMER_CFG to value 0x04"]
impl crate::Resettable for PDMA_TIMER_CFG_SPEC {
    const RESET_VALUE: u32 = 0x04;
}
