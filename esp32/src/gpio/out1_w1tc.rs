#[doc = "Register `OUT1_W1TC` reader"]
pub struct R(crate::R<OUT1_W1TC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT1_W1TC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT1_W1TC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT1_W1TC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUT1_W1TC` writer"]
pub struct W(crate::W<OUT1_W1TC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUT1_W1TC_SPEC>;
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
impl From<crate::W<OUT1_W1TC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUT1_W1TC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUT1_DATA_W1TC` reader - GPIO32~39 output value write 1 to clear"]
pub type OUT1_DATA_W1TC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OUT1_DATA_W1TC` writer - GPIO32~39 output value write 1 to clear"]
pub type OUT1_DATA_W1TC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OUT1_W1TC_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - GPIO32~39 output value write 1 to clear"]
    #[inline(always)]
    pub fn out1_data_w1tc(&self) -> OUT1_DATA_W1TC_R {
        OUT1_DATA_W1TC_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO32~39 output value write 1 to clear"]
    #[inline(always)]
    pub fn out1_data_w1tc(&mut self) -> OUT1_DATA_W1TC_W<0> {
        OUT1_DATA_W1TC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out1_w1tc](index.html) module"]
pub struct OUT1_W1TC_SPEC;
impl crate::RegisterSpec for OUT1_W1TC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out1_w1tc::R](R) reader structure"]
impl crate::Readable for OUT1_W1TC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [out1_w1tc::W](W) writer structure"]
impl crate::Writable for OUT1_W1TC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUT1_W1TC to value 0"]
impl crate::Resettable for OUT1_W1TC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
