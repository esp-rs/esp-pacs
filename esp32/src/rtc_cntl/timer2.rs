#[doc = "Register `TIMER2` reader"]
pub struct R(crate::R<TIMER2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER2` writer"]
pub struct W(crate::W<TIMER2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER2_SPEC>;
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
impl From<crate::W<TIMER2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ULPCP_TOUCH_START_WAIT` reader - wait cycles in slow_clk_rtc before ULP-coprocessor / touch controller start to work"]
pub type ULPCP_TOUCH_START_WAIT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ULPCP_TOUCH_START_WAIT` writer - wait cycles in slow_clk_rtc before ULP-coprocessor / touch controller start to work"]
pub type ULPCP_TOUCH_START_WAIT_W<'a, const O: u8> =
    crate::FieldWriter<'a, TIMER2_SPEC, 9, O, u16, u16>;
#[doc = "Field `MIN_TIME_CK8M_OFF` reader - minimal cycles in slow_clk_rtc for CK8M in power down state"]
pub type MIN_TIME_CK8M_OFF_R = crate::FieldReader;
#[doc = "Field `MIN_TIME_CK8M_OFF` writer - minimal cycles in slow_clk_rtc for CK8M in power down state"]
pub type MIN_TIME_CK8M_OFF_W<'a, const O: u8> = crate::FieldWriter<'a, TIMER2_SPEC, 8, O>;
impl R {
    #[doc = "Bits 15:23 - wait cycles in slow_clk_rtc before ULP-coprocessor / touch controller start to work"]
    #[inline(always)]
    pub fn ulpcp_touch_start_wait(&self) -> ULPCP_TOUCH_START_WAIT_R {
        ULPCP_TOUCH_START_WAIT_R::new(((self.bits >> 15) & 0x01ff) as u16)
    }
    #[doc = "Bits 24:31 - minimal cycles in slow_clk_rtc for CK8M in power down state"]
    #[inline(always)]
    pub fn min_time_ck8m_off(&self) -> MIN_TIME_CK8M_OFF_R {
        MIN_TIME_CK8M_OFF_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER2")
            .field(
                "ulpcp_touch_start_wait",
                &format_args!("{}", self.ulpcp_touch_start_wait().bits()),
            )
            .field(
                "min_time_ck8m_off",
                &format_args!("{}", self.min_time_ck8m_off().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TIMER2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 15:23 - wait cycles in slow_clk_rtc before ULP-coprocessor / touch controller start to work"]
    #[inline(always)]
    #[must_use]
    pub fn ulpcp_touch_start_wait(&mut self) -> ULPCP_TOUCH_START_WAIT_W<15> {
        ULPCP_TOUCH_START_WAIT_W::new(self)
    }
    #[doc = "Bits 24:31 - minimal cycles in slow_clk_rtc for CK8M in power down state"]
    #[inline(always)]
    #[must_use]
    pub fn min_time_ck8m_off(&mut self) -> MIN_TIME_CK8M_OFF_W<24> {
        MIN_TIME_CK8M_OFF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer2](index.html) module"]
pub struct TIMER2_SPEC;
impl crate::RegisterSpec for TIMER2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer2::R](R) reader structure"]
impl crate::Readable for TIMER2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer2::W](W) writer structure"]
impl crate::Writable for TIMER2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMER2 to value 0x0108_0000"]
impl crate::Resettable for TIMER2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0108_0000;
}
