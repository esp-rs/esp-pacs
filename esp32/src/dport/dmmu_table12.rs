#[doc = "Register `DMMU_TABLE12` reader"]
pub type R = crate::R<DMMU_TABLE12_SPEC>;
#[doc = "Register `DMMU_TABLE12` writer"]
pub type W = crate::W<DMMU_TABLE12_SPEC>;
#[doc = "Field `DMMU_TABLE12` reader - "]
pub type DMMU_TABLE12_R = crate::FieldReader;
#[doc = "Field `DMMU_TABLE12` writer - "]
pub type DMMU_TABLE12_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn dmmu_table12(&self) -> DMMU_TABLE12_R {
        DMMU_TABLE12_R::new((self.bits & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMMU_TABLE12")
            .field("dmmu_table12", &self.dmmu_table12())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    #[must_use]
    pub fn dmmu_table12(&mut self) -> DMMU_TABLE12_W<DMMU_TABLE12_SPEC> {
        DMMU_TABLE12_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dmmu_table12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmmu_table12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMMU_TABLE12_SPEC;
impl crate::RegisterSpec for DMMU_TABLE12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmmu_table12::R`](R) reader structure"]
impl crate::Readable for DMMU_TABLE12_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmmu_table12::W`](W) writer structure"]
impl crate::Writable for DMMU_TABLE12_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMMU_TABLE12 to value 0x0c"]
impl crate::Resettable for DMMU_TABLE12_SPEC {
    const RESET_VALUE: u32 = 0x0c;
}
