#[doc = "Register `DT0_FED_CFG` reader"]
pub type R = crate::R<DT0_FED_CFG_SPEC>;
#[doc = "Register `DT0_FED_CFG` writer"]
pub type W = crate::W<DT0_FED_CFG_SPEC>;
#[doc = "Field `DB0_FED` reader - Shadow register for FED"]
pub type DB0_FED_R = crate::FieldReader<u16>;
#[doc = "Field `DB0_FED` writer - Shadow register for FED"]
pub type DB0_FED_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Shadow register for FED"]
    #[inline(always)]
    pub fn db0_fed(&self) -> DB0_FED_R {
        DB0_FED_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DT0_FED_CFG")
            .field("db0_fed", &self.db0_fed())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Shadow register for FED"]
    #[inline(always)]
    pub fn db0_fed(&mut self) -> DB0_FED_W<'_, DT0_FED_CFG_SPEC> {
        DB0_FED_W::new(self, 0)
    }
}
#[doc = "Shadow register for falling edge delay (FED).\n\nYou can [`read`](crate::Reg::read) this register and get [`dt0_fed_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dt0_fed_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DT0_FED_CFG_SPEC;
impl crate::RegisterSpec for DT0_FED_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt0_fed_cfg::R`](R) reader structure"]
impl crate::Readable for DT0_FED_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dt0_fed_cfg::W`](W) writer structure"]
impl crate::Writable for DT0_FED_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DT0_FED_CFG to value 0"]
impl crate::Resettable for DT0_FED_CFG_SPEC {}
