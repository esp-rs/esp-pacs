#[doc = "Register `T%sUPDATE` reader"]
pub struct R(crate::R<TUPDATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TUPDATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TUPDATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TUPDATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `T%sUPDATE` writer"]
pub struct W(crate::W<TUPDATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TUPDATE_SPEC>;
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
impl From<crate::W<TUPDATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TUPDATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UPDATE` reader - After writing 0 or 1 to TIMG_T%sUPDATE_REG, the counter value is latched."]
pub type UPDATE_R = crate::BitReader<bool>;
#[doc = "Field `UPDATE` writer - After writing 0 or 1 to TIMG_T%sUPDATE_REG, the counter value is latched."]
pub type UPDATE_W<'a> = crate::BitWriter<'a, u32, TUPDATE_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 31 - After writing 0 or 1 to TIMG_T%sUPDATE_REG, the counter value is latched."]
    #[inline(always)]
    pub fn update(&self) -> UPDATE_R {
        UPDATE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - After writing 0 or 1 to TIMG_T%sUPDATE_REG, the counter value is latched."]
    #[inline(always)]
    pub fn update(&mut self) -> UPDATE_W {
        UPDATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write to copy current timer value to TIMG_T%sLO_REG or TIMGn_T%sHI_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tupdate](index.html) module"]
pub struct TUPDATE_SPEC;
impl crate::RegisterSpec for TUPDATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tupdate::R](R) reader structure"]
impl crate::Readable for TUPDATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tupdate::W](W) writer structure"]
impl crate::Writable for TUPDATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets T%sUPDATE to value 0"]
impl crate::Resettable for TUPDATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
