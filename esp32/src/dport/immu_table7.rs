#[doc = "Register `IMMU_TABLE7` reader"]
pub type R = crate::R<IMMU_TABLE7_SPEC>;
#[doc = "Register `IMMU_TABLE7` writer"]
pub type W = crate::W<IMMU_TABLE7_SPEC>;
#[doc = "Field `IMMU_TABLE7` reader - "]
pub type IMMU_TABLE7_R = crate::FieldReader;
#[doc = "Field `IMMU_TABLE7` writer - "]
pub type IMMU_TABLE7_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn immu_table7(&self) -> IMMU_TABLE7_R {
        IMMU_TABLE7_R::new((self.bits & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IMMU_TABLE7")
            .field("immu_table7", &self.immu_table7())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    #[must_use]
    pub fn immu_table7(&mut self) -> IMMU_TABLE7_W<IMMU_TABLE7_SPEC> {
        IMMU_TABLE7_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`immu_table7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`immu_table7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IMMU_TABLE7_SPEC;
impl crate::RegisterSpec for IMMU_TABLE7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`immu_table7::R`](R) reader structure"]
impl crate::Readable for IMMU_TABLE7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`immu_table7::W`](W) writer structure"]
impl crate::Writable for IMMU_TABLE7_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMMU_TABLE7 to value 0x07"]
impl crate::Resettable for IMMU_TABLE7_SPEC {
    const RESET_VALUE: u32 = 0x07;
}
