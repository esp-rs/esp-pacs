#[doc = "Register `SINTX_TIMER_CFG` reader"]
pub type R = crate::R<SINTX_TIMER_CFG_SPEC>;
#[doc = "Register `SINTX_TIMER_CFG` writer"]
pub type W = crate::W<SINTX_TIMER_CFG_SPEC>;
#[doc = "Field `SINTX_TIMER_EN` reader - 1:enables using dac timer to reduce output frequency 0:no data output"]
pub type SINTX_TIMER_EN_R = crate::BitReader;
#[doc = "Field `SINTX_TIMER_EN` writer - 1:enables using dac timer to reduce output frequency 0:no data output"]
pub type SINTX_TIMER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SINTX_TIMER_TARGET` reader - dac sintx timer wait target"]
pub type SINTX_TIMER_TARGET_R = crate::FieldReader<u32>;
#[doc = "Field `SINTX_TIMER_TARGET` writer - dac sintx timer wait target"]
pub type SINTX_TIMER_TARGET_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - 1:enables using dac timer to reduce output frequency 0:no data output"]
    #[inline(always)]
    pub fn sintx_timer_en(&self) -> SINTX_TIMER_EN_R {
        SINTX_TIMER_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:24 - dac sintx timer wait target"]
    #[inline(always)]
    pub fn sintx_timer_target(&self) -> SINTX_TIMER_TARGET_R {
        SINTX_TIMER_TARGET_R::new((self.bits >> 1) & 0x00ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SINTX_TIMER_CFG")
            .field("sintx_timer_en", &self.sintx_timer_en())
            .field("sintx_timer_target", &self.sintx_timer_target())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - 1:enables using dac timer to reduce output frequency 0:no data output"]
    #[inline(always)]
    pub fn sintx_timer_en(&mut self) -> SINTX_TIMER_EN_W<'_, SINTX_TIMER_CFG_SPEC> {
        SINTX_TIMER_EN_W::new(self, 0)
    }
    #[doc = "Bits 1:24 - dac sintx timer wait target"]
    #[inline(always)]
    pub fn sintx_timer_target(&mut self) -> SINTX_TIMER_TARGET_W<'_, SINTX_TIMER_CFG_SPEC> {
        SINTX_TIMER_TARGET_W::new(self, 1)
    }
}
#[doc = "SINTX path timer register\n\nYou can [`read`](crate::Reg::read) this register and get [`sintx_timer_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sintx_timer_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SINTX_TIMER_CFG_SPEC;
impl crate::RegisterSpec for SINTX_TIMER_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sintx_timer_cfg::R`](R) reader structure"]
impl crate::Readable for SINTX_TIMER_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sintx_timer_cfg::W`](W) writer structure"]
impl crate::Writable for SINTX_TIMER_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SINTX_TIMER_CFG to value 0x04"]
impl crate::Resettable for SINTX_TIMER_CFG_SPEC {
    const RESET_VALUE: u32 = 0x04;
}
