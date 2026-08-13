#[doc = "Register `CHNL0_CFG4` reader"]
pub type R = crate::R<CHNL0_CFG4_SPEC>;
#[doc = "Register `CHNL0_CFG4` writer"]
pub type W = crate::W<CHNL0_CFG4_SPEC>;
#[doc = "Field `CHNL0_START` reader - Set this bit to start channel0."]
pub type CHNL0_START_R = crate::BitReader;
#[doc = "Field `CHNL0_START` writer - Set this bit to start channel0."]
pub type CHNL0_START_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to start channel0."]
    #[inline(always)]
    pub fn chnl0_start(&self) -> CHNL0_START_R {
        CHNL0_START_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHNL0_CFG4")
            .field("chnl0_start", &self.chnl0_start())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to start channel0."]
    #[inline(always)]
    pub fn chnl0_start(&mut self) -> CHNL0_START_W<'_, CHNL0_CFG4_SPEC> {
        CHNL0_START_W::new(self, 0)
    }
}
#[doc = "Control and configuration registers\n\nYou can [`read`](crate::Reg::read) this register and get [`chnl0_cfg4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnl0_cfg4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHNL0_CFG4_SPEC;
impl crate::RegisterSpec for CHNL0_CFG4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chnl0_cfg4::R`](R) reader structure"]
impl crate::Readable for CHNL0_CFG4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chnl0_cfg4::W`](W) writer structure"]
impl crate::Writable for CHNL0_CFG4_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHNL0_CFG4 to value 0"]
impl crate::Resettable for CHNL0_CFG4_SPEC {}
