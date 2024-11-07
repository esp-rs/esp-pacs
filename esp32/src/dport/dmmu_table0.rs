#[doc = "Register `DMMU_TABLE0` reader"]
pub type R = crate::R<DMMU_TABLE0_SPEC>;
#[doc = "Register `DMMU_TABLE0` writer"]
pub type W = crate::W<DMMU_TABLE0_SPEC>;
#[doc = "Field `DMMU_TABLE0` reader - "]
pub type DMMU_TABLE0_R = crate::FieldReader;
#[doc = "Field `DMMU_TABLE0` writer - "]
pub type DMMU_TABLE0_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn dmmu_table0(&self) -> DMMU_TABLE0_R {
        DMMU_TABLE0_R::new((self.bits & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMMU_TABLE0")
            .field("dmmu_table0", &self.dmmu_table0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn dmmu_table0(&mut self) -> DMMU_TABLE0_W<DMMU_TABLE0_SPEC> {
        DMMU_TABLE0_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dmmu_table0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmmu_table0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMMU_TABLE0_SPEC;
impl crate::RegisterSpec for DMMU_TABLE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmmu_table0::R`](R) reader structure"]
impl crate::Readable for DMMU_TABLE0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmmu_table0::W`](W) writer structure"]
impl crate::Writable for DMMU_TABLE0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMMU_TABLE0 to value 0"]
impl crate::Resettable for DMMU_TABLE0_SPEC {
    const RESET_VALUE: u32 = 0;
}
