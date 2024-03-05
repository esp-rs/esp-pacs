#[doc = "Register `DMMU_TABLE6` reader"]
pub type R = crate::R<DMMU_TABLE6_SPEC>;
#[doc = "Register `DMMU_TABLE6` writer"]
pub type W = crate::W<DMMU_TABLE6_SPEC>;
#[doc = "Field `DMMU_TABLE6` reader - "]
pub type DMMU_TABLE6_R = crate::FieldReader;
#[doc = "Field `DMMU_TABLE6` writer - "]
pub type DMMU_TABLE6_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn dmmu_table6(&self) -> DMMU_TABLE6_R {
        DMMU_TABLE6_R::new((self.bits & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMMU_TABLE6")
            .field(
                "dmmu_table6",
                &format_args!("{}", self.dmmu_table6().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMMU_TABLE6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    #[must_use]
    pub fn dmmu_table6(&mut self) -> DMMU_TABLE6_W<DMMU_TABLE6_SPEC> {
        DMMU_TABLE6_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmmu_table6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmmu_table6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMMU_TABLE6_SPEC;
impl crate::RegisterSpec for DMMU_TABLE6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmmu_table6::R`](R) reader structure"]
impl crate::Readable for DMMU_TABLE6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmmu_table6::W`](W) writer structure"]
impl crate::Writable for DMMU_TABLE6_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMMU_TABLE6 to value 0x06"]
impl crate::Resettable for DMMU_TABLE6_SPEC {
    const RESET_VALUE: u32 = 0x06;
}
