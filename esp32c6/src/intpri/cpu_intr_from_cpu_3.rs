#[doc = "Register `CPU_INTR_FROM_CPU_3` reader"]
pub struct R(crate::R<CPU_INTR_FROM_CPU_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_INTR_FROM_CPU_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_INTR_FROM_CPU_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_INTR_FROM_CPU_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPU_INTR_FROM_CPU_3` writer"]
pub struct W(crate::W<CPU_INTR_FROM_CPU_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU_INTR_FROM_CPU_3_SPEC>;
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
impl From<crate::W<CPU_INTR_FROM_CPU_3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU_INTR_FROM_CPU_3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPU_INTR_FROM_CPU_3` reader - Need add description"]
pub type CPU_INTR_FROM_CPU_3_R = crate::BitReader<bool>;
#[doc = "Field `CPU_INTR_FROM_CPU_3` writer - Need add description"]
pub type CPU_INTR_FROM_CPU_3_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CPU_INTR_FROM_CPU_3_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Need add description"]
    #[inline(always)]
    pub fn cpu_intr_from_cpu_3(&self) -> CPU_INTR_FROM_CPU_3_R {
        CPU_INTR_FROM_CPU_3_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Need add description"]
    #[inline(always)]
    #[must_use]
    pub fn cpu_intr_from_cpu_3(&mut self) -> CPU_INTR_FROM_CPU_3_W<0> {
        CPU_INTR_FROM_CPU_3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_intr_from_cpu_3](index.html) module"]
pub struct CPU_INTR_FROM_CPU_3_SPEC;
impl crate::RegisterSpec for CPU_INTR_FROM_CPU_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_intr_from_cpu_3::R](R) reader structure"]
impl crate::Readable for CPU_INTR_FROM_CPU_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu_intr_from_cpu_3::W](W) writer structure"]
impl crate::Writable for CPU_INTR_FROM_CPU_3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPU_INTR_FROM_CPU_3 to value 0"]
impl crate::Resettable for CPU_INTR_FROM_CPU_3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}