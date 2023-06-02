#[doc = "Register `WAKEUP_STATE` reader"]
pub struct R(crate::R<WAKEUP_STATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WAKEUP_STATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WAKEUP_STATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WAKEUP_STATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WAKEUP_STATE` writer"]
pub struct W(crate::W<WAKEUP_STATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WAKEUP_STATE_SPEC>;
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
impl From<crate::W<WAKEUP_STATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WAKEUP_STATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAKEUP_CAUSE` reader - wakeup cause"]
pub type WAKEUP_CAUSE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WAKEUP_ENA` reader - wakeup enable bitmap"]
pub type WAKEUP_ENA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WAKEUP_ENA` writer - wakeup enable bitmap"]
pub type WAKEUP_ENA_W<'a, const O: u8> = crate::FieldWriter<'a, WAKEUP_STATE_SPEC, 11, O, u16, u16>;
#[doc = "Field `GPIO_WAKEUP_FILTER` reader - enable filter for gpio wakeup event"]
pub type GPIO_WAKEUP_FILTER_R = crate::BitReader;
#[doc = "Field `GPIO_WAKEUP_FILTER` writer - enable filter for gpio wakeup event"]
pub type GPIO_WAKEUP_FILTER_W<'a, const O: u8> = crate::BitWriter<'a, WAKEUP_STATE_SPEC, O>;
impl R {
    #[doc = "Bits 0:10 - wakeup cause"]
    #[inline(always)]
    pub fn wakeup_cause(&self) -> WAKEUP_CAUSE_R {
        WAKEUP_CAUSE_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:21 - wakeup enable bitmap"]
    #[inline(always)]
    pub fn wakeup_ena(&self) -> WAKEUP_ENA_R {
        WAKEUP_ENA_R::new(((self.bits >> 11) & 0x07ff) as u16)
    }
    #[doc = "Bit 22 - enable filter for gpio wakeup event"]
    #[inline(always)]
    pub fn gpio_wakeup_filter(&self) -> GPIO_WAKEUP_FILTER_R {
        GPIO_WAKEUP_FILTER_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WAKEUP_STATE")
            .field(
                "wakeup_cause",
                &format_args!("{}", self.wakeup_cause().bits()),
            )
            .field("wakeup_ena", &format_args!("{}", self.wakeup_ena().bits()))
            .field(
                "gpio_wakeup_filter",
                &format_args!("{}", self.gpio_wakeup_filter().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<WAKEUP_STATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 11:21 - wakeup enable bitmap"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_ena(&mut self) -> WAKEUP_ENA_W<11> {
        WAKEUP_ENA_W::new(self)
    }
    #[doc = "Bit 22 - enable filter for gpio wakeup event"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_wakeup_filter(&mut self) -> GPIO_WAKEUP_FILTER_W<22> {
        GPIO_WAKEUP_FILTER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wakeup_state](index.html) module"]
pub struct WAKEUP_STATE_SPEC;
impl crate::RegisterSpec for WAKEUP_STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wakeup_state::R](R) reader structure"]
impl crate::Readable for WAKEUP_STATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wakeup_state::W](W) writer structure"]
impl crate::Writable for WAKEUP_STATE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WAKEUP_STATE to value 0x6000"]
impl crate::Resettable for WAKEUP_STATE_SPEC {
    const RESET_VALUE: Self::Ux = 0x6000;
}
