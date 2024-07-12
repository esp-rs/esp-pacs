#[doc = "Register `RF_PWC` reader"]
pub type R = crate::R<RF_PWC_SPEC>;
#[doc = "Register `RF_PWC` writer"]
pub type W = crate::W<RF_PWC_SPEC>;
#[doc = "Field `XPD_PERIF_I2C` reader - need_des"]
pub type XPD_PERIF_I2C_R = crate::BitReader;
#[doc = "Field `XPD_PERIF_I2C` writer - need_des"]
pub type XPD_PERIF_I2C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XPD_RFTX_I2C` reader - need_des"]
pub type XPD_RFTX_I2C_R = crate::BitReader;
#[doc = "Field `XPD_RFTX_I2C` writer - need_des"]
pub type XPD_RFTX_I2C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XPD_RFRX_I2C` reader - need_des"]
pub type XPD_RFRX_I2C_R = crate::BitReader;
#[doc = "Field `XPD_RFRX_I2C` writer - need_des"]
pub type XPD_RFRX_I2C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XPD_RFPLL` reader - need_des"]
pub type XPD_RFPLL_R = crate::BitReader;
#[doc = "Field `XPD_RFPLL` writer - need_des"]
pub type XPD_RFPLL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XPD_FORCE_RFPLL` reader - need_des"]
pub type XPD_FORCE_RFPLL_R = crate::BitReader;
#[doc = "Field `XPD_FORCE_RFPLL` writer - need_des"]
pub type XPD_FORCE_RFPLL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn xpd_perif_i2c(&self) -> XPD_PERIF_I2C_R {
        XPD_PERIF_I2C_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn xpd_rftx_i2c(&self) -> XPD_RFTX_I2C_R {
        XPD_RFTX_I2C_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn xpd_rfrx_i2c(&self) -> XPD_RFRX_I2C_R {
        XPD_RFRX_I2C_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn xpd_rfpll(&self) -> XPD_RFPLL_R {
        XPD_RFPLL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn xpd_force_rfpll(&self) -> XPD_FORCE_RFPLL_R {
        XPD_FORCE_RFPLL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RF_PWC")
            .field("xpd_perif_i2c", &self.xpd_perif_i2c())
            .field("xpd_rftx_i2c", &self.xpd_rftx_i2c())
            .field("xpd_rfrx_i2c", &self.xpd_rfrx_i2c())
            .field("xpd_rfpll", &self.xpd_rfpll())
            .field("xpd_force_rfpll", &self.xpd_force_rfpll())
            .finish()
    }
}
impl W {
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn xpd_perif_i2c(&mut self) -> XPD_PERIF_I2C_W<RF_PWC_SPEC> {
        XPD_PERIF_I2C_W::new(self, 27)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn xpd_rftx_i2c(&mut self) -> XPD_RFTX_I2C_W<RF_PWC_SPEC> {
        XPD_RFTX_I2C_W::new(self, 28)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn xpd_rfrx_i2c(&mut self) -> XPD_RFRX_I2C_W<RF_PWC_SPEC> {
        XPD_RFRX_I2C_W::new(self, 29)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn xpd_rfpll(&mut self) -> XPD_RFPLL_W<RF_PWC_SPEC> {
        XPD_RFPLL_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn xpd_force_rfpll(&mut self) -> XPD_FORCE_RFPLL_W<RF_PWC_SPEC> {
        XPD_FORCE_RFPLL_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_pwc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_pwc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RF_PWC_SPEC;
impl crate::RegisterSpec for RF_PWC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rf_pwc::R`](R) reader structure"]
impl crate::Readable for RF_PWC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rf_pwc::W`](W) writer structure"]
impl crate::Writable for RF_PWC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RF_PWC to value 0x0800_0000"]
impl crate::Resettable for RF_PWC_SPEC {
    const RESET_VALUE: u32 = 0x0800_0000;
}
