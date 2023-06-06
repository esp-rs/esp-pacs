#[doc = "Register `LPCK_DIV_INT` reader"]
pub struct R(crate::R<LPCK_DIV_INT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPCK_DIV_INT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPCK_DIV_INT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPCK_DIV_INT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPCK_DIV_INT` writer"]
pub struct W(crate::W<LPCK_DIV_INT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPCK_DIV_INT_SPEC>;
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
impl From<crate::W<LPCK_DIV_INT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPCK_DIV_INT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPCK_DIV_NUM` reader - This field is used to set the integer number of the divider value."]
pub type LPCK_DIV_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `LPCK_DIV_NUM` writer - This field is used to set the integer number of the divider value."]
pub type LPCK_DIV_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, LPCK_DIV_INT_SPEC, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - This field is used to set the integer number of the divider value."]
    #[inline(always)]
    pub fn lpck_div_num(&self) -> LPCK_DIV_NUM_R {
        LPCK_DIV_NUM_R::new((self.bits & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPCK_DIV_INT")
            .field(
                "lpck_div_num",
                &format_args!("{}", self.lpck_div_num().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LPCK_DIV_INT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:11 - This field is used to set the integer number of the divider value."]
    #[inline(always)]
    #[must_use]
    pub fn lpck_div_num(&mut self) -> LPCK_DIV_NUM_W<0> {
        LPCK_DIV_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Low power clock divider integer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpck_div_int](index.html) module"]
pub struct LPCK_DIV_INT_SPEC;
impl crate::RegisterSpec for LPCK_DIV_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpck_div_int::R](R) reader structure"]
impl crate::Readable for LPCK_DIV_INT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpck_div_int::W](W) writer structure"]
impl crate::Writable for LPCK_DIV_INT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPCK_DIV_INT to value 0xff"]
impl crate::Resettable for LPCK_DIV_INT_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
