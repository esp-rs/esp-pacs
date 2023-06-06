#[doc = "Register `TOUCH_CTRL1` reader"]
pub struct R(crate::R<TOUCH_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOUCH_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOUCH_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOUCH_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TOUCH_CTRL1` writer"]
pub struct W(crate::W<TOUCH_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOUCH_CTRL1_SPEC>;
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
impl From<crate::W<TOUCH_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOUCH_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOUCH_SLEEP_CYCLES` reader - sleep cycles for timer"]
pub type TOUCH_SLEEP_CYCLES_R = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_SLEEP_CYCLES` writer - sleep cycles for timer"]
pub type TOUCH_SLEEP_CYCLES_W<'a, const O: u8> =
    crate::FieldWriter<'a, TOUCH_CTRL1_SPEC, 16, O, u16>;
#[doc = "Field `TOUCH_MEAS_NUM` reader - the meas length (in 8MHz)"]
pub type TOUCH_MEAS_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_MEAS_NUM` writer - the meas length (in 8MHz)"]
pub type TOUCH_MEAS_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, TOUCH_CTRL1_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - sleep cycles for timer"]
    #[inline(always)]
    pub fn touch_sleep_cycles(&self) -> TOUCH_SLEEP_CYCLES_R {
        TOUCH_SLEEP_CYCLES_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - the meas length (in 8MHz)"]
    #[inline(always)]
    pub fn touch_meas_num(&self) -> TOUCH_MEAS_NUM_R {
        TOUCH_MEAS_NUM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOUCH_CTRL1")
            .field(
                "touch_sleep_cycles",
                &format_args!("{}", self.touch_sleep_cycles().bits()),
            )
            .field(
                "touch_meas_num",
                &format_args!("{}", self.touch_meas_num().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TOUCH_CTRL1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - sleep cycles for timer"]
    #[inline(always)]
    #[must_use]
    pub fn touch_sleep_cycles(&mut self) -> TOUCH_SLEEP_CYCLES_W<0> {
        TOUCH_SLEEP_CYCLES_W::new(self)
    }
    #[doc = "Bits 16:31 - the meas length (in 8MHz)"]
    #[inline(always)]
    #[must_use]
    pub fn touch_meas_num(&mut self) -> TOUCH_MEAS_NUM_W<16> {
        TOUCH_MEAS_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure touch controller\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [touch_ctrl1](index.html) module"]
pub struct TOUCH_CTRL1_SPEC;
impl crate::RegisterSpec for TOUCH_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [touch_ctrl1::R](R) reader structure"]
impl crate::Readable for TOUCH_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [touch_ctrl1::W](W) writer structure"]
impl crate::Writable for TOUCH_CTRL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TOUCH_CTRL1 to value 0x1000_0100"]
impl crate::Resettable for TOUCH_CTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000_0100;
}
