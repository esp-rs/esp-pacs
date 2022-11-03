#[doc = "Register `CORE0_CPU_INT_PRI_30` reader"]
pub struct R(crate::R<CORE0_CPU_INT_PRI_30_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE0_CPU_INT_PRI_30_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE0_CPU_INT_PRI_30_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE0_CPU_INT_PRI_30_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE0_CPU_INT_PRI_30` writer"]
pub struct W(crate::W<CORE0_CPU_INT_PRI_30_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE0_CPU_INT_PRI_30_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CORE0_CPU_INT_PRI_30_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE0_CPU_INT_PRI_30_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE0_CPU_PRI_30_MAP` reader - Need add description"]
pub type CORE0_CPU_PRI_30_MAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE0_CPU_PRI_30_MAP` writer - Need add description"]
pub type CORE0_CPU_PRI_30_MAP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE0_CPU_INT_PRI_30_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Need add description"]
    #[inline(always)]
    pub fn core0_cpu_pri_30_map(&self) -> CORE0_CPU_PRI_30_MAP_R {
        CORE0_CPU_PRI_30_MAP_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Need add description"]
    #[inline(always)]
    #[must_use]
    pub fn core0_cpu_pri_30_map(&mut self) -> CORE0_CPU_PRI_30_MAP_W<0> {
        CORE0_CPU_PRI_30_MAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core0_cpu_int_pri_30](index.html) module"]
pub struct CORE0_CPU_INT_PRI_30_SPEC;
impl crate::RegisterSpec for CORE0_CPU_INT_PRI_30_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core0_cpu_int_pri_30::R](R) reader structure"]
impl crate::Readable for CORE0_CPU_INT_PRI_30_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core0_cpu_int_pri_30::W](W) writer structure"]
impl crate::Writable for CORE0_CPU_INT_PRI_30_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CORE0_CPU_INT_PRI_30 to value 0"]
impl crate::Resettable for CORE0_CPU_INT_PRI_30_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
