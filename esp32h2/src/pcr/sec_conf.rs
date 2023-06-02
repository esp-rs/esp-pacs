#[doc = "Register `SEC_CONF` reader"]
pub struct R(crate::R<SEC_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEC_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEC_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEC_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEC_CONF` writer"]
pub struct W(crate::W<SEC_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEC_CONF_SPEC>;
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
impl From<crate::W<SEC_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEC_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEC_CLK_SEL` reader - xxxx"]
pub type SEC_CLK_SEL_R = crate::FieldReader;
#[doc = "Field `SEC_CLK_SEL` writer - xxxx"]
pub type SEC_CLK_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, SEC_CONF_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:1 - xxxx"]
    #[inline(always)]
    pub fn sec_clk_sel(&self) -> SEC_CLK_SEL_R {
        SEC_CLK_SEL_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CONF")
            .field(
                "sec_clk_sel",
                &format_args!("{}", self.sec_clk_sel().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SEC_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - xxxx"]
    #[inline(always)]
    #[must_use]
    pub fn sec_clk_sel(&mut self) -> SEC_CLK_SEL_W<0> {
        SEC_CLK_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "xxxx\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_conf](index.html) module"]
pub struct SEC_CONF_SPEC;
impl crate::RegisterSpec for SEC_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sec_conf::R](R) reader structure"]
impl crate::Readable for SEC_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sec_conf::W](W) writer structure"]
impl crate::Writable for SEC_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEC_CONF to value 0"]
impl crate::Resettable for SEC_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
