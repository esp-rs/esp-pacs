#[doc = "Register `INTR_RLS` reader"]
pub struct R(crate::R<INTR_RLS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_RLS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_RLS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_RLS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_RLS` writer"]
pub struct W(crate::W<INTR_RLS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_RLS_SPEC>;
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
impl From<crate::W<INTR_RLS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_RLS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO0_INT_ENA` reader - The enable bit for DEDIC_GPIO0_INT_ST register."]
pub type GPIO0_INT_ENA_R = crate::BitReader;
#[doc = "Field `GPIO0_INT_ENA` writer - The enable bit for DEDIC_GPIO0_INT_ST register."]
pub type GPIO0_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INTR_RLS_SPEC, O>;
#[doc = "Field `GPIO1_INT_ENA` reader - The enable bit for DEDIC_GPIO1_INT_ST register."]
pub type GPIO1_INT_ENA_R = crate::BitReader;
#[doc = "Field `GPIO1_INT_ENA` writer - The enable bit for DEDIC_GPIO1_INT_ST register."]
pub type GPIO1_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INTR_RLS_SPEC, O>;
#[doc = "Field `GPIO2_INT_ENA` reader - The enable bit for DEDIC_GPIO2_INT_ST register."]
pub type GPIO2_INT_ENA_R = crate::BitReader;
#[doc = "Field `GPIO2_INT_ENA` writer - The enable bit for DEDIC_GPIO2_INT_ST register."]
pub type GPIO2_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INTR_RLS_SPEC, O>;
#[doc = "Field `GPIO3_INT_ENA` reader - The enable bit for DEDIC_GPIO3_INT_ST register."]
pub type GPIO3_INT_ENA_R = crate::BitReader;
#[doc = "Field `GPIO3_INT_ENA` writer - The enable bit for DEDIC_GPIO3_INT_ST register."]
pub type GPIO3_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INTR_RLS_SPEC, O>;
#[doc = "Field `GPIO4_INT_ENA` reader - The enable bit for DEDIC_GPIO4_INT_ST register."]
pub type GPIO4_INT_ENA_R = crate::BitReader;
#[doc = "Field `GPIO4_INT_ENA` writer - The enable bit for DEDIC_GPIO4_INT_ST register."]
pub type GPIO4_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INTR_RLS_SPEC, O>;
#[doc = "Field `GPIO5_INT_ENA` reader - The enable bit for DEDIC_GPIO5_INT_ST register."]
pub type GPIO5_INT_ENA_R = crate::BitReader;
#[doc = "Field `GPIO5_INT_ENA` writer - The enable bit for DEDIC_GPIO5_INT_ST register."]
pub type GPIO5_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INTR_RLS_SPEC, O>;
#[doc = "Field `GPIO6_INT_ENA` reader - The enable bit for DEDIC_GPIO6_INT_ST register."]
pub type GPIO6_INT_ENA_R = crate::BitReader;
#[doc = "Field `GPIO6_INT_ENA` writer - The enable bit for DEDIC_GPIO6_INT_ST register."]
pub type GPIO6_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INTR_RLS_SPEC, O>;
#[doc = "Field `GPIO7_INT_ENA` reader - The enable bit for DEDIC_GPIO7_INT_ST register."]
pub type GPIO7_INT_ENA_R = crate::BitReader;
#[doc = "Field `GPIO7_INT_ENA` writer - The enable bit for DEDIC_GPIO7_INT_ST register."]
pub type GPIO7_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INTR_RLS_SPEC, O>;
impl R {
    #[doc = "Bit 0 - The enable bit for DEDIC_GPIO0_INT_ST register."]
    #[inline(always)]
    pub fn gpio0_int_ena(&self) -> GPIO0_INT_ENA_R {
        GPIO0_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The enable bit for DEDIC_GPIO1_INT_ST register."]
    #[inline(always)]
    pub fn gpio1_int_ena(&self) -> GPIO1_INT_ENA_R {
        GPIO1_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The enable bit for DEDIC_GPIO2_INT_ST register."]
    #[inline(always)]
    pub fn gpio2_int_ena(&self) -> GPIO2_INT_ENA_R {
        GPIO2_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The enable bit for DEDIC_GPIO3_INT_ST register."]
    #[inline(always)]
    pub fn gpio3_int_ena(&self) -> GPIO3_INT_ENA_R {
        GPIO3_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The enable bit for DEDIC_GPIO4_INT_ST register."]
    #[inline(always)]
    pub fn gpio4_int_ena(&self) -> GPIO4_INT_ENA_R {
        GPIO4_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The enable bit for DEDIC_GPIO5_INT_ST register."]
    #[inline(always)]
    pub fn gpio5_int_ena(&self) -> GPIO5_INT_ENA_R {
        GPIO5_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The enable bit for DEDIC_GPIO6_INT_ST register."]
    #[inline(always)]
    pub fn gpio6_int_ena(&self) -> GPIO6_INT_ENA_R {
        GPIO6_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The enable bit for DEDIC_GPIO7_INT_ST register."]
    #[inline(always)]
    pub fn gpio7_int_ena(&self) -> GPIO7_INT_ENA_R {
        GPIO7_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTR_RLS")
            .field(
                "gpio0_int_ena",
                &format_args!("{}", self.gpio0_int_ena().bit()),
            )
            .field(
                "gpio1_int_ena",
                &format_args!("{}", self.gpio1_int_ena().bit()),
            )
            .field(
                "gpio2_int_ena",
                &format_args!("{}", self.gpio2_int_ena().bit()),
            )
            .field(
                "gpio3_int_ena",
                &format_args!("{}", self.gpio3_int_ena().bit()),
            )
            .field(
                "gpio4_int_ena",
                &format_args!("{}", self.gpio4_int_ena().bit()),
            )
            .field(
                "gpio5_int_ena",
                &format_args!("{}", self.gpio5_int_ena().bit()),
            )
            .field(
                "gpio6_int_ena",
                &format_args!("{}", self.gpio6_int_ena().bit()),
            )
            .field(
                "gpio7_int_ena",
                &format_args!("{}", self.gpio7_int_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INTR_RLS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - The enable bit for DEDIC_GPIO0_INT_ST register."]
    #[inline(always)]
    #[must_use]
    pub fn gpio0_int_ena(&mut self) -> GPIO0_INT_ENA_W<0> {
        GPIO0_INT_ENA_W::new(self)
    }
    #[doc = "Bit 1 - The enable bit for DEDIC_GPIO1_INT_ST register."]
    #[inline(always)]
    #[must_use]
    pub fn gpio1_int_ena(&mut self) -> GPIO1_INT_ENA_W<1> {
        GPIO1_INT_ENA_W::new(self)
    }
    #[doc = "Bit 2 - The enable bit for DEDIC_GPIO2_INT_ST register."]
    #[inline(always)]
    #[must_use]
    pub fn gpio2_int_ena(&mut self) -> GPIO2_INT_ENA_W<2> {
        GPIO2_INT_ENA_W::new(self)
    }
    #[doc = "Bit 3 - The enable bit for DEDIC_GPIO3_INT_ST register."]
    #[inline(always)]
    #[must_use]
    pub fn gpio3_int_ena(&mut self) -> GPIO3_INT_ENA_W<3> {
        GPIO3_INT_ENA_W::new(self)
    }
    #[doc = "Bit 4 - The enable bit for DEDIC_GPIO4_INT_ST register."]
    #[inline(always)]
    #[must_use]
    pub fn gpio4_int_ena(&mut self) -> GPIO4_INT_ENA_W<4> {
        GPIO4_INT_ENA_W::new(self)
    }
    #[doc = "Bit 5 - The enable bit for DEDIC_GPIO5_INT_ST register."]
    #[inline(always)]
    #[must_use]
    pub fn gpio5_int_ena(&mut self) -> GPIO5_INT_ENA_W<5> {
        GPIO5_INT_ENA_W::new(self)
    }
    #[doc = "Bit 6 - The enable bit for DEDIC_GPIO6_INT_ST register."]
    #[inline(always)]
    #[must_use]
    pub fn gpio6_int_ena(&mut self) -> GPIO6_INT_ENA_W<6> {
        GPIO6_INT_ENA_W::new(self)
    }
    #[doc = "Bit 7 - The enable bit for DEDIC_GPIO7_INT_ST register."]
    #[inline(always)]
    #[must_use]
    pub fn gpio7_int_ena(&mut self) -> GPIO7_INT_ENA_W<7> {
        GPIO7_INT_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt enable bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_rls](index.html) module"]
pub struct INTR_RLS_SPEC;
impl crate::RegisterSpec for INTR_RLS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_rls::R](R) reader structure"]
impl crate::Readable for INTR_RLS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_rls::W](W) writer structure"]
impl crate::Writable for INTR_RLS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR_RLS to value 0"]
impl crate::Resettable for INTR_RLS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
