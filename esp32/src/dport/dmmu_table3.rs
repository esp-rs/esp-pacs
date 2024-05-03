#[doc = "Register `DMMU_TABLE3` reader"]
pub type R = crate::R<DMMU_TABLE3_SPEC>;
#[doc = "Register `DMMU_TABLE3` writer"]
pub type W = crate::W<DMMU_TABLE3_SPEC>;
#[doc = "Field `DMMU_TABLE3` reader - "]
pub type DMMU_TABLE3_R = crate::FieldReader;
#[doc = "Field `DMMU_TABLE3` writer - "]
pub type DMMU_TABLE3_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn dmmu_table3(&self) -> DMMU_TABLE3_R {
        DMMU_TABLE3_R::new((self.bits & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMMU_TABLE3")
            .field("dmmu_table3", &self.dmmu_table3().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMMU_TABLE3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    #[must_use]
    pub fn dmmu_table3(&mut self) -> DMMU_TABLE3_W<DMMU_TABLE3_SPEC> {
        DMMU_TABLE3_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmmu_table3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmmu_table3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMMU_TABLE3_SPEC;
impl crate::RegisterSpec for DMMU_TABLE3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmmu_table3::R`](R) reader structure"]
impl crate::Readable for DMMU_TABLE3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmmu_table3::W`](W) writer structure"]
impl crate::Writable for DMMU_TABLE3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMMU_TABLE3 to value 0x03"]
impl crate::Resettable for DMMU_TABLE3_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
