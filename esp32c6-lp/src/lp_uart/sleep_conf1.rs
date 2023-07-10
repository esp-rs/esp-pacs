#[doc = "Register `SLEEP_CONF1` reader"]
pub struct R(crate::R<SLEEP_CONF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLEEP_CONF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLEEP_CONF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLEEP_CONF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLEEP_CONF1` writer"]
pub struct W(crate::W<SLEEP_CONF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLEEP_CONF1_SPEC>;
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
impl From<crate::W<SLEEP_CONF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLEEP_CONF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WK_CHAR0` reader - This register restores the specified char0 to wake up"]
pub type WK_CHAR0_R = crate::FieldReader;
#[doc = "Field `WK_CHAR0` writer - This register restores the specified char0 to wake up"]
pub type WK_CHAR0_W<'a, const O: u8> = crate::FieldWriter<'a, SLEEP_CONF1_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - This register restores the specified char0 to wake up"]
    #[inline(always)]
    pub fn wk_char0(&self) -> WK_CHAR0_R {
        WK_CHAR0_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLEEP_CONF1")
            .field("wk_char0", &format_args!("{}", self.wk_char0().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLEEP_CONF1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register restores the specified char0 to wake up"]
    #[inline(always)]
    #[must_use]
    pub fn wk_char0(&mut self) -> WK_CHAR0_W<0> {
        WK_CHAR0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART sleep configure register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sleep_conf1](index.html) module"]
pub struct SLEEP_CONF1_SPEC;
impl crate::RegisterSpec for SLEEP_CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sleep_conf1::R](R) reader structure"]
impl crate::Readable for SLEEP_CONF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sleep_conf1::W](W) writer structure"]
impl crate::Writable for SLEEP_CONF1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLEEP_CONF1 to value 0"]
impl crate::Resettable for SLEEP_CONF1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
