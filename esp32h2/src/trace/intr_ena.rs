#[doc = "Register `INTR_ENA` reader"]
pub struct R(crate::R<INTR_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_ENA` writer"]
pub struct W(crate::W<INTR_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_ENA_SPEC>;
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
impl From<crate::W<INTR_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFO_OVERFLOW_INTR_ENA` reader - Set 1 enable fifo_overflow interrupt"]
pub type FIFO_OVERFLOW_INTR_ENA_R = crate::BitReader;
#[doc = "Field `FIFO_OVERFLOW_INTR_ENA` writer - Set 1 enable fifo_overflow interrupt"]
pub type FIFO_OVERFLOW_INTR_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INTR_ENA_SPEC, O>;
#[doc = "Field `MEM_FULL_INTR_ENA` reader - Set 1 enable mem_full interrupt"]
pub type MEM_FULL_INTR_ENA_R = crate::BitReader;
#[doc = "Field `MEM_FULL_INTR_ENA` writer - Set 1 enable mem_full interrupt"]
pub type MEM_FULL_INTR_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INTR_ENA_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Set 1 enable fifo_overflow interrupt"]
    #[inline(always)]
    pub fn fifo_overflow_intr_ena(&self) -> FIFO_OVERFLOW_INTR_ENA_R {
        FIFO_OVERFLOW_INTR_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 1 enable mem_full interrupt"]
    #[inline(always)]
    pub fn mem_full_intr_ena(&self) -> MEM_FULL_INTR_ENA_R {
        MEM_FULL_INTR_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTR_ENA")
            .field(
                "fifo_overflow_intr_ena",
                &format_args!("{}", self.fifo_overflow_intr_ena().bit()),
            )
            .field(
                "mem_full_intr_ena",
                &format_args!("{}", self.mem_full_intr_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INTR_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 enable fifo_overflow interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_overflow_intr_ena(&mut self) -> FIFO_OVERFLOW_INTR_ENA_W<0> {
        FIFO_OVERFLOW_INTR_ENA_W::new(self)
    }
    #[doc = "Bit 1 - Set 1 enable mem_full interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn mem_full_intr_ena(&mut self) -> MEM_FULL_INTR_ENA_W<1> {
        MEM_FULL_INTR_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_ena](index.html) module"]
pub struct INTR_ENA_SPEC;
impl crate::RegisterSpec for INTR_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_ena::R](R) reader structure"]
impl crate::Readable for INTR_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_ena::W](W) writer structure"]
impl crate::Writable for INTR_ENA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR_ENA to value 0"]
impl crate::Resettable for INTR_ENA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
