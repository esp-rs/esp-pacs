#[doc = "Register `I2STIMING` reader"]
pub type R = crate::R<I2STIMING_SPEC>;
#[doc = "Register `I2STIMING` writer"]
pub type W = crate::W<I2STIMING_SPEC>;
#[doc = "Field `I2S_TRANS_BCK_IN_DELAY` reader - "]
pub type I2S_TRANS_BCK_IN_DELAY_R = crate::FieldReader;
#[doc = "Field `I2S_TRANS_BCK_IN_DELAY` writer - "]
pub type I2S_TRANS_BCK_IN_DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2S_TRANS_WS_IN_DELAY` reader - "]
pub type I2S_TRANS_WS_IN_DELAY_R = crate::FieldReader;
#[doc = "Field `I2S_TRANS_WS_IN_DELAY` writer - "]
pub type I2S_TRANS_WS_IN_DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2S_RECE_BCK_IN_DELAY` reader - "]
pub type I2S_RECE_BCK_IN_DELAY_R = crate::FieldReader;
#[doc = "Field `I2S_RECE_BCK_IN_DELAY` writer - "]
pub type I2S_RECE_BCK_IN_DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2S_RECE_WS_IN_DELAY` reader - "]
pub type I2S_RECE_WS_IN_DELAY_R = crate::FieldReader;
#[doc = "Field `I2S_RECE_WS_IN_DELAY` writer - "]
pub type I2S_RECE_WS_IN_DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2S_RECE_SD_IN_DELAY` reader - "]
pub type I2S_RECE_SD_IN_DELAY_R = crate::FieldReader;
#[doc = "Field `I2S_RECE_SD_IN_DELAY` writer - "]
pub type I2S_RECE_SD_IN_DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2S_TRANS_BCK_OUT_DELAY` reader - "]
pub type I2S_TRANS_BCK_OUT_DELAY_R = crate::FieldReader;
#[doc = "Field `I2S_TRANS_BCK_OUT_DELAY` writer - "]
pub type I2S_TRANS_BCK_OUT_DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2S_TRANS_WS_OUT_DELAY` reader - "]
pub type I2S_TRANS_WS_OUT_DELAY_R = crate::FieldReader;
#[doc = "Field `I2S_TRANS_WS_OUT_DELAY` writer - "]
pub type I2S_TRANS_WS_OUT_DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2S_TRANS_SD_OUT_DELAY` reader - "]
pub type I2S_TRANS_SD_OUT_DELAY_R = crate::FieldReader;
#[doc = "Field `I2S_TRANS_SD_OUT_DELAY` writer - "]
pub type I2S_TRANS_SD_OUT_DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2S_RECE_WS_OUT_DELAY` reader - "]
pub type I2S_RECE_WS_OUT_DELAY_R = crate::FieldReader;
#[doc = "Field `I2S_RECE_WS_OUT_DELAY` writer - "]
pub type I2S_RECE_WS_OUT_DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2S_RECE_BCK_OUT_DELAY` reader - "]
pub type I2S_RECE_BCK_OUT_DELAY_R = crate::FieldReader;
#[doc = "Field `I2S_RECE_BCK_OUT_DELAY` writer - "]
pub type I2S_RECE_BCK_OUT_DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2S_TRANS_DSYNC_SW` reader - "]
pub type I2S_TRANS_DSYNC_SW_R = crate::BitReader;
#[doc = "Field `I2S_TRANS_DSYNC_SW` writer - "]
pub type I2S_TRANS_DSYNC_SW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S_RECE_DSYNC_SW` reader - "]
pub type I2S_RECE_DSYNC_SW_R = crate::BitReader;
#[doc = "Field `I2S_RECE_DSYNC_SW` writer - "]
pub type I2S_RECE_DSYNC_SW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S_TRANS_BCK_IN_INV` reader - "]
pub type I2S_TRANS_BCK_IN_INV_R = crate::BitReader;
#[doc = "Field `I2S_TRANS_BCK_IN_INV` writer - "]
pub type I2S_TRANS_BCK_IN_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
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
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_trans_bck_in_delay(&mut self) -> I2S_TRANS_BCK_IN_DELAY_W<I2STIMING_SPEC> {
        I2S_TRANS_BCK_IN_DELAY_W::new(self, 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_trans_ws_in_delay(&mut self) -> I2S_TRANS_WS_IN_DELAY_W<I2STIMING_SPEC> {
        I2S_TRANS_WS_IN_DELAY_W::new(self, 2)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_rece_bck_in_delay(&mut self) -> I2S_RECE_BCK_IN_DELAY_W<I2STIMING_SPEC> {
        I2S_RECE_BCK_IN_DELAY_W::new(self, 4)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_rece_ws_in_delay(&mut self) -> I2S_RECE_WS_IN_DELAY_W<I2STIMING_SPEC> {
        I2S_RECE_WS_IN_DELAY_W::new(self, 6)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_rece_sd_in_delay(&mut self) -> I2S_RECE_SD_IN_DELAY_W<I2STIMING_SPEC> {
        I2S_RECE_SD_IN_DELAY_W::new(self, 8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_trans_bck_out_delay(&mut self) -> I2S_TRANS_BCK_OUT_DELAY_W<I2STIMING_SPEC> {
        I2S_TRANS_BCK_OUT_DELAY_W::new(self, 10)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_trans_ws_out_delay(&mut self) -> I2S_TRANS_WS_OUT_DELAY_W<I2STIMING_SPEC> {
        I2S_TRANS_WS_OUT_DELAY_W::new(self, 12)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_trans_sd_out_delay(&mut self) -> I2S_TRANS_SD_OUT_DELAY_W<I2STIMING_SPEC> {
        I2S_TRANS_SD_OUT_DELAY_W::new(self, 14)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_rece_ws_out_delay(&mut self) -> I2S_RECE_WS_OUT_DELAY_W<I2STIMING_SPEC> {
        I2S_RECE_WS_OUT_DELAY_W::new(self, 16)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_rece_bck_out_delay(&mut self) -> I2S_RECE_BCK_OUT_DELAY_W<I2STIMING_SPEC> {
        I2S_RECE_BCK_OUT_DELAY_W::new(self, 18)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_trans_dsync_sw(&mut self) -> I2S_TRANS_DSYNC_SW_W<I2STIMING_SPEC> {
        I2S_TRANS_DSYNC_SW_W::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_rece_dsync_sw(&mut self) -> I2S_RECE_DSYNC_SW_W<I2STIMING_SPEC> {
        I2S_RECE_DSYNC_SW_W::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_trans_bck_in_inv(&mut self) -> I2S_TRANS_BCK_IN_INV_W<I2STIMING_SPEC> {
        I2S_TRANS_BCK_IN_INV_W::new(self, 22)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "I2STIMING\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2stiming::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2stiming::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2STIMING_SPEC;
impl crate::RegisterSpec for I2STIMING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2stiming::R`](R) reader structure"]
impl crate::Readable for I2STIMING_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2stiming::W`](W) writer structure"]
impl crate::Writable for I2STIMING_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2STIMING to value 0"]
impl crate::Resettable for I2STIMING_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
