#[doc = "Register `SAR_TOUCH_CONF` reader"]
pub struct R(crate::R<SAR_TOUCH_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_TOUCH_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_TOUCH_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_TOUCH_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_TOUCH_CONF` writer"]
pub struct W(crate::W<SAR_TOUCH_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_TOUCH_CONF_SPEC>;
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
impl From<crate::W<SAR_TOUCH_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_TOUCH_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOUCH_OUTEN` reader - Enable touch controller output."]
pub type TOUCH_OUTEN_R = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_OUTEN` writer - Enable touch controller output."]
pub type TOUCH_OUTEN_W<'a, const O: u8> = crate::FieldWriter<'a, SAR_TOUCH_CONF_SPEC, 15, O, u16>;
#[doc = "Field `TOUCH_STATUS_CLR` writer - Clear all touch active status."]
pub type TOUCH_STATUS_CLR_W<'a, const O: u8> = crate::BitWriter<'a, SAR_TOUCH_CONF_SPEC, O>;
#[doc = "Field `TOUCH_DATA_SEL` reader - 0 and 1: touch_raw_data; 2: base_line; 3: touch_smooth_data."]
pub type TOUCH_DATA_SEL_R = crate::FieldReader;
#[doc = "Field `TOUCH_DATA_SEL` writer - 0 and 1: touch_raw_data; 2: base_line; 3: touch_smooth_data."]
pub type TOUCH_DATA_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, SAR_TOUCH_CONF_SPEC, 2, O>;
#[doc = "Field `TOUCH_DENOISE_END` reader - Touch denoise done."]
pub type TOUCH_DENOISE_END_R = crate::BitReader;
#[doc = "Field `TOUCH_UNIT_END` reader - Indicate the completion of sampling."]
pub type TOUCH_UNIT_END_R = crate::BitReader;
#[doc = "Field `TOUCH_APPROACH_PAD2` reader - Indicate which pad is selected as proximity pad2"]
pub type TOUCH_APPROACH_PAD2_R = crate::FieldReader;
#[doc = "Field `TOUCH_APPROACH_PAD2` writer - Indicate which pad is selected as proximity pad2"]
pub type TOUCH_APPROACH_PAD2_W<'a, const O: u8> = crate::FieldWriter<'a, SAR_TOUCH_CONF_SPEC, 4, O>;
#[doc = "Field `TOUCH_APPROACH_PAD1` reader - Indicate which pad is selected as proximity pad1"]
pub type TOUCH_APPROACH_PAD1_R = crate::FieldReader;
#[doc = "Field `TOUCH_APPROACH_PAD1` writer - Indicate which pad is selected as proximity pad1"]
pub type TOUCH_APPROACH_PAD1_W<'a, const O: u8> = crate::FieldWriter<'a, SAR_TOUCH_CONF_SPEC, 4, O>;
#[doc = "Field `TOUCH_APPROACH_PAD0` reader - Indicate which pad is selected as proximity pad0"]
pub type TOUCH_APPROACH_PAD0_R = crate::FieldReader;
#[doc = "Field `TOUCH_APPROACH_PAD0` writer - Indicate which pad is selected as proximity pad0"]
pub type TOUCH_APPROACH_PAD0_W<'a, const O: u8> = crate::FieldWriter<'a, SAR_TOUCH_CONF_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:14 - Enable touch controller output."]
    #[inline(always)]
    pub fn touch_outen(&self) -> TOUCH_OUTEN_R {
        TOUCH_OUTEN_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:17 - 0 and 1: touch_raw_data; 2: base_line; 3: touch_smooth_data."]
    #[inline(always)]
    pub fn touch_data_sel(&self) -> TOUCH_DATA_SEL_R {
        TOUCH_DATA_SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Touch denoise done."]
    #[inline(always)]
    pub fn touch_denoise_end(&self) -> TOUCH_DENOISE_END_R {
        TOUCH_DENOISE_END_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Indicate the completion of sampling."]
    #[inline(always)]
    pub fn touch_unit_end(&self) -> TOUCH_UNIT_END_R {
        TOUCH_UNIT_END_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - Indicate which pad is selected as proximity pad2"]
    #[inline(always)]
    pub fn touch_approach_pad2(&self) -> TOUCH_APPROACH_PAD2_R {
        TOUCH_APPROACH_PAD2_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Indicate which pad is selected as proximity pad1"]
    #[inline(always)]
    pub fn touch_approach_pad1(&self) -> TOUCH_APPROACH_PAD1_R {
        TOUCH_APPROACH_PAD1_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Indicate which pad is selected as proximity pad0"]
    #[inline(always)]
    pub fn touch_approach_pad0(&self) -> TOUCH_APPROACH_PAD0_R {
        TOUCH_APPROACH_PAD0_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_TOUCH_CONF")
            .field(
                "touch_outen",
                &format_args!("{}", self.touch_outen().bits()),
            )
            .field(
                "touch_data_sel",
                &format_args!("{}", self.touch_data_sel().bits()),
            )
            .field(
                "touch_denoise_end",
                &format_args!("{}", self.touch_denoise_end().bit()),
            )
            .field(
                "touch_unit_end",
                &format_args!("{}", self.touch_unit_end().bit()),
            )
            .field(
                "touch_approach_pad2",
                &format_args!("{}", self.touch_approach_pad2().bits()),
            )
            .field(
                "touch_approach_pad1",
                &format_args!("{}", self.touch_approach_pad1().bits()),
            )
            .field(
                "touch_approach_pad0",
                &format_args!("{}", self.touch_approach_pad0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_TOUCH_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:14 - Enable touch controller output."]
    #[inline(always)]
    #[must_use]
    pub fn touch_outen(&mut self) -> TOUCH_OUTEN_W<0> {
        TOUCH_OUTEN_W::new(self)
    }
    #[doc = "Bit 15 - Clear all touch active status."]
    #[inline(always)]
    #[must_use]
    pub fn touch_status_clr(&mut self) -> TOUCH_STATUS_CLR_W<15> {
        TOUCH_STATUS_CLR_W::new(self)
    }
    #[doc = "Bits 16:17 - 0 and 1: touch_raw_data; 2: base_line; 3: touch_smooth_data."]
    #[inline(always)]
    #[must_use]
    pub fn touch_data_sel(&mut self) -> TOUCH_DATA_SEL_W<16> {
        TOUCH_DATA_SEL_W::new(self)
    }
    #[doc = "Bits 20:23 - Indicate which pad is selected as proximity pad2"]
    #[inline(always)]
    #[must_use]
    pub fn touch_approach_pad2(&mut self) -> TOUCH_APPROACH_PAD2_W<20> {
        TOUCH_APPROACH_PAD2_W::new(self)
    }
    #[doc = "Bits 24:27 - Indicate which pad is selected as proximity pad1"]
    #[inline(always)]
    #[must_use]
    pub fn touch_approach_pad1(&mut self) -> TOUCH_APPROACH_PAD1_W<24> {
        TOUCH_APPROACH_PAD1_W::new(self)
    }
    #[doc = "Bits 28:31 - Indicate which pad is selected as proximity pad0"]
    #[inline(always)]
    #[must_use]
    pub fn touch_approach_pad0(&mut self) -> TOUCH_APPROACH_PAD0_W<28> {
        TOUCH_APPROACH_PAD0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Touch sensor configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_touch_conf](index.html) module"]
pub struct SAR_TOUCH_CONF_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_touch_conf::R](R) reader structure"]
impl crate::Readable for SAR_TOUCH_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_touch_conf::W](W) writer structure"]
impl crate::Writable for SAR_TOUCH_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAR_TOUCH_CONF to value 0xfff0_7fff"]
impl crate::Resettable for SAR_TOUCH_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0xfff0_7fff;
}
