#[doc = "Register `HOST_SLCHOST_CONF` reader"]
pub struct R(crate::R<HOST_SLCHOST_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_SLCHOST_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_SLCHOST_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_SLCHOST_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOST_SLCHOST_CONF` writer"]
pub struct W(crate::W<HOST_SLCHOST_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOST_SLCHOST_CONF_SPEC>;
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
impl From<crate::W<HOST_SLCHOST_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOST_SLCHOST_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HOST_FRC_SDIO11` reader - "]
pub type HOST_FRC_SDIO11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HOST_FRC_SDIO11` writer - "]
pub type HOST_FRC_SDIO11_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HOST_SLCHOST_CONF_SPEC, u8, u8, 5, O>;
#[doc = "Field `HOST_FRC_SDIO20` reader - "]
pub type HOST_FRC_SDIO20_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HOST_FRC_SDIO20` writer - "]
pub type HOST_FRC_SDIO20_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HOST_SLCHOST_CONF_SPEC, u8, u8, 5, O>;
#[doc = "Field `HOST_FRC_NEG_SAMP` reader - "]
pub type HOST_FRC_NEG_SAMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HOST_FRC_NEG_SAMP` writer - "]
pub type HOST_FRC_NEG_SAMP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HOST_SLCHOST_CONF_SPEC, u8, u8, 5, O>;
#[doc = "Field `HOST_FRC_POS_SAMP` reader - "]
pub type HOST_FRC_POS_SAMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HOST_FRC_POS_SAMP` writer - "]
pub type HOST_FRC_POS_SAMP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HOST_SLCHOST_CONF_SPEC, u8, u8, 5, O>;
#[doc = "Field `HOST_FRC_QUICK_IN` reader - "]
pub type HOST_FRC_QUICK_IN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HOST_FRC_QUICK_IN` writer - "]
pub type HOST_FRC_QUICK_IN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HOST_SLCHOST_CONF_SPEC, u8, u8, 5, O>;
#[doc = "Field `HOST_SDIO20_INT_DELAY` reader - "]
pub type HOST_SDIO20_INT_DELAY_R = crate::BitReader<bool>;
#[doc = "Field `HOST_SDIO20_INT_DELAY` writer - "]
pub type HOST_SDIO20_INT_DELAY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HOST_SLCHOST_CONF_SPEC, bool, O>;
#[doc = "Field `HOST_SDIO_PAD_PULLUP` reader - "]
pub type HOST_SDIO_PAD_PULLUP_R = crate::BitReader<bool>;
#[doc = "Field `HOST_SDIO_PAD_PULLUP` writer - "]
pub type HOST_SDIO_PAD_PULLUP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HOST_SLCHOST_CONF_SPEC, bool, O>;
#[doc = "Field `HOST_HSPEED_CON_EN` reader - "]
pub type HOST_HSPEED_CON_EN_R = crate::BitReader<bool>;
#[doc = "Field `HOST_HSPEED_CON_EN` writer - "]
pub type HOST_HSPEED_CON_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HOST_SLCHOST_CONF_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn host_frc_sdio11(&self) -> HOST_FRC_SDIO11_R {
        HOST_FRC_SDIO11_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn host_frc_sdio20(&self) -> HOST_FRC_SDIO20_R {
        HOST_FRC_SDIO20_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14"]
    #[inline(always)]
    pub fn host_frc_neg_samp(&self) -> HOST_FRC_NEG_SAMP_R {
        HOST_FRC_NEG_SAMP_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19"]
    #[inline(always)]
    pub fn host_frc_pos_samp(&self) -> HOST_FRC_POS_SAMP_R {
        HOST_FRC_POS_SAMP_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn host_frc_quick_in(&self) -> HOST_FRC_QUICK_IN_R {
        HOST_FRC_QUICK_IN_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn host_sdio20_int_delay(&self) -> HOST_SDIO20_INT_DELAY_R {
        HOST_SDIO20_INT_DELAY_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn host_sdio_pad_pullup(&self) -> HOST_SDIO_PAD_PULLUP_R {
        HOST_SDIO_PAD_PULLUP_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn host_hspeed_con_en(&self) -> HOST_HSPEED_CON_EN_R {
        HOST_HSPEED_CON_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn host_frc_sdio11(&mut self) -> HOST_FRC_SDIO11_W<0> {
        HOST_FRC_SDIO11_W::new(self)
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn host_frc_sdio20(&mut self) -> HOST_FRC_SDIO20_W<5> {
        HOST_FRC_SDIO20_W::new(self)
    }
    #[doc = "Bits 10:14"]
    #[inline(always)]
    pub fn host_frc_neg_samp(&mut self) -> HOST_FRC_NEG_SAMP_W<10> {
        HOST_FRC_NEG_SAMP_W::new(self)
    }
    #[doc = "Bits 15:19"]
    #[inline(always)]
    pub fn host_frc_pos_samp(&mut self) -> HOST_FRC_POS_SAMP_W<15> {
        HOST_FRC_POS_SAMP_W::new(self)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn host_frc_quick_in(&mut self) -> HOST_FRC_QUICK_IN_W<20> {
        HOST_FRC_QUICK_IN_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn host_sdio20_int_delay(&mut self) -> HOST_SDIO20_INT_DELAY_W<25> {
        HOST_SDIO20_INT_DELAY_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn host_sdio_pad_pullup(&mut self) -> HOST_SDIO_PAD_PULLUP_W<26> {
        HOST_SDIO_PAD_PULLUP_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn host_hspeed_con_en(&mut self) -> HOST_HSPEED_CON_EN_W<27> {
        HOST_HSPEED_CON_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_slchost_conf](index.html) module"]
pub struct HOST_SLCHOST_CONF_SPEC;
impl crate::RegisterSpec for HOST_SLCHOST_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_slchost_conf::R](R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [host_slchost_conf::W](W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HOST_SLCHOST_CONF to value 0"]
impl crate::Resettable for HOST_SLCHOST_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
