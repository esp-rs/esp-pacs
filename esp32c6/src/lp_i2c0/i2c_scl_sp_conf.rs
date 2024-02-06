#[doc = "Register `I2C_SCL_SP_CONF` reader"]
pub type R = crate::R<I2C_SCL_SP_CONF_SPEC>;
#[doc = "Register `I2C_SCL_SP_CONF` writer"]
pub type W = crate::W<I2C_SCL_SP_CONF_SPEC>;
#[doc = "Field `I2C_SCL_RST_SLV_EN` reader - When I2C master is IDLE, set this bit to send out SCL pulses. The number of pulses equals to reg_scl_rst_slv_num\\[4:0\\]."]
pub type I2C_SCL_RST_SLV_EN_R = crate::BitReader;
#[doc = "Field `I2C_SCL_RST_SLV_EN` writer - When I2C master is IDLE, set this bit to send out SCL pulses. The number of pulses equals to reg_scl_rst_slv_num\\[4:0\\]."]
pub type I2C_SCL_RST_SLV_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_SCL_RST_SLV_NUM` reader - Configure the pulses of SCL generated in I2C master mode. Valid when reg_scl_rst_slv_en is 1."]
pub type I2C_SCL_RST_SLV_NUM_R = crate::FieldReader;
#[doc = "Field `I2C_SCL_RST_SLV_NUM` writer - Configure the pulses of SCL generated in I2C master mode. Valid when reg_scl_rst_slv_en is 1."]
pub type I2C_SCL_RST_SLV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `I2C_SCL_PD_EN` reader - The power down enable bit for the I2C output SCL line. 1: Power down. 0: Not power down. Set reg_scl_force_out and reg_scl_pd_en to 1 to stretch SCL low."]
pub type I2C_SCL_PD_EN_R = crate::BitReader;
#[doc = "Field `I2C_SCL_PD_EN` writer - The power down enable bit for the I2C output SCL line. 1: Power down. 0: Not power down. Set reg_scl_force_out and reg_scl_pd_en to 1 to stretch SCL low."]
pub type I2C_SCL_PD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_SDA_PD_EN` reader - The power down enable bit for the I2C output SDA line. 1: Power down. 0: Not power down. Set reg_sda_force_out and reg_sda_pd_en to 1 to stretch SDA low."]
pub type I2C_SDA_PD_EN_R = crate::BitReader;
#[doc = "Field `I2C_SDA_PD_EN` writer - The power down enable bit for the I2C output SDA line. 1: Power down. 0: Not power down. Set reg_sda_force_out and reg_sda_pd_en to 1 to stretch SDA low."]
pub type I2C_SDA_PD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When I2C master is IDLE, set this bit to send out SCL pulses. The number of pulses equals to reg_scl_rst_slv_num\\[4:0\\]."]
    #[inline(always)]
    pub fn i2c_scl_rst_slv_en(&self) -> I2C_SCL_RST_SLV_EN_R {
        I2C_SCL_RST_SLV_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - Configure the pulses of SCL generated in I2C master mode. Valid when reg_scl_rst_slv_en is 1."]
    #[inline(always)]
    pub fn i2c_scl_rst_slv_num(&self) -> I2C_SCL_RST_SLV_NUM_R {
        I2C_SCL_RST_SLV_NUM_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bit 6 - The power down enable bit for the I2C output SCL line. 1: Power down. 0: Not power down. Set reg_scl_force_out and reg_scl_pd_en to 1 to stretch SCL low."]
    #[inline(always)]
    pub fn i2c_scl_pd_en(&self) -> I2C_SCL_PD_EN_R {
        I2C_SCL_PD_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The power down enable bit for the I2C output SDA line. 1: Power down. 0: Not power down. Set reg_sda_force_out and reg_sda_pd_en to 1 to stretch SDA low."]
    #[inline(always)]
    pub fn i2c_sda_pd_en(&self) -> I2C_SDA_PD_EN_R {
        I2C_SDA_PD_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_SCL_SP_CONF")
            .field(
                "i2c_scl_rst_slv_en",
                &format_args!("{}", self.i2c_scl_rst_slv_en().bit()),
            )
            .field(
                "i2c_scl_rst_slv_num",
                &format_args!("{}", self.i2c_scl_rst_slv_num().bits()),
            )
            .field(
                "i2c_scl_pd_en",
                &format_args!("{}", self.i2c_scl_pd_en().bit()),
            )
            .field(
                "i2c_sda_pd_en",
                &format_args!("{}", self.i2c_sda_pd_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<I2C_SCL_SP_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - When I2C master is IDLE, set this bit to send out SCL pulses. The number of pulses equals to reg_scl_rst_slv_num\\[4:0\\]."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_scl_rst_slv_en(&mut self) -> I2C_SCL_RST_SLV_EN_W<I2C_SCL_SP_CONF_SPEC> {
        I2C_SCL_RST_SLV_EN_W::new(self, 0)
    }
    #[doc = "Bits 1:5 - Configure the pulses of SCL generated in I2C master mode. Valid when reg_scl_rst_slv_en is 1."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_scl_rst_slv_num(&mut self) -> I2C_SCL_RST_SLV_NUM_W<I2C_SCL_SP_CONF_SPEC> {
        I2C_SCL_RST_SLV_NUM_W::new(self, 1)
    }
    #[doc = "Bit 6 - The power down enable bit for the I2C output SCL line. 1: Power down. 0: Not power down. Set reg_scl_force_out and reg_scl_pd_en to 1 to stretch SCL low."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_scl_pd_en(&mut self) -> I2C_SCL_PD_EN_W<I2C_SCL_SP_CONF_SPEC> {
        I2C_SCL_PD_EN_W::new(self, 6)
    }
    #[doc = "Bit 7 - The power down enable bit for the I2C output SDA line. 1: Power down. 0: Not power down. Set reg_sda_force_out and reg_sda_pd_en to 1 to stretch SDA low."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_sda_pd_en(&mut self) -> I2C_SDA_PD_EN_W<I2C_SCL_SP_CONF_SPEC> {
        I2C_SDA_PD_EN_W::new(self, 7)
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
#[doc = "Power configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_scl_sp_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_scl_sp_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C_SCL_SP_CONF_SPEC;
impl crate::RegisterSpec for I2C_SCL_SP_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_scl_sp_conf::R`](R) reader structure"]
impl crate::Readable for I2C_SCL_SP_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2c_scl_sp_conf::W`](W) writer structure"]
impl crate::Writable for I2C_SCL_SP_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2C_SCL_SP_CONF to value 0"]
impl crate::Resettable for I2C_SCL_SP_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
