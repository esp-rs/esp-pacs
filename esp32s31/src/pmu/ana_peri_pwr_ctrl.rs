#[doc = "Register `ANA_PERI_PWR_CTRL` reader"]
pub type R = crate::R<ANA_PERI_PWR_CTRL_SPEC>;
#[doc = "Register `ANA_PERI_PWR_CTRL` writer"]
pub type W = crate::W<ANA_PERI_PWR_CTRL_SPEC>;
#[doc = "Field `XPD_PERIF_I2C` reader - need_des"]
pub type XPD_PERIF_I2C_R = crate::BitReader;
#[doc = "Field `XPD_PERIF_I2C` writer - need_des"]
pub type XPD_PERIF_I2C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTB_PERIF_I2C` reader - need_des"]
pub type RSTB_PERIF_I2C_R = crate::BitReader;
#[doc = "Field `RSTB_PERIF_I2C` writer - need_des"]
pub type RSTB_PERIF_I2C_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn xpd_perif_i2c(&self) -> XPD_PERIF_I2C_R {
        XPD_PERIF_I2C_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn rstb_perif_i2c(&self) -> RSTB_PERIF_I2C_R {
        RSTB_PERIF_I2C_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ANA_PERI_PWR_CTRL")
            .field("xpd_perif_i2c", &self.xpd_perif_i2c())
            .field("rstb_perif_i2c", &self.rstb_perif_i2c())
            .finish()
    }
}
impl W {
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn xpd_perif_i2c(&mut self) -> XPD_PERIF_I2C_W<'_, ANA_PERI_PWR_CTRL_SPEC> {
        XPD_PERIF_I2C_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn rstb_perif_i2c(&mut self) -> RSTB_PERIF_I2C_W<'_, ANA_PERI_PWR_CTRL_SPEC> {
        RSTB_PERIF_I2C_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_peri_pwr_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_peri_pwr_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ANA_PERI_PWR_CTRL_SPEC;
impl crate::RegisterSpec for ANA_PERI_PWR_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ana_peri_pwr_ctrl::R`](R) reader structure"]
impl crate::Readable for ANA_PERI_PWR_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ana_peri_pwr_ctrl::W`](W) writer structure"]
impl crate::Writable for ANA_PERI_PWR_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ANA_PERI_PWR_CTRL to value 0x4000_0000"]
impl crate::Resettable for ANA_PERI_PWR_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x4000_0000;
}
