#[doc = "Register `DT1_FED_CFG` reader"]
pub type R = crate::R<DT1_FED_CFG_SPEC>;
#[doc = "Register `DT1_FED_CFG` writer"]
pub type W = crate::W<DT1_FED_CFG_SPEC>;
#[doc = "Field `DT1_FED` reader - "]
pub type DT1_FED_R = crate::FieldReader<u16>;
#[doc = "Field `DT1_FED` writer - "]
pub type DT1_FED_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn dt1_fed(&self) -> DT1_FED_R {
        DT1_FED_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DT1_FED_CFG")
            .field("dt1_fed", &format_args!("{}", self.dt1_fed().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DT1_FED_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn dt1_fed(&mut self) -> DT1_FED_W<DT1_FED_CFG_SPEC> {
        DT1_FED_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt1_fed_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt1_fed_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DT1_FED_CFG_SPEC;
impl crate::RegisterSpec for DT1_FED_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt1_fed_cfg::R`](R) reader structure"]
impl crate::Readable for DT1_FED_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dt1_fed_cfg::W`](W) writer structure"]
impl crate::Writable for DT1_FED_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DT1_FED_CFG to value 0"]
impl crate::Resettable for DT1_FED_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
