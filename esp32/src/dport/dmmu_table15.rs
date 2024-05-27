#[doc = "Register `DMMU_TABLE15` reader"]
pub type R = crate::R<DMMU_TABLE15_SPEC>;
#[doc = "Register `DMMU_TABLE15` writer"]
pub type W = crate::W<DMMU_TABLE15_SPEC>;
#[doc = "Field `DMMU_TABLE15` reader - "]
pub type DMMU_TABLE15_R = crate::FieldReader;
#[doc = "Field `DMMU_TABLE15` writer - "]
pub type DMMU_TABLE15_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn dmmu_table15(&self) -> DMMU_TABLE15_R {
        DMMU_TABLE15_R::new((self.bits & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMMU_TABLE15")
            .field("dmmu_table15", &self.dmmu_table15())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    #[must_use]
    pub fn dmmu_table15(&mut self) -> DMMU_TABLE15_W<DMMU_TABLE15_SPEC> {
        DMMU_TABLE15_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmmu_table15::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmmu_table15::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMMU_TABLE15_SPEC;
impl crate::RegisterSpec for DMMU_TABLE15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmmu_table15::R`](R) reader structure"]
impl crate::Readable for DMMU_TABLE15_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmmu_table15::W`](W) writer structure"]
impl crate::Writable for DMMU_TABLE15_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMMU_TABLE15 to value 0x0f"]
impl crate::Resettable for DMMU_TABLE15_SPEC {
    const RESET_VALUE: u32 = 0x0f;
}
