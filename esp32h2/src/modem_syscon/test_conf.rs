#[doc = "Register `TEST_CONF` reader"]
pub struct R(crate::R<TEST_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEST_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEST_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEST_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TEST_CONF` writer"]
pub struct W(crate::W<TEST_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEST_CONF_SPEC>;
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
impl From<crate::W<TEST_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEST_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLK_EN` reader - "]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - "]
pub type CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, TEST_CONF_SPEC, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TEST_CONF")
            .field("clk_en", &format_args!("{}", self.clk_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TEST_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<0> {
        CLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [test_conf](index.html) module"]
pub struct TEST_CONF_SPEC;
impl crate::RegisterSpec for TEST_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [test_conf::R](R) reader structure"]
impl crate::Readable for TEST_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [test_conf::W](W) writer structure"]
impl crate::Writable for TEST_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TEST_CONF to value 0"]
impl crate::Resettable for TEST_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
