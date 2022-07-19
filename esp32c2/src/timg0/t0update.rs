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
#[doc = "Field `T0_UPDATE` reader - After writing 0 or 1 to TIMG_T31UPDATE_REG, the counter value is latched."]
pub type T0_UPDATE_R = crate::BitReader<bool>;
#[doc = "Field `T0_UPDATE` writer - After writing 0 or 1 to TIMG_T31UPDATE_REG, the counter value is latched."]
pub type T0_UPDATE_W<'a> = crate::BitWriter<'a, u32, T0UPDATE_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 31 - After writing 0 or 1 to TIMG_T31UPDATE_REG, the counter value is latched."]
    #[inline(always)]
    pub fn t0_update(&self) -> T0_UPDATE_R {
        T0_UPDATE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - After writing 0 or 1 to TIMG_T31UPDATE_REG, the counter value is latched."]
    #[inline(always)]
    pub fn t0_update(&mut self) -> T0_UPDATE_W {
        T0_UPDATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write to copy current timer value to TIMGn_T$x_(LO/HI)_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t0update](index.html) module"]
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
}
#[doc = "`reset()` method sets T0UPDATE to value 0"]
impl crate::Resettable for T0UPDATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
