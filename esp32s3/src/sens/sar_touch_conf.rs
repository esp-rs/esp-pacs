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
#[doc = "Field `SAR_TOUCH_OUTEN` reader - touch controller output enable"]
pub type SAR_TOUCH_OUTEN_R = crate::FieldReader<u16>;
#[doc = "Field `SAR_TOUCH_OUTEN` writer - touch controller output enable"]
pub type SAR_TOUCH_OUTEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, SAR_TOUCH_CONF_SPEC, 15, O, u16>;
#[doc = "Field `SAR_TOUCH_STATUS_CLR` writer - clear all touch active status"]
pub type SAR_TOUCH_STATUS_CLR_W<'a, const O: u8> = crate::BitWriter<'a, SAR_TOUCH_CONF_SPEC, O>;
#[doc = "Field `SAR_TOUCH_DATA_SEL` reader - 3: smooth data 2: baseline 1,0: raw_data"]
pub type SAR_TOUCH_DATA_SEL_R = crate::FieldReader;
#[doc = "Field `SAR_TOUCH_DATA_SEL` writer - 3: smooth data 2: baseline 1,0: raw_data"]
pub type SAR_TOUCH_DATA_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, SAR_TOUCH_CONF_SPEC, 2, O>;
#[doc = "Field `SAR_TOUCH_DENOISE_END` reader - touch_denoise_done"]
pub type SAR_TOUCH_DENOISE_END_R = crate::BitReader;
#[doc = "Field `SAR_TOUCH_UNIT_END` reader - touch_unit_done"]
pub type SAR_TOUCH_UNIT_END_R = crate::BitReader;
#[doc = "Field `SAR_TOUCH_APPROACH_PAD2` reader - indicate which pad is approach pad2"]
pub type SAR_TOUCH_APPROACH_PAD2_R = crate::FieldReader;
#[doc = "Field `SAR_TOUCH_APPROACH_PAD2` writer - indicate which pad is approach pad2"]
pub type SAR_TOUCH_APPROACH_PAD2_W<'a, const O: u8> =
    crate::FieldWriter<'a, SAR_TOUCH_CONF_SPEC, 4, O>;
#[doc = "Field `SAR_TOUCH_APPROACH_PAD1` reader - indicate which pad is approach pad1"]
pub type SAR_TOUCH_APPROACH_PAD1_R = crate::FieldReader;
#[doc = "Field `SAR_TOUCH_APPROACH_PAD1` writer - indicate which pad is approach pad1"]
pub type SAR_TOUCH_APPROACH_PAD1_W<'a, const O: u8> =
    crate::FieldWriter<'a, SAR_TOUCH_CONF_SPEC, 4, O>;
#[doc = "Field `SAR_TOUCH_APPROACH_PAD0` reader - indicate which pad is approach pad0"]
pub type SAR_TOUCH_APPROACH_PAD0_R = crate::FieldReader;
#[doc = "Field `SAR_TOUCH_APPROACH_PAD0` writer - indicate which pad is approach pad0"]
pub type SAR_TOUCH_APPROACH_PAD0_W<'a, const O: u8> =
    crate::FieldWriter<'a, SAR_TOUCH_CONF_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:14 - touch controller output enable"]
    #[inline(always)]
    pub fn sar_touch_outen(&self) -> SAR_TOUCH_OUTEN_R {
        SAR_TOUCH_OUTEN_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:17 - 3: smooth data 2: baseline 1,0: raw_data"]
    #[inline(always)]
    pub fn sar_touch_data_sel(&self) -> SAR_TOUCH_DATA_SEL_R {
        SAR_TOUCH_DATA_SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - touch_denoise_done"]
    #[inline(always)]
    pub fn sar_touch_denoise_end(&self) -> SAR_TOUCH_DENOISE_END_R {
        SAR_TOUCH_DENOISE_END_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - touch_unit_done"]
    #[inline(always)]
    pub fn sar_touch_unit_end(&self) -> SAR_TOUCH_UNIT_END_R {
        SAR_TOUCH_UNIT_END_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - indicate which pad is approach pad2"]
    #[inline(always)]
    pub fn sar_touch_approach_pad2(&self) -> SAR_TOUCH_APPROACH_PAD2_R {
        SAR_TOUCH_APPROACH_PAD2_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - indicate which pad is approach pad1"]
    #[inline(always)]
    pub fn sar_touch_approach_pad1(&self) -> SAR_TOUCH_APPROACH_PAD1_R {
        SAR_TOUCH_APPROACH_PAD1_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - indicate which pad is approach pad0"]
    #[inline(always)]
    pub fn sar_touch_approach_pad0(&self) -> SAR_TOUCH_APPROACH_PAD0_R {
        SAR_TOUCH_APPROACH_PAD0_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_TOUCH_CONF")
            .field(
                "sar_touch_outen",
                &format_args!("{}", self.sar_touch_outen().bits()),
            )
            .field(
                "sar_touch_data_sel",
                &format_args!("{}", self.sar_touch_data_sel().bits()),
            )
            .field(
                "sar_touch_denoise_end",
                &format_args!("{}", self.sar_touch_denoise_end().bit()),
            )
            .field(
                "sar_touch_unit_end",
                &format_args!("{}", self.sar_touch_unit_end().bit()),
            )
            .field(
                "sar_touch_approach_pad2",
                &format_args!("{}", self.sar_touch_approach_pad2().bits()),
            )
            .field(
                "sar_touch_approach_pad1",
                &format_args!("{}", self.sar_touch_approach_pad1().bits()),
            )
            .field(
                "sar_touch_approach_pad0",
                &format_args!("{}", self.sar_touch_approach_pad0().bits()),
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
    #[doc = "Bits 0:14 - touch controller output enable"]
    #[inline(always)]
    #[must_use]
    pub fn sar_touch_outen(&mut self) -> SAR_TOUCH_OUTEN_W<0> {
        SAR_TOUCH_OUTEN_W::new(self)
    }
    #[doc = "Bit 15 - clear all touch active status"]
    #[inline(always)]
    #[must_use]
    pub fn sar_touch_status_clr(&mut self) -> SAR_TOUCH_STATUS_CLR_W<15> {
        SAR_TOUCH_STATUS_CLR_W::new(self)
    }
    #[doc = "Bits 16:17 - 3: smooth data 2: baseline 1,0: raw_data"]
    #[inline(always)]
    #[must_use]
    pub fn sar_touch_data_sel(&mut self) -> SAR_TOUCH_DATA_SEL_W<16> {
        SAR_TOUCH_DATA_SEL_W::new(self)
    }
    #[doc = "Bits 20:23 - indicate which pad is approach pad2"]
    #[inline(always)]
    #[must_use]
    pub fn sar_touch_approach_pad2(&mut self) -> SAR_TOUCH_APPROACH_PAD2_W<20> {
        SAR_TOUCH_APPROACH_PAD2_W::new(self)
    }
    #[doc = "Bits 24:27 - indicate which pad is approach pad1"]
    #[inline(always)]
    #[must_use]
    pub fn sar_touch_approach_pad1(&mut self) -> SAR_TOUCH_APPROACH_PAD1_W<24> {
        SAR_TOUCH_APPROACH_PAD1_W::new(self)
    }
    #[doc = "Bits 28:31 - indicate which pad is approach pad0"]
    #[inline(always)]
    #[must_use]
    pub fn sar_touch_approach_pad0(&mut self) -> SAR_TOUCH_APPROACH_PAD0_W<28> {
        SAR_TOUCH_APPROACH_PAD0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure touch controller\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_touch_conf](index.html) module"]
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
