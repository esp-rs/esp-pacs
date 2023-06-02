#[doc = "Register `TOUCH_TIMEOUT_CTRL` reader"]
pub struct R(crate::R<TOUCH_TIMEOUT_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOUCH_TIMEOUT_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOUCH_TIMEOUT_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOUCH_TIMEOUT_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TOUCH_TIMEOUT_CTRL` writer"]
pub struct W(crate::W<TOUCH_TIMEOUT_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOUCH_TIMEOUT_CTRL_SPEC>;
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
impl From<crate::W<TOUCH_TIMEOUT_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOUCH_TIMEOUT_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOUCH_TIMEOUT_NUM` reader - Set touch timeout threshold."]
pub type TOUCH_TIMEOUT_NUM_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TOUCH_TIMEOUT_NUM` writer - Set touch timeout threshold."]
pub type TOUCH_TIMEOUT_NUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, TOUCH_TIMEOUT_CTRL_SPEC, 22, O, u32, u32>;
#[doc = "Field `TOUCH_TIMEOUT_EN` reader - Enable touch timeout."]
pub type TOUCH_TIMEOUT_EN_R = crate::BitReader;
#[doc = "Field `TOUCH_TIMEOUT_EN` writer - Enable touch timeout."]
pub type TOUCH_TIMEOUT_EN_W<'a, const O: u8> = crate::BitWriter<'a, TOUCH_TIMEOUT_CTRL_SPEC, O>;
impl R {
    #[doc = "Bits 0:21 - Set touch timeout threshold."]
    #[inline(always)]
    pub fn touch_timeout_num(&self) -> TOUCH_TIMEOUT_NUM_R {
        TOUCH_TIMEOUT_NUM_R::new(self.bits & 0x003f_ffff)
    }
    #[doc = "Bit 22 - Enable touch timeout."]
    #[inline(always)]
    pub fn touch_timeout_en(&self) -> TOUCH_TIMEOUT_EN_R {
        TOUCH_TIMEOUT_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOUCH_TIMEOUT_CTRL")
            .field(
                "touch_timeout_num",
                &format_args!("{}", self.touch_timeout_num().bits()),
            )
            .field(
                "touch_timeout_en",
                &format_args!("{}", self.touch_timeout_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TOUCH_TIMEOUT_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:21 - Set touch timeout threshold."]
    #[inline(always)]
    #[must_use]
    pub fn touch_timeout_num(&mut self) -> TOUCH_TIMEOUT_NUM_W<0> {
        TOUCH_TIMEOUT_NUM_W::new(self)
    }
    #[doc = "Bit 22 - Enable touch timeout."]
    #[inline(always)]
    #[must_use]
    pub fn touch_timeout_en(&mut self) -> TOUCH_TIMEOUT_EN_W<22> {
        TOUCH_TIMEOUT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configure touch timeout settings\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [touch_timeout_ctrl](index.html) module"]
pub struct TOUCH_TIMEOUT_CTRL_SPEC;
impl crate::RegisterSpec for TOUCH_TIMEOUT_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [touch_timeout_ctrl::R](R) reader structure"]
impl crate::Readable for TOUCH_TIMEOUT_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [touch_timeout_ctrl::W](W) writer structure"]
impl crate::Writable for TOUCH_TIMEOUT_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TOUCH_TIMEOUT_CTRL to value 0x007f_ffff"]
impl crate::Resettable for TOUCH_TIMEOUT_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x007f_ffff;
}
