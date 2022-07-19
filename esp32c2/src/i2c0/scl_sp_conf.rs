#[doc = "Register `SCL_SP_CONF` reader"]
pub struct R(crate::R<SCL_SP_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCL_SP_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCL_SP_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCL_SP_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCL_SP_CONF` writer"]
pub struct W(crate::W<SCL_SP_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCL_SP_CONF_SPEC>;
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
impl From<crate::W<SCL_SP_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCL_SP_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCL_RST_SLV_EN` reader - When I2C master is IDLE, set this bit to send out SCL pulses. The number of pulses equals to reg_scl_rst_slv_num\\[4:0\\]."]
pub type SCL_RST_SLV_EN_R = crate::BitReader<bool>;
#[doc = "Field `SCL_RST_SLV_EN` writer - When I2C master is IDLE, set this bit to send out SCL pulses. The number of pulses equals to reg_scl_rst_slv_num\\[4:0\\]."]
pub type SCL_RST_SLV_EN_W<'a> = crate::BitWriter<'a, u32, SCL_SP_CONF_SPEC, bool, 0>;
#[doc = "Field `SCL_RST_SLV_NUM` reader - Configure the pulses of SCL generated in I2C master mode. Valid when reg_scl_rst_slv_en is 1."]
pub type SCL_RST_SLV_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCL_RST_SLV_NUM` writer - Configure the pulses of SCL generated in I2C master mode. Valid when reg_scl_rst_slv_en is 1."]
pub type SCL_RST_SLV_NUM_W<'a> = crate::FieldWriter<'a, u32, SCL_SP_CONF_SPEC, u8, u8, 5, 1>;
#[doc = "Field `SCL_PD_EN` reader - The power down enable bit for the I2C output SCL line. 1: Power down. 0: Not power down. Set reg_scl_force_out and reg_scl_pd_en to 1 to stretch SCL low."]
pub type SCL_PD_EN_R = crate::BitReader<bool>;
#[doc = "Field `SCL_PD_EN` writer - The power down enable bit for the I2C output SCL line. 1: Power down. 0: Not power down. Set reg_scl_force_out and reg_scl_pd_en to 1 to stretch SCL low."]
pub type SCL_PD_EN_W<'a> = crate::BitWriter<'a, u32, SCL_SP_CONF_SPEC, bool, 6>;
#[doc = "Field `SDA_PD_EN` reader - The power down enable bit for the I2C output SDA line. 1: Power down. 0: Not power down. Set reg_sda_force_out and reg_sda_pd_en to 1 to stretch SDA low."]
pub type SDA_PD_EN_R = crate::BitReader<bool>;
#[doc = "Field `SDA_PD_EN` writer - The power down enable bit for the I2C output SDA line. 1: Power down. 0: Not power down. Set reg_sda_force_out and reg_sda_pd_en to 1 to stretch SDA low."]
pub type SDA_PD_EN_W<'a> = crate::BitWriter<'a, u32, SCL_SP_CONF_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - When I2C master is IDLE, set this bit to send out SCL pulses. The number of pulses equals to reg_scl_rst_slv_num\\[4:0\\]."]
    #[inline(always)]
    pub fn scl_rst_slv_en(&self) -> SCL_RST_SLV_EN_R {
        SCL_RST_SLV_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - Configure the pulses of SCL generated in I2C master mode. Valid when reg_scl_rst_slv_en is 1."]
    #[inline(always)]
    pub fn scl_rst_slv_num(&self) -> SCL_RST_SLV_NUM_R {
        SCL_RST_SLV_NUM_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bit 6 - The power down enable bit for the I2C output SCL line. 1: Power down. 0: Not power down. Set reg_scl_force_out and reg_scl_pd_en to 1 to stretch SCL low."]
    #[inline(always)]
    pub fn scl_pd_en(&self) -> SCL_PD_EN_R {
        SCL_PD_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The power down enable bit for the I2C output SDA line. 1: Power down. 0: Not power down. Set reg_sda_force_out and reg_sda_pd_en to 1 to stretch SDA low."]
    #[inline(always)]
    pub fn sda_pd_en(&self) -> SDA_PD_EN_R {
        SDA_PD_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When I2C master is IDLE, set this bit to send out SCL pulses. The number of pulses equals to reg_scl_rst_slv_num\\[4:0\\]."]
    #[inline(always)]
    pub fn scl_rst_slv_en(&mut self) -> SCL_RST_SLV_EN_W {
        SCL_RST_SLV_EN_W::new(self)
    }
    #[doc = "Bits 1:5 - Configure the pulses of SCL generated in I2C master mode. Valid when reg_scl_rst_slv_en is 1."]
    #[inline(always)]
    pub fn scl_rst_slv_num(&mut self) -> SCL_RST_SLV_NUM_W {
        SCL_RST_SLV_NUM_W::new(self)
    }
    #[doc = "Bit 6 - The power down enable bit for the I2C output SCL line. 1: Power down. 0: Not power down. Set reg_scl_force_out and reg_scl_pd_en to 1 to stretch SCL low."]
    #[inline(always)]
    pub fn scl_pd_en(&mut self) -> SCL_PD_EN_W {
        SCL_PD_EN_W::new(self)
    }
    #[doc = "Bit 7 - The power down enable bit for the I2C output SDA line. 1: Power down. 0: Not power down. Set reg_sda_force_out and reg_sda_pd_en to 1 to stretch SDA low."]
    #[inline(always)]
    pub fn sda_pd_en(&mut self) -> SDA_PD_EN_W {
        SDA_PD_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scl_sp_conf](index.html) module"]
pub struct SCL_SP_CONF_SPEC;
impl crate::RegisterSpec for SCL_SP_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scl_sp_conf::R](R) reader structure"]
impl crate::Readable for SCL_SP_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scl_sp_conf::W](W) writer structure"]
impl crate::Writable for SCL_SP_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCL_SP_CONF to value 0"]
impl crate::Resettable for SCL_SP_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
