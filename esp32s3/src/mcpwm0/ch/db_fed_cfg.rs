#[doc = "Register `DB_FED_CFG` reader"]
pub type R = crate::R<DB_FED_CFG_SPEC>;
#[doc = "Register `DB_FED_CFG` writer"]
pub type W = crate::W<DB_FED_CFG_SPEC>;
#[doc = "Field `FED` reader - Shadow register for FED"]
pub type FED_R = crate::FieldReader<u16>;
#[doc = "Field `FED` writer - Shadow register for FED"]
pub type FED_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Shadow register for FED"]
    #[inline(always)]
    pub fn fed(&self) -> FED_R {
        FED_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DB_FED_CFG")
            .field("fed", &self.fed())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Shadow register for FED"]
    #[inline(always)]
    #[must_use]
    pub fn fed(&mut self) -> FED_W<DB_FED_CFG_SPEC> {
        FED_W::new(self, 0)
    }
}
#[doc = "Shadow register for falling edge delay (FED).\n\nYou can [`read`](crate::Reg::read) this register and get [`db_fed_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`db_fed_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DB_FED_CFG_SPEC;
impl crate::RegisterSpec for DB_FED_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`db_fed_cfg::R`](R) reader structure"]
impl crate::Readable for DB_FED_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`db_fed_cfg::W`](W) writer structure"]
impl crate::Writable for DB_FED_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DB_FED_CFG to value 0"]
impl crate::Resettable for DB_FED_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
