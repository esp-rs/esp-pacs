#[doc = "Register `INT_ENA` reader"]
pub struct R(crate::R<INT_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_ENA` writer"]
pub struct W(crate::W<INT_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_ENA_SPEC>;
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
impl From<crate::W<INT_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TARGET0_INT_ENA` reader - interupt0 enable"]
pub type TARGET0_INT_ENA_R = crate::BitReader;
#[doc = "Field `TARGET0_INT_ENA` writer - interupt0 enable"]
pub type TARGET0_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `TARGET1_INT_ENA` reader - interupt1 enable"]
pub type TARGET1_INT_ENA_R = crate::BitReader;
#[doc = "Field `TARGET1_INT_ENA` writer - interupt1 enable"]
pub type TARGET1_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `TARGET2_INT_ENA` reader - interupt2 enable"]
pub type TARGET2_INT_ENA_R = crate::BitReader;
#[doc = "Field `TARGET2_INT_ENA` writer - interupt2 enable"]
pub type TARGET2_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
impl R {
    #[doc = "Bit 0 - interupt0 enable"]
    #[inline(always)]
    pub fn target0_int_ena(&self) -> TARGET0_INT_ENA_R {
        TARGET0_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - interupt1 enable"]
    #[inline(always)]
    pub fn target1_int_ena(&self) -> TARGET1_INT_ENA_R {
        TARGET1_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - interupt2 enable"]
    #[inline(always)]
    pub fn target2_int_ena(&self) -> TARGET2_INT_ENA_R {
        TARGET2_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field(
                "target0_int_ena",
                &format_args!("{}", self.target0_int_ena().bit()),
            )
            .field(
                "target1_int_ena",
                &format_args!("{}", self.target1_int_ena().bit()),
            )
            .field(
                "target2_int_ena",
                &format_args!("{}", self.target2_int_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - interupt0 enable"]
    #[inline(always)]
    #[must_use]
    pub fn target0_int_ena(&mut self) -> TARGET0_INT_ENA_W<0> {
        TARGET0_INT_ENA_W::new(self)
    }
    #[doc = "Bit 1 - interupt1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn target1_int_ena(&mut self) -> TARGET1_INT_ENA_W<1> {
        TARGET1_INT_ENA_W::new(self)
    }
    #[doc = "Bit 2 - interupt2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn target2_int_ena(&mut self) -> TARGET2_INT_ENA_W<2> {
        TARGET2_INT_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "systimer interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ena](index.html) module"]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_ena::R](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_ena::W](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
