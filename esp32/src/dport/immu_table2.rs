#[doc = "Register `IMMU_TABLE2` reader"]
pub type R = crate::R<IMMU_TABLE2_SPEC>;
#[doc = "Register `IMMU_TABLE2` writer"]
pub type W = crate::W<IMMU_TABLE2_SPEC>;
#[doc = "Field `IMMU_TABLE2` reader - "]
pub type IMMU_TABLE2_R = crate::FieldReader;
#[doc = "Field `IMMU_TABLE2` writer - "]
pub type IMMU_TABLE2_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn immu_table2(&self) -> IMMU_TABLE2_R {
        IMMU_TABLE2_R::new((self.bits & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IMMU_TABLE2")
            .field(
                "immu_table2",
                &format_args!("{}", self.immu_table2().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IMMU_TABLE2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    #[must_use]
    pub fn immu_table2(&mut self) -> IMMU_TABLE2_W<IMMU_TABLE2_SPEC> {
        IMMU_TABLE2_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`immu_table2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`immu_table2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IMMU_TABLE2_SPEC;
impl crate::RegisterSpec for IMMU_TABLE2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`immu_table2::R`](R) reader structure"]
impl crate::Readable for IMMU_TABLE2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`immu_table2::W`](W) writer structure"]
impl crate::Writable for IMMU_TABLE2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMMU_TABLE2 to value 0x02"]
impl crate::Resettable for IMMU_TABLE2_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
