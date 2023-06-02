#[doc = "Register `ENABLE1_W1TS` reader"]
pub struct R(crate::R<ENABLE1_W1TS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENABLE1_W1TS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENABLE1_W1TS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENABLE1_W1TS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ENABLE1_W1TS` writer"]
pub struct W(crate::W<ENABLE1_W1TS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENABLE1_W1TS_SPEC>;
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
impl From<crate::W<ENABLE1_W1TS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENABLE1_W1TS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE1_DATA_W1TS` reader - GPIO32~39 output enable write 1 to set"]
pub type ENABLE1_DATA_W1TS_R = crate::FieldReader;
#[doc = "Field `ENABLE1_DATA_W1TS` writer - GPIO32~39 output enable write 1 to set"]
pub type ENABLE1_DATA_W1TS_W<'a, const O: u8> = crate::FieldWriter<'a, ENABLE1_W1TS_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - GPIO32~39 output enable write 1 to set"]
    #[inline(always)]
    pub fn enable1_data_w1ts(&self) -> ENABLE1_DATA_W1TS_R {
        ENABLE1_DATA_W1TS_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENABLE1_W1TS")
            .field(
                "enable1_data_w1ts",
                &format_args!("{}", self.enable1_data_w1ts().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ENABLE1_W1TS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO32~39 output enable write 1 to set"]
    #[inline(always)]
    #[must_use]
    pub fn enable1_data_w1ts(&mut self) -> ENABLE1_DATA_W1TS_W<0> {
        ENABLE1_DATA_W1TS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enable1_w1ts](index.html) module"]
pub struct ENABLE1_W1TS_SPEC;
impl crate::RegisterSpec for ENABLE1_W1TS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [enable1_w1ts::R](R) reader structure"]
impl crate::Readable for ENABLE1_W1TS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [enable1_w1ts::W](W) writer structure"]
impl crate::Writable for ENABLE1_W1TS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ENABLE1_W1TS to value 0"]
impl crate::Resettable for ENABLE1_W1TS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
