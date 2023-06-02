#[doc = "Register `T0UPDATE` reader"]
pub struct R(crate::R<T0UPDATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T0UPDATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T0UPDATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T0UPDATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `T0UPDATE` writer"]
pub struct W(crate::W<T0UPDATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<T0UPDATE_SPEC>;
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
impl From<crate::W<T0UPDATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<T0UPDATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UPDATE` reader - After writing 0 or 1 to TIMG_T%sUPDATE_REG, the counter value is latched."]
pub type UPDATE_R = crate::BitReader;
#[doc = "Field `UPDATE` writer - After writing 0 or 1 to TIMG_T%sUPDATE_REG, the counter value is latched."]
pub type UPDATE_W<'a, const O: u8> = crate::BitWriter<'a, T0UPDATE_SPEC, O>;
impl R {
    #[doc = "Bit 31 - After writing 0 or 1 to TIMG_T%sUPDATE_REG, the counter value is latched."]
    #[inline(always)]
    pub fn update(&self) -> UPDATE_R {
        UPDATE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T0UPDATE")
            .field("update", &format_args!("{}", self.update().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<T0UPDATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 31 - After writing 0 or 1 to TIMG_T%sUPDATE_REG, the counter value is latched."]
    #[inline(always)]
    #[must_use]
    pub fn update(&mut self) -> UPDATE_W<31> {
        UPDATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write to copy current timer value to TIMGn_T%s_(LO/HI)_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t0update](index.html) module"]
pub struct T0UPDATE_SPEC;
impl crate::RegisterSpec for T0UPDATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t0update::R](R) reader structure"]
impl crate::Readable for T0UPDATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [t0update::W](W) writer structure"]
impl crate::Writable for T0UPDATE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets T0UPDATE to value 0"]
impl crate::Resettable for T0UPDATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
