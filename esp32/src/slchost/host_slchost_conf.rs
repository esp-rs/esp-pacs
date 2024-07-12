#[doc = "Register `HOST_SLCHOST_CONF` reader"]
pub type R = crate::R<HOST_SLCHOST_CONF_SPEC>;
#[doc = "Register `HOST_SLCHOST_CONF` writer"]
pub type W = crate::W<HOST_SLCHOST_CONF_SPEC>;
#[doc = "Field `HOST_FRC_SDIO11` reader - "]
pub type HOST_FRC_SDIO11_R = crate::FieldReader;
#[doc = "Field `HOST_FRC_SDIO11` writer - "]
pub type HOST_FRC_SDIO11_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `HOST_FRC_SDIO20` reader - "]
pub type HOST_FRC_SDIO20_R = crate::FieldReader;
#[doc = "Field `HOST_FRC_SDIO20` writer - "]
pub type HOST_FRC_SDIO20_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `HOST_FRC_NEG_SAMP` reader - "]
pub type HOST_FRC_NEG_SAMP_R = crate::FieldReader;
#[doc = "Field `HOST_FRC_NEG_SAMP` writer - "]
pub type HOST_FRC_NEG_SAMP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `HOST_FRC_POS_SAMP` reader - "]
pub type HOST_FRC_POS_SAMP_R = crate::FieldReader;
#[doc = "Field `HOST_FRC_POS_SAMP` writer - "]
pub type HOST_FRC_POS_SAMP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `HOST_FRC_QUICK_IN` reader - "]
pub type HOST_FRC_QUICK_IN_R = crate::FieldReader;
#[doc = "Field `HOST_FRC_QUICK_IN` writer - "]
pub type HOST_FRC_QUICK_IN_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `HOST_SDIO20_INT_DELAY` reader - "]
pub type HOST_SDIO20_INT_DELAY_R = crate::BitReader;
#[doc = "Field `HOST_SDIO20_INT_DELAY` writer - "]
pub type HOST_SDIO20_INT_DELAY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST_SDIO_PAD_PULLUP` reader - "]
pub type HOST_SDIO_PAD_PULLUP_R = crate::BitReader;
#[doc = "Field `HOST_SDIO_PAD_PULLUP` writer - "]
pub type HOST_SDIO_PAD_PULLUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST_HSPEED_CON_EN` reader - "]
pub type HOST_HSPEED_CON_EN_R = crate::BitReader;
#[doc = "Field `HOST_HSPEED_CON_EN` writer - "]
pub type HOST_HSPEED_CON_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_SLCHOST_CONF")
            .field("host_frc_sdio11", &self.host_frc_sdio11())
            .field("host_frc_sdio20", &self.host_frc_sdio20())
            .field("host_frc_neg_samp", &self.host_frc_neg_samp())
            .field("host_frc_pos_samp", &self.host_frc_pos_samp())
            .field("host_frc_quick_in", &self.host_frc_quick_in())
            .field("host_sdio20_int_delay", &self.host_sdio20_int_delay())
            .field("host_sdio_pad_pullup", &self.host_sdio_pad_pullup())
            .field("host_hspeed_con_en", &self.host_hspeed_con_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    #[must_use]
    pub fn host_frc_sdio11(&mut self) -> HOST_FRC_SDIO11_W<HOST_SLCHOST_CONF_SPEC> {
        HOST_FRC_SDIO11_W::new(self, 0)
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    #[must_use]
    pub fn host_frc_sdio20(&mut self) -> HOST_FRC_SDIO20_W<HOST_SLCHOST_CONF_SPEC> {
        HOST_FRC_SDIO20_W::new(self, 5)
    }
    #[doc = "Bits 10:14"]
    #[inline(always)]
    #[must_use]
    pub fn host_frc_neg_samp(&mut self) -> HOST_FRC_NEG_SAMP_W<HOST_SLCHOST_CONF_SPEC> {
        HOST_FRC_NEG_SAMP_W::new(self, 10)
    }
    #[doc = "Bits 15:19"]
    #[inline(always)]
    #[must_use]
    pub fn host_frc_pos_samp(&mut self) -> HOST_FRC_POS_SAMP_W<HOST_SLCHOST_CONF_SPEC> {
        HOST_FRC_POS_SAMP_W::new(self, 15)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    #[must_use]
    pub fn host_frc_quick_in(&mut self) -> HOST_FRC_QUICK_IN_W<HOST_SLCHOST_CONF_SPEC> {
        HOST_FRC_QUICK_IN_W::new(self, 20)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn host_sdio20_int_delay(&mut self) -> HOST_SDIO20_INT_DELAY_W<HOST_SLCHOST_CONF_SPEC> {
        HOST_SDIO20_INT_DELAY_W::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn host_sdio_pad_pullup(&mut self) -> HOST_SDIO_PAD_PULLUP_W<HOST_SLCHOST_CONF_SPEC> {
        HOST_SDIO_PAD_PULLUP_W::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn host_hspeed_con_en(&mut self) -> HOST_HSPEED_CON_EN_W<HOST_SLCHOST_CONF_SPEC> {
        HOST_HSPEED_CON_EN_W::new(self, 27)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`host_slchost_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_slchost_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_SLCHOST_CONF_SPEC;
impl crate::RegisterSpec for HOST_SLCHOST_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_slchost_conf::R`](R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`host_slchost_conf::W`](W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOST_SLCHOST_CONF to value 0"]
impl crate::Resettable for HOST_SLCHOST_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
