#[doc = "Register `IMMU_TABLE3` reader"]
pub type R = crate::R<IMMU_TABLE3_SPEC>;
#[doc = "Register `IMMU_TABLE3` writer"]
pub type W = crate::W<IMMU_TABLE3_SPEC>;
#[doc = "Field `IMMU_TABLE3` reader - "]
pub type IMMU_TABLE3_R = crate::FieldReader;
#[doc = "Field `IMMU_TABLE3` writer - "]
pub type IMMU_TABLE3_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn immu_table3(&self) -> IMMU_TABLE3_R {
        IMMU_TABLE3_R::new((self.bits & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IMMU_TABLE3")
            .field("immu_table3", &self.immu_table3())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn immu_table3(&mut self) -> IMMU_TABLE3_W<IMMU_TABLE3_SPEC> {
        IMMU_TABLE3_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`immu_table3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`immu_table3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IMMU_TABLE3_SPEC;
impl crate::RegisterSpec for IMMU_TABLE3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`immu_table3::R`](R) reader structure"]
impl crate::Readable for IMMU_TABLE3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`immu_table3::W`](W) writer structure"]
impl crate::Writable for IMMU_TABLE3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMMU_TABLE3 to value 0x03"]
impl crate::Resettable for IMMU_TABLE3_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
