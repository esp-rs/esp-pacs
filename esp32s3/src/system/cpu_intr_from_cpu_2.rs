#[doc = "Register `CPU_INTR_FROM_CPU_2` reader"]
pub struct R(crate::R<CPU_INTR_FROM_CPU_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_INTR_FROM_CPU_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_INTR_FROM_CPU_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_INTR_FROM_CPU_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPU_INTR_FROM_CPU_2` writer"]
pub struct W(crate::W<CPU_INTR_FROM_CPU_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU_INTR_FROM_CPU_2_SPEC>;
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
impl From<crate::W<CPU_INTR_FROM_CPU_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU_INTR_FROM_CPU_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPU_INTR_FROM_CPU_2` reader - Set 1 to generate cpu interrupt 2"]
pub type CPU_INTR_FROM_CPU_2_R = crate::BitReader;
#[doc = "Field `CPU_INTR_FROM_CPU_2` writer - Set 1 to generate cpu interrupt 2"]
pub type CPU_INTR_FROM_CPU_2_W<'a, const O: u8> = crate::BitWriter<'a, CPU_INTR_FROM_CPU_2_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Set 1 to generate cpu interrupt 2"]
    #[inline(always)]
    pub fn cpu_intr_from_cpu_2(&self) -> CPU_INTR_FROM_CPU_2_R {
        CPU_INTR_FROM_CPU_2_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU_INTR_FROM_CPU_2")
            .field(
                "cpu_intr_from_cpu_2",
                &format_args!("{}", self.cpu_intr_from_cpu_2().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CPU_INTR_FROM_CPU_2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to generate cpu interrupt 2"]
    #[inline(always)]
    #[must_use]
    pub fn cpu_intr_from_cpu_2(&mut self) -> CPU_INTR_FROM_CPU_2_W<0> {
        CPU_INTR_FROM_CPU_2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "interrupt source register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_intr_from_cpu_2](index.html) module"]
pub struct CPU_INTR_FROM_CPU_2_SPEC;
impl crate::RegisterSpec for CPU_INTR_FROM_CPU_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_intr_from_cpu_2::R](R) reader structure"]
impl crate::Readable for CPU_INTR_FROM_CPU_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu_intr_from_cpu_2::W](W) writer structure"]
impl crate::Writable for CPU_INTR_FROM_CPU_2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPU_INTR_FROM_CPU_2 to value 0"]
impl crate::Resettable for CPU_INTR_FROM_CPU_2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
