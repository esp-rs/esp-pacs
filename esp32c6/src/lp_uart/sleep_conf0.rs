#[doc = "Register `SLEEP_CONF0` reader"]
pub struct R(crate::R<SLEEP_CONF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLEEP_CONF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLEEP_CONF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLEEP_CONF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLEEP_CONF0` writer"]
pub struct W(crate::W<SLEEP_CONF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLEEP_CONF0_SPEC>;
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
impl From<crate::W<SLEEP_CONF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLEEP_CONF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WK_CHAR1` reader - This register restores the specified wake up char1 to wake up"]
pub type WK_CHAR1_R = crate::FieldReader;
#[doc = "Field `WK_CHAR1` writer - This register restores the specified wake up char1 to wake up"]
pub type WK_CHAR1_W<'a, const O: u8> = crate::FieldWriter<'a, SLEEP_CONF0_SPEC, 8, O>;
#[doc = "Field `WK_CHAR2` reader - This register restores the specified wake up char2 to wake up"]
pub type WK_CHAR2_R = crate::FieldReader;
#[doc = "Field `WK_CHAR2` writer - This register restores the specified wake up char2 to wake up"]
pub type WK_CHAR2_W<'a, const O: u8> = crate::FieldWriter<'a, SLEEP_CONF0_SPEC, 8, O>;
#[doc = "Field `WK_CHAR3` reader - This register restores the specified wake up char3 to wake up"]
pub type WK_CHAR3_R = crate::FieldReader;
#[doc = "Field `WK_CHAR3` writer - This register restores the specified wake up char3 to wake up"]
pub type WK_CHAR3_W<'a, const O: u8> = crate::FieldWriter<'a, SLEEP_CONF0_SPEC, 8, O>;
#[doc = "Field `WK_CHAR4` reader - This register restores the specified wake up char4 to wake up"]
pub type WK_CHAR4_R = crate::FieldReader;
#[doc = "Field `WK_CHAR4` writer - This register restores the specified wake up char4 to wake up"]
pub type WK_CHAR4_W<'a, const O: u8> = crate::FieldWriter<'a, SLEEP_CONF0_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - This register restores the specified wake up char1 to wake up"]
    #[inline(always)]
    pub fn wk_char1(&self) -> WK_CHAR1_R {
        WK_CHAR1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - This register restores the specified wake up char2 to wake up"]
    #[inline(always)]
    pub fn wk_char2(&self) -> WK_CHAR2_R {
        WK_CHAR2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - This register restores the specified wake up char3 to wake up"]
    #[inline(always)]
    pub fn wk_char3(&self) -> WK_CHAR3_R {
        WK_CHAR3_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - This register restores the specified wake up char4 to wake up"]
    #[inline(always)]
    pub fn wk_char4(&self) -> WK_CHAR4_R {
        WK_CHAR4_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLEEP_CONF0")
            .field("wk_char1", &format_args!("{}", self.wk_char1().bits()))
            .field("wk_char2", &format_args!("{}", self.wk_char2().bits()))
            .field("wk_char3", &format_args!("{}", self.wk_char3().bits()))
            .field("wk_char4", &format_args!("{}", self.wk_char4().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLEEP_CONF0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register restores the specified wake up char1 to wake up"]
    #[inline(always)]
    #[must_use]
    pub fn wk_char1(&mut self) -> WK_CHAR1_W<0> {
        WK_CHAR1_W::new(self)
    }
    #[doc = "Bits 8:15 - This register restores the specified wake up char2 to wake up"]
    #[inline(always)]
    #[must_use]
    pub fn wk_char2(&mut self) -> WK_CHAR2_W<8> {
        WK_CHAR2_W::new(self)
    }
    #[doc = "Bits 16:23 - This register restores the specified wake up char3 to wake up"]
    #[inline(always)]
    #[must_use]
    pub fn wk_char3(&mut self) -> WK_CHAR3_W<16> {
        WK_CHAR3_W::new(self)
    }
    #[doc = "Bits 24:31 - This register restores the specified wake up char4 to wake up"]
    #[inline(always)]
    #[must_use]
    pub fn wk_char4(&mut self) -> WK_CHAR4_W<24> {
        WK_CHAR4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART sleep configure register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sleep_conf0](index.html) module"]
pub struct SLEEP_CONF0_SPEC;
impl crate::RegisterSpec for SLEEP_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sleep_conf0::R](R) reader structure"]
impl crate::Readable for SLEEP_CONF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sleep_conf0::W](W) writer structure"]
impl crate::Writable for SLEEP_CONF0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLEEP_CONF0 to value 0"]
impl crate::Resettable for SLEEP_CONF0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
