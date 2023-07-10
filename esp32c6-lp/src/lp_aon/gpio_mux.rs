#[doc = "Register `GPIO_MUX` reader"]
pub struct R(crate::R<GPIO_MUX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_MUX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_MUX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_MUX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_MUX` writer"]
pub struct W(crate::W<GPIO_MUX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_MUX_SPEC>;
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
impl From<crate::W<GPIO_MUX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_MUX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL` reader - need_des"]
pub type SEL_R = crate::FieldReader;
#[doc = "Field `SEL` writer - need_des"]
pub type SEL_W<'a, const O: u8> = crate::FieldWriter<'a, GPIO_MUX_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_MUX")
            .field("sel", &format_args!("{}", self.sel().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GPIO_MUX_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SEL_W<0> {
        SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_mux](index.html) module"]
pub struct GPIO_MUX_SPEC;
impl crate::RegisterSpec for GPIO_MUX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_mux::R](R) reader structure"]
impl crate::Readable for GPIO_MUX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_mux::W](W) writer structure"]
impl crate::Writable for GPIO_MUX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO_MUX to value 0"]
impl crate::Resettable for GPIO_MUX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
