#[doc = "Register `I2STIMING` reader"]
pub struct R(crate::R<I2STIMING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2STIMING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2STIMING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2STIMING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2STIMING` writer"]
pub struct W(crate::W<I2STIMING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2STIMING_SPEC>;
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
impl From<crate::W<I2STIMING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2STIMING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2S_TRANS_BCK_IN_DELAY` reader - "]
pub type I2S_TRANS_BCK_IN_DELAY_R = crate::FieldReader;
#[doc = "Field `I2S_TRANS_BCK_IN_DELAY` writer - "]
pub type I2S_TRANS_BCK_IN_DELAY_W<'a, const O: u8> = crate::FieldWriter<'a, I2STIMING_SPEC, 2, O>;
#[doc = "Field `I2S_TRANS_WS_IN_DELAY` reader - "]
pub type I2S_TRANS_WS_IN_DELAY_R = crate::FieldReader;
#[doc = "Field `I2S_TRANS_WS_IN_DELAY` writer - "]
pub type I2S_TRANS_WS_IN_DELAY_W<'a, const O: u8> = crate::FieldWriter<'a, I2STIMING_SPEC, 2, O>;
#[doc = "Field `I2S_RECE_BCK_IN_DELAY` reader - "]
pub type I2S_RECE_BCK_IN_DELAY_R = crate::FieldReader;
#[doc = "Field `I2S_RECE_BCK_IN_DELAY` writer - "]
pub type I2S_RECE_BCK_IN_DELAY_W<'a, const O: u8> = crate::FieldWriter<'a, I2STIMING_SPEC, 2, O>;
#[doc = "Field `I2S_RECE_WS_IN_DELAY` reader - "]
pub type I2S_RECE_WS_IN_DELAY_R = crate::FieldReader;
#[doc = "Field `I2S_RECE_WS_IN_DELAY` writer - "]
pub type I2S_RECE_WS_IN_DELAY_W<'a, const O: u8> = crate::FieldWriter<'a, I2STIMING_SPEC, 2, O>;
#[doc = "Field `I2S_RECE_SD_IN_DELAY` reader - "]
pub type I2S_RECE_SD_IN_DELAY_R = crate::FieldReader;
#[doc = "Field `I2S_RECE_SD_IN_DELAY` writer - "]
pub type I2S_RECE_SD_IN_DELAY_W<'a, const O: u8> = crate::FieldWriter<'a, I2STIMING_SPEC, 2, O>;
#[doc = "Field `I2S_TRANS_BCK_OUT_DELAY` reader - "]
pub type I2S_TRANS_BCK_OUT_DELAY_R = crate::FieldReader;
#[doc = "Field `I2S_TRANS_BCK_OUT_DELAY` writer - "]
pub type I2S_TRANS_BCK_OUT_DELAY_W<'a, const O: u8> = crate::FieldWriter<'a, I2STIMING_SPEC, 2, O>;
#[doc = "Field `I2S_TRANS_WS_OUT_DELAY` reader - "]
pub type I2S_TRANS_WS_OUT_DELAY_R = crate::FieldReader;
#[doc = "Field `I2S_TRANS_WS_OUT_DELAY` writer - "]
pub type I2S_TRANS_WS_OUT_DELAY_W<'a, const O: u8> = crate::FieldWriter<'a, I2STIMING_SPEC, 2, O>;
#[doc = "Field `I2S_TRANS_SD_OUT_DELAY` reader - "]
pub type I2S_TRANS_SD_OUT_DELAY_R = crate::FieldReader;
#[doc = "Field `I2S_TRANS_SD_OUT_DELAY` writer - "]
pub type I2S_TRANS_SD_OUT_DELAY_W<'a, const O: u8> = crate::FieldWriter<'a, I2STIMING_SPEC, 2, O>;
#[doc = "Field `I2S_RECE_WS_OUT_DELAY` reader - "]
pub type I2S_RECE_WS_OUT_DELAY_R = crate::FieldReader;
#[doc = "Field `I2S_RECE_WS_OUT_DELAY` writer - "]
pub type I2S_RECE_WS_OUT_DELAY_W<'a, const O: u8> = crate::FieldWriter<'a, I2STIMING_SPEC, 2, O>;
#[doc = "Field `I2S_RECE_BCK_OUT_DELAY` reader - "]
pub type I2S_RECE_BCK_OUT_DELAY_R = crate::FieldReader;
#[doc = "Field `I2S_RECE_BCK_OUT_DELAY` writer - "]
pub type I2S_RECE_BCK_OUT_DELAY_W<'a, const O: u8> = crate::FieldWriter<'a, I2STIMING_SPEC, 2, O>;
#[doc = "Field `I2S_TRANS_DSYNC_SW` reader - "]
pub type I2S_TRANS_DSYNC_SW_R = crate::BitReader;
#[doc = "Field `I2S_TRANS_DSYNC_SW` writer - "]
pub type I2S_TRANS_DSYNC_SW_W<'a, const O: u8> = crate::BitWriter<'a, I2STIMING_SPEC, O>;
#[doc = "Field `I2S_RECE_DSYNC_SW` reader - "]
pub type I2S_RECE_DSYNC_SW_R = crate::BitReader;
#[doc = "Field `I2S_RECE_DSYNC_SW` writer - "]
pub type I2S_RECE_DSYNC_SW_W<'a, const O: u8> = crate::BitWriter<'a, I2STIMING_SPEC, O>;
#[doc = "Field `I2S_TRANS_BCK_IN_INV` reader - "]
pub type I2S_TRANS_BCK_IN_INV_R = crate::BitReader;
#[doc = "Field `I2S_TRANS_BCK_IN_INV` writer - "]
pub type I2S_TRANS_BCK_IN_INV_W<'a, const O: u8> = crate::BitWriter<'a, I2STIMING_SPEC, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn i2s_trans_bck_in_delay(&self) -> I2S_TRANS_BCK_IN_DELAY_R {
        I2S_TRANS_BCK_IN_DELAY_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn i2s_trans_ws_in_delay(&self) -> I2S_TRANS_WS_IN_DELAY_R {
        I2S_TRANS_WS_IN_DELAY_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn i2s_rece_bck_in_delay(&self) -> I2S_RECE_BCK_IN_DELAY_R {
        I2S_RECE_BCK_IN_DELAY_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn i2s_rece_ws_in_delay(&self) -> I2S_RECE_WS_IN_DELAY_R {
        I2S_RECE_WS_IN_DELAY_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn i2s_rece_sd_in_delay(&self) -> I2S_RECE_SD_IN_DELAY_R {
        I2S_RECE_SD_IN_DELAY_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn i2s_trans_bck_out_delay(&self) -> I2S_TRANS_BCK_OUT_DELAY_R {
        I2S_TRANS_BCK_OUT_DELAY_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn i2s_trans_ws_out_delay(&self) -> I2S_TRANS_WS_OUT_DELAY_R {
        I2S_TRANS_WS_OUT_DELAY_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn i2s_trans_sd_out_delay(&self) -> I2S_TRANS_SD_OUT_DELAY_R {
        I2S_TRANS_SD_OUT_DELAY_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn i2s_rece_ws_out_delay(&self) -> I2S_RECE_WS_OUT_DELAY_R {
        I2S_RECE_WS_OUT_DELAY_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn i2s_rece_bck_out_delay(&self) -> I2S_RECE_BCK_OUT_DELAY_R {
        I2S_RECE_BCK_OUT_DELAY_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn i2s_trans_dsync_sw(&self) -> I2S_TRANS_DSYNC_SW_R {
        I2S_TRANS_DSYNC_SW_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn i2s_rece_dsync_sw(&self) -> I2S_RECE_DSYNC_SW_R {
        I2S_RECE_DSYNC_SW_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn i2s_trans_bck_in_inv(&self) -> I2S_TRANS_BCK_IN_INV_R {
        I2S_TRANS_BCK_IN_INV_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2STIMING")
            .field(
                "i2s_trans_bck_in_inv",
                &format_args!("{}", self.i2s_trans_bck_in_inv().bit()),
            )
            .field(
                "i2s_rece_dsync_sw",
                &format_args!("{}", self.i2s_rece_dsync_sw().bit()),
            )
            .field(
                "i2s_trans_dsync_sw",
                &format_args!("{}", self.i2s_trans_dsync_sw().bit()),
            )
            .field(
                "i2s_rece_bck_out_delay",
                &format_args!("{}", self.i2s_rece_bck_out_delay().bits()),
            )
            .field(
                "i2s_rece_ws_out_delay",
                &format_args!("{}", self.i2s_rece_ws_out_delay().bits()),
            )
            .field(
                "i2s_trans_sd_out_delay",
                &format_args!("{}", self.i2s_trans_sd_out_delay().bits()),
            )
            .field(
                "i2s_trans_ws_out_delay",
                &format_args!("{}", self.i2s_trans_ws_out_delay().bits()),
            )
            .field(
                "i2s_trans_bck_out_delay",
                &format_args!("{}", self.i2s_trans_bck_out_delay().bits()),
            )
            .field(
                "i2s_rece_sd_in_delay",
                &format_args!("{}", self.i2s_rece_sd_in_delay().bits()),
            )
            .field(
                "i2s_rece_ws_in_delay",
                &format_args!("{}", self.i2s_rece_ws_in_delay().bits()),
            )
            .field(
                "i2s_rece_bck_in_delay",
                &format_args!("{}", self.i2s_rece_bck_in_delay().bits()),
            )
            .field(
                "i2s_trans_ws_in_delay",
                &format_args!("{}", self.i2s_trans_ws_in_delay().bits()),
            )
            .field(
                "i2s_trans_bck_in_delay",
                &format_args!("{}", self.i2s_trans_bck_in_delay().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<I2STIMING_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_trans_bck_in_delay(&mut self) -> I2S_TRANS_BCK_IN_DELAY_W<0> {
        I2S_TRANS_BCK_IN_DELAY_W::new(self)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_trans_ws_in_delay(&mut self) -> I2S_TRANS_WS_IN_DELAY_W<2> {
        I2S_TRANS_WS_IN_DELAY_W::new(self)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_rece_bck_in_delay(&mut self) -> I2S_RECE_BCK_IN_DELAY_W<4> {
        I2S_RECE_BCK_IN_DELAY_W::new(self)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_rece_ws_in_delay(&mut self) -> I2S_RECE_WS_IN_DELAY_W<6> {
        I2S_RECE_WS_IN_DELAY_W::new(self)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_rece_sd_in_delay(&mut self) -> I2S_RECE_SD_IN_DELAY_W<8> {
        I2S_RECE_SD_IN_DELAY_W::new(self)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_trans_bck_out_delay(&mut self) -> I2S_TRANS_BCK_OUT_DELAY_W<10> {
        I2S_TRANS_BCK_OUT_DELAY_W::new(self)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_trans_ws_out_delay(&mut self) -> I2S_TRANS_WS_OUT_DELAY_W<12> {
        I2S_TRANS_WS_OUT_DELAY_W::new(self)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_trans_sd_out_delay(&mut self) -> I2S_TRANS_SD_OUT_DELAY_W<14> {
        I2S_TRANS_SD_OUT_DELAY_W::new(self)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_rece_ws_out_delay(&mut self) -> I2S_RECE_WS_OUT_DELAY_W<16> {
        I2S_RECE_WS_OUT_DELAY_W::new(self)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_rece_bck_out_delay(&mut self) -> I2S_RECE_BCK_OUT_DELAY_W<18> {
        I2S_RECE_BCK_OUT_DELAY_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_trans_dsync_sw(&mut self) -> I2S_TRANS_DSYNC_SW_W<20> {
        I2S_TRANS_DSYNC_SW_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_rece_dsync_sw(&mut self) -> I2S_RECE_DSYNC_SW_W<21> {
        I2S_RECE_DSYNC_SW_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_trans_bck_in_inv(&mut self) -> I2S_TRANS_BCK_IN_INV_W<22> {
        I2S_TRANS_BCK_IN_INV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2STIMING\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2stiming](index.html) module"]
pub struct I2STIMING_SPEC;
impl crate::RegisterSpec for I2STIMING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2stiming::R](R) reader structure"]
impl crate::Readable for I2STIMING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2stiming::W](W) writer structure"]
impl crate::Writable for I2STIMING_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2STIMING to value 0"]
impl crate::Resettable for I2STIMING_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
