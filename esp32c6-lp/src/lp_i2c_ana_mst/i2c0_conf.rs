#[doc = "Register `I2C0_CONF` reader"]
pub type R = crate::R<I2C0_CONF_SPEC>;
#[doc = "Register `I2C0_CONF` writer"]
pub type W = crate::W<I2C0_CONF_SPEC>;
#[doc = "Field `LP_I2C_ANA_MAST_I2C0_CONF` reader - need_des"]
pub type LP_I2C_ANA_MAST_I2C0_CONF_R = crate::FieldReader<u32>;
#[doc = "Field `LP_I2C_ANA_MAST_I2C0_CONF` writer - need_des"]
pub type LP_I2C_ANA_MAST_I2C0_CONF_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 24, O, u32>;
#[doc = "Field `LP_I2C_ANA_MAST_I2C0_STATUS` reader - reserved"]
pub type LP_I2C_ANA_MAST_I2C0_STATUS_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:23 - need_des"]
    #[inline(always)]
    pub fn lp_i2c_ana_mast_i2c0_conf(&self) -> LP_I2C_ANA_MAST_I2C0_CONF_R {
        LP_I2C_ANA_MAST_I2C0_CONF_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - reserved"]
    #[inline(always)]
    pub fn lp_i2c_ana_mast_i2c0_status(&self) -> LP_I2C_ANA_MAST_I2C0_STATUS_R {
        LP_I2C_ANA_MAST_I2C0_STATUS_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C0_CONF")
            .field(
                "lp_i2c_ana_mast_i2c0_conf",
                &format_args!("{}", self.lp_i2c_ana_mast_i2c0_conf().bits()),
            )
            .field(
                "lp_i2c_ana_mast_i2c0_status",
                &format_args!("{}", self.lp_i2c_ana_mast_i2c0_status().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<I2C0_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:23 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_i2c_ana_mast_i2c0_conf(&mut self) -> LP_I2C_ANA_MAST_I2C0_CONF_W<I2C0_CONF_SPEC, 0> {
        LP_I2C_ANA_MAST_I2C0_CONF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c0_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c0_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C0_CONF_SPEC;
impl crate::RegisterSpec for I2C0_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c0_conf::R`](R) reader structure"]
impl crate::Readable for I2C0_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2c0_conf::W`](W) writer structure"]
impl crate::Writable for I2C0_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2C0_CONF to value 0x0700_0000"]
impl crate::Resettable for I2C0_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0700_0000;
}
