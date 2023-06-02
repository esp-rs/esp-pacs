#[doc = "Register `CPU_PERIPHERAL_INTR` reader"]
pub struct R(crate::R<CPU_PERIPHERAL_INTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_PERIPHERAL_INTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_PERIPHERAL_INTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_PERIPHERAL_INTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPU_PERIPHERAL_INTR` writer"]
pub struct W(crate::W<CPU_PERIPHERAL_INTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU_PERIPHERAL_INTR_SPEC>;
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
impl From<crate::W<CPU_PERIPHERAL_INTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU_PERIPHERAL_INTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPU_PERI_BYTE_ERROR_CLR` reader - The clear signal for CPU peripheral access interrupt."]
pub type CPU_PERI_BYTE_ERROR_CLR_R = crate::BitReader;
#[doc = "Field `CPU_PERI_BYTE_ERROR_CLR` writer - The clear signal for CPU peripheral access interrupt."]
pub type CPU_PERI_BYTE_ERROR_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, CPU_PERIPHERAL_INTR_SPEC, O>;
#[doc = "Field `CPU_PERI_BYTE_ERROR_EN` reader - The enable signal for CPU peripheral access interrupt."]
pub type CPU_PERI_BYTE_ERROR_EN_R = crate::BitReader;
#[doc = "Field `CPU_PERI_BYTE_ERROR_EN` writer - The enable signal for CPU peripheral access interrupt."]
pub type CPU_PERI_BYTE_ERROR_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, CPU_PERIPHERAL_INTR_SPEC, O>;
#[doc = "Field `CPU_PERI_BYTE_ERROR_INTR` reader - CPU peripheral access interrupt signal."]
pub type CPU_PERI_BYTE_ERROR_INTR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The clear signal for CPU peripheral access interrupt."]
    #[inline(always)]
    pub fn cpu_peri_byte_error_clr(&self) -> CPU_PERI_BYTE_ERROR_CLR_R {
        CPU_PERI_BYTE_ERROR_CLR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The enable signal for CPU peripheral access interrupt."]
    #[inline(always)]
    pub fn cpu_peri_byte_error_en(&self) -> CPU_PERI_BYTE_ERROR_EN_R {
        CPU_PERI_BYTE_ERROR_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CPU peripheral access interrupt signal."]
    #[inline(always)]
    pub fn cpu_peri_byte_error_intr(&self) -> CPU_PERI_BYTE_ERROR_INTR_R {
        CPU_PERI_BYTE_ERROR_INTR_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU_PERIPHERAL_INTR")
            .field(
                "cpu_peri_byte_error_clr",
                &format_args!("{}", self.cpu_peri_byte_error_clr().bit()),
            )
            .field(
                "cpu_peri_byte_error_en",
                &format_args!("{}", self.cpu_peri_byte_error_en().bit()),
            )
            .field(
                "cpu_peri_byte_error_intr",
                &format_args!("{}", self.cpu_peri_byte_error_intr().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CPU_PERIPHERAL_INTR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - The clear signal for CPU peripheral access interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cpu_peri_byte_error_clr(&mut self) -> CPU_PERI_BYTE_ERROR_CLR_W<0> {
        CPU_PERI_BYTE_ERROR_CLR_W::new(self)
    }
    #[doc = "Bit 1 - The enable signal for CPU peripheral access interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cpu_peri_byte_error_en(&mut self) -> CPU_PERI_BYTE_ERROR_EN_W<1> {
        CPU_PERI_BYTE_ERROR_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PeribBus1 permission control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_peripheral_intr](index.html) module"]
pub struct CPU_PERIPHERAL_INTR_SPEC;
impl crate::RegisterSpec for CPU_PERIPHERAL_INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_peripheral_intr::R](R) reader structure"]
impl crate::Readable for CPU_PERIPHERAL_INTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu_peripheral_intr::W](W) writer structure"]
impl crate::Writable for CPU_PERIPHERAL_INTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPU_PERIPHERAL_INTR to value 0"]
impl crate::Resettable for CPU_PERIPHERAL_INTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
