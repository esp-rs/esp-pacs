///Register `SCL_SP_CONF` reader
pub type R = crate::R<SCL_SP_CONF_SPEC>;
///Register `SCL_SP_CONF` writer
pub type W = crate::W<SCL_SP_CONF_SPEC>;
///Field `SCL_RST_SLV_EN` reader - When I2C master is IDLE, set this bit to send out SCL pulses. The number of pulses equals to I2C_SCL_RST_SLV_NUM\[4:0\].
pub type SCL_RST_SLV_EN_R = crate::BitReader;
///Field `SCL_RST_SLV_EN` writer - When I2C master is IDLE, set this bit to send out SCL pulses. The number of pulses equals to I2C_SCL_RST_SLV_NUM\[4:0\].
pub type SCL_RST_SLV_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCL_RST_SLV_NUM` reader - Configure the pulses of SCL generated in I2C master mode. Valid when I2C_SCL_RST_SLV_EN is 1.
pub type SCL_RST_SLV_NUM_R = crate::FieldReader;
///Field `SCL_RST_SLV_NUM` writer - Configure the pulses of SCL generated in I2C master mode. Valid when I2C_SCL_RST_SLV_EN is 1.
pub type SCL_RST_SLV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `SCL_PD_EN` reader - The power down enable bit for the I2C output SCL line. 1: Power down. 0: Not power down. Set I2C_SCL_FORCE_OUT and I2C_SCL_PD_EN to 1 to stretch SCL low.
pub type SCL_PD_EN_R = crate::BitReader;
///Field `SCL_PD_EN` writer - The power down enable bit for the I2C output SCL line. 1: Power down. 0: Not power down. Set I2C_SCL_FORCE_OUT and I2C_SCL_PD_EN to 1 to stretch SCL low.
pub type SCL_PD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDA_PD_EN` reader - The power down enable bit for the I2C output SDA line. 1: Power down. 0: Not power down. Set I2C_SDA_FORCE_OUT and I2C_SDA_PD_EN to 1 to stretch SDA low.
pub type SDA_PD_EN_R = crate::BitReader;
///Field `SDA_PD_EN` writer - The power down enable bit for the I2C output SDA line. 1: Power down. 0: Not power down. Set I2C_SDA_FORCE_OUT and I2C_SDA_PD_EN to 1 to stretch SDA low.
pub type SDA_PD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - When I2C master is IDLE, set this bit to send out SCL pulses. The number of pulses equals to I2C_SCL_RST_SLV_NUM\[4:0\].
    #[inline(always)]
    pub fn scl_rst_slv_en(&self) -> SCL_RST_SLV_EN_R {
        SCL_RST_SLV_EN_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:5 - Configure the pulses of SCL generated in I2C master mode. Valid when I2C_SCL_RST_SLV_EN is 1.
    #[inline(always)]
    pub fn scl_rst_slv_num(&self) -> SCL_RST_SLV_NUM_R {
        SCL_RST_SLV_NUM_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    ///Bit 6 - The power down enable bit for the I2C output SCL line. 1: Power down. 0: Not power down. Set I2C_SCL_FORCE_OUT and I2C_SCL_PD_EN to 1 to stretch SCL low.
    #[inline(always)]
    pub fn scl_pd_en(&self) -> SCL_PD_EN_R {
        SCL_PD_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - The power down enable bit for the I2C output SDA line. 1: Power down. 0: Not power down. Set I2C_SDA_FORCE_OUT and I2C_SDA_PD_EN to 1 to stretch SDA low.
    #[inline(always)]
    pub fn sda_pd_en(&self) -> SDA_PD_EN_R {
        SDA_PD_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCL_SP_CONF")
            .field("scl_rst_slv_en", &self.scl_rst_slv_en())
            .field("scl_rst_slv_num", &self.scl_rst_slv_num())
            .field("scl_pd_en", &self.scl_pd_en())
            .field("sda_pd_en", &self.sda_pd_en())
            .finish()
    }
}
impl W {
    ///Bit 0 - When I2C master is IDLE, set this bit to send out SCL pulses. The number of pulses equals to I2C_SCL_RST_SLV_NUM\[4:0\].
    #[inline(always)]
    #[must_use]
    pub fn scl_rst_slv_en(&mut self) -> SCL_RST_SLV_EN_W<SCL_SP_CONF_SPEC> {
        SCL_RST_SLV_EN_W::new(self, 0)
    }
    ///Bits 1:5 - Configure the pulses of SCL generated in I2C master mode. Valid when I2C_SCL_RST_SLV_EN is 1.
    #[inline(always)]
    #[must_use]
    pub fn scl_rst_slv_num(&mut self) -> SCL_RST_SLV_NUM_W<SCL_SP_CONF_SPEC> {
        SCL_RST_SLV_NUM_W::new(self, 1)
    }
    ///Bit 6 - The power down enable bit for the I2C output SCL line. 1: Power down. 0: Not power down. Set I2C_SCL_FORCE_OUT and I2C_SCL_PD_EN to 1 to stretch SCL low.
    #[inline(always)]
    #[must_use]
    pub fn scl_pd_en(&mut self) -> SCL_PD_EN_W<SCL_SP_CONF_SPEC> {
        SCL_PD_EN_W::new(self, 6)
    }
    ///Bit 7 - The power down enable bit for the I2C output SDA line. 1: Power down. 0: Not power down. Set I2C_SDA_FORCE_OUT and I2C_SDA_PD_EN to 1 to stretch SDA low.
    #[inline(always)]
    #[must_use]
    pub fn sda_pd_en(&mut self) -> SDA_PD_EN_W<SCL_SP_CONF_SPEC> {
        SDA_PD_EN_W::new(self, 7)
    }
}
/**Power configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`scl_sp_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_sp_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SCL_SP_CONF_SPEC;
impl crate::RegisterSpec for SCL_SP_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`scl_sp_conf::R`](R) reader structure
impl crate::Readable for SCL_SP_CONF_SPEC {}
///`write(|w| ..)` method takes [`scl_sp_conf::W`](W) writer structure
impl crate::Writable for SCL_SP_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SCL_SP_CONF to value 0
impl crate::Resettable for SCL_SP_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
