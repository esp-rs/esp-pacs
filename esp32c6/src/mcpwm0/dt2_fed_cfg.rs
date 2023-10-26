#[doc = "Register `DT2_FED_CFG` reader"]
pub type R = crate::R<DT2_FED_CFG_SPEC>;
#[doc = "Register `DT2_FED_CFG` writer"]
pub type W = crate::W<DT2_FED_CFG_SPEC>;
#[doc = "Field `DB2_FED` reader - Shadow register for FED"]
pub type DB2_FED_R = crate::FieldReader<u16>;
#[doc = "Field `DB2_FED` writer - Shadow register for FED"]
pub type DB2_FED_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Shadow register for FED"]
    #[inline(always)]
    pub fn db2_fed(&self) -> DB2_FED_R {
        DB2_FED_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DT2_FED_CFG")
            .field("db2_fed", &format_args!("{}", self.db2_fed().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DT2_FED_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - Shadow register for FED"]
    #[inline(always)]
    #[must_use]
    pub fn db2_fed(&mut self) -> DB2_FED_W<DT2_FED_CFG_SPEC, 0> {
        DB2_FED_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Shadow register for falling edge delay (FED).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt2_fed_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt2_fed_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DT2_FED_CFG_SPEC;
impl crate::RegisterSpec for DT2_FED_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt2_fed_cfg::R`](R) reader structure"]
impl crate::Readable for DT2_FED_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dt2_fed_cfg::W`](W) writer structure"]
impl crate::Writable for DT2_FED_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DT2_FED_CFG to value 0"]
impl crate::Resettable for DT2_FED_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
