#[doc = "Register `IMMU_TABLE1` reader"]
pub type R = crate::R<IMMU_TABLE1_SPEC>;
#[doc = "Register `IMMU_TABLE1` writer"]
pub type W = crate::W<IMMU_TABLE1_SPEC>;
#[doc = "Field `IMMU_TABLE1` reader - "]
pub type IMMU_TABLE1_R = crate::FieldReader;
#[doc = "Field `IMMU_TABLE1` writer - "]
pub type IMMU_TABLE1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn immu_table1(&self) -> IMMU_TABLE1_R {
        IMMU_TABLE1_R::new((self.bits & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IMMU_TABLE1")
            .field(
                "immu_table1",
                &format_args!("{}", self.immu_table1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IMMU_TABLE1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    #[must_use]
    pub fn immu_table1(&mut self) -> IMMU_TABLE1_W<IMMU_TABLE1_SPEC, 0> {
        IMMU_TABLE1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`immu_table1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`immu_table1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IMMU_TABLE1_SPEC;
impl crate::RegisterSpec for IMMU_TABLE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`immu_table1::R`](R) reader structure"]
impl crate::Readable for IMMU_TABLE1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`immu_table1::W`](W) writer structure"]
impl crate::Writable for IMMU_TABLE1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IMMU_TABLE1 to value 0x01"]
impl crate::Resettable for IMMU_TABLE1_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
