#[doc = "Register `HW_STANDBY_CNT` reader"]
pub struct R(crate::R<HW_STANDBY_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HW_STANDBY_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HW_STANDBY_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HW_STANDBY_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HW_STANDBY_CNT` writer"]
pub struct W(crate::W<HW_STANDBY_CNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HW_STANDBY_CNT_SPEC>;
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
impl From<crate::W<HW_STANDBY_CNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HW_STANDBY_CNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STANDBY_WAIT_CNT` reader - Configure the number of cycles before standby becomes high when TWAI_HW_STANDBY_EN is enabled."]
pub type STANDBY_WAIT_CNT_R = crate::FieldReader<u32>;
#[doc = "Field `STANDBY_WAIT_CNT` writer - Configure the number of cycles before standby becomes high when TWAI_HW_STANDBY_EN is enabled."]
pub type STANDBY_WAIT_CNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, HW_STANDBY_CNT_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Configure the number of cycles before standby becomes high when TWAI_HW_STANDBY_EN is enabled."]
    #[inline(always)]
    pub fn standby_wait_cnt(&self) -> STANDBY_WAIT_CNT_R {
        STANDBY_WAIT_CNT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HW_STANDBY_CNT")
            .field(
                "standby_wait_cnt",
                &format_args!("{}", self.standby_wait_cnt().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HW_STANDBY_CNT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Configure the number of cycles before standby becomes high when TWAI_HW_STANDBY_EN is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn standby_wait_cnt(&mut self) -> STANDBY_WAIT_CNT_W<0> {
        STANDBY_WAIT_CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configure standby counter.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hw_standby_cnt](index.html) module"]
pub struct HW_STANDBY_CNT_SPEC;
impl crate::RegisterSpec for HW_STANDBY_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hw_standby_cnt::R](R) reader structure"]
impl crate::Readable for HW_STANDBY_CNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hw_standby_cnt::W](W) writer structure"]
impl crate::Writable for HW_STANDBY_CNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HW_STANDBY_CNT to value 0x01"]
impl crate::Resettable for HW_STANDBY_CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
