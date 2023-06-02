#[doc = "Register `MEM_CONF` reader"]
pub struct R(crate::R<MEM_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEM_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEM_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEM_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MEM_CONF` writer"]
pub struct W(crate::W<MEM_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEM_CONF_SPEC>;
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
impl From<crate::W<MEM_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEM_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AGC_MEM_FORCE_PU` reader - "]
pub type AGC_MEM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `AGC_MEM_FORCE_PU` writer - "]
pub type AGC_MEM_FORCE_PU_W<'a, const O: u8> = crate::BitWriter<'a, MEM_CONF_SPEC, O>;
#[doc = "Field `AGC_MEM_FORCE_PD` reader - "]
pub type AGC_MEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `AGC_MEM_FORCE_PD` writer - "]
pub type AGC_MEM_FORCE_PD_W<'a, const O: u8> = crate::BitWriter<'a, MEM_CONF_SPEC, O>;
#[doc = "Field `PBUS_MEM_FORCE_PU` reader - "]
pub type PBUS_MEM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `PBUS_MEM_FORCE_PU` writer - "]
pub type PBUS_MEM_FORCE_PU_W<'a, const O: u8> = crate::BitWriter<'a, MEM_CONF_SPEC, O>;
#[doc = "Field `PBUS_MEM_FORCE_PD` reader - "]
pub type PBUS_MEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `PBUS_MEM_FORCE_PD` writer - "]
pub type PBUS_MEM_FORCE_PD_W<'a, const O: u8> = crate::BitWriter<'a, MEM_CONF_SPEC, O>;
#[doc = "Field `I2C_MST_MEM_FORCE_PU` reader - "]
pub type I2C_MST_MEM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `I2C_MST_MEM_FORCE_PU` writer - "]
pub type I2C_MST_MEM_FORCE_PU_W<'a, const O: u8> = crate::BitWriter<'a, MEM_CONF_SPEC, O>;
#[doc = "Field `I2C_MST_MEM_FORCE_PD` reader - "]
pub type I2C_MST_MEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `I2C_MST_MEM_FORCE_PD` writer - "]
pub type I2C_MST_MEM_FORCE_PD_W<'a, const O: u8> = crate::BitWriter<'a, MEM_CONF_SPEC, O>;
#[doc = "Field `CHAN_FREQ_MEM_FORCE_PU` reader - "]
pub type CHAN_FREQ_MEM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `CHAN_FREQ_MEM_FORCE_PU` writer - "]
pub type CHAN_FREQ_MEM_FORCE_PU_W<'a, const O: u8> = crate::BitWriter<'a, MEM_CONF_SPEC, O>;
#[doc = "Field `CHAN_FREQ_MEM_FORCE_PD` reader - "]
pub type CHAN_FREQ_MEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `CHAN_FREQ_MEM_FORCE_PD` writer - "]
pub type CHAN_FREQ_MEM_FORCE_PD_W<'a, const O: u8> = crate::BitWriter<'a, MEM_CONF_SPEC, O>;
#[doc = "Field `MODEM_PWR_MEM_WP` reader - "]
pub type MODEM_PWR_MEM_WP_R = crate::FieldReader;
#[doc = "Field `MODEM_PWR_MEM_WP` writer - "]
pub type MODEM_PWR_MEM_WP_W<'a, const O: u8> = crate::FieldWriter<'a, MEM_CONF_SPEC, 3, O>;
#[doc = "Field `MODEM_PWR_MEM_WA` reader - "]
pub type MODEM_PWR_MEM_WA_R = crate::FieldReader;
#[doc = "Field `MODEM_PWR_MEM_WA` writer - "]
pub type MODEM_PWR_MEM_WA_W<'a, const O: u8> = crate::FieldWriter<'a, MEM_CONF_SPEC, 3, O>;
#[doc = "Field `MODEM_PWR_MEM_RA` reader - "]
pub type MODEM_PWR_MEM_RA_R = crate::FieldReader;
#[doc = "Field `MODEM_PWR_MEM_RA` writer - "]
pub type MODEM_PWR_MEM_RA_W<'a, const O: u8> = crate::FieldWriter<'a, MEM_CONF_SPEC, 2, O>;
#[doc = "Field `MODEM_PWR_MEM_RM` reader - "]
pub type MODEM_PWR_MEM_RM_R = crate::FieldReader;
#[doc = "Field `MODEM_PWR_MEM_RM` writer - "]
pub type MODEM_PWR_MEM_RM_W<'a, const O: u8> = crate::FieldWriter<'a, MEM_CONF_SPEC, 4, O>;
impl R {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn agc_mem_force_pu(&self) -> AGC_MEM_FORCE_PU_R {
        AGC_MEM_FORCE_PU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn agc_mem_force_pd(&self) -> AGC_MEM_FORCE_PD_R {
        AGC_MEM_FORCE_PD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pbus_mem_force_pu(&self) -> PBUS_MEM_FORCE_PU_R {
        PBUS_MEM_FORCE_PU_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pbus_mem_force_pd(&self) -> PBUS_MEM_FORCE_PD_R {
        PBUS_MEM_FORCE_PD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn i2c_mst_mem_force_pu(&self) -> I2C_MST_MEM_FORCE_PU_R {
        I2C_MST_MEM_FORCE_PU_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn i2c_mst_mem_force_pd(&self) -> I2C_MST_MEM_FORCE_PD_R {
        I2C_MST_MEM_FORCE_PD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn chan_freq_mem_force_pu(&self) -> CHAN_FREQ_MEM_FORCE_PU_R {
        CHAN_FREQ_MEM_FORCE_PU_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn chan_freq_mem_force_pd(&self) -> CHAN_FREQ_MEM_FORCE_PD_R {
        CHAN_FREQ_MEM_FORCE_PD_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn modem_pwr_mem_wp(&self) -> MODEM_PWR_MEM_WP_R {
        MODEM_PWR_MEM_WP_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17"]
    #[inline(always)]
    pub fn modem_pwr_mem_wa(&self) -> MODEM_PWR_MEM_WA_R {
        MODEM_PWR_MEM_WA_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn modem_pwr_mem_ra(&self) -> MODEM_PWR_MEM_RA_R {
        MODEM_PWR_MEM_RA_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn modem_pwr_mem_rm(&self) -> MODEM_PWR_MEM_RM_R {
        MODEM_PWR_MEM_RM_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_CONF")
            .field(
                "agc_mem_force_pu",
                &format_args!("{}", self.agc_mem_force_pu().bit()),
            )
            .field(
                "agc_mem_force_pd",
                &format_args!("{}", self.agc_mem_force_pd().bit()),
            )
            .field(
                "pbus_mem_force_pu",
                &format_args!("{}", self.pbus_mem_force_pu().bit()),
            )
            .field(
                "pbus_mem_force_pd",
                &format_args!("{}", self.pbus_mem_force_pd().bit()),
            )
            .field(
                "i2c_mst_mem_force_pu",
                &format_args!("{}", self.i2c_mst_mem_force_pu().bit()),
            )
            .field(
                "i2c_mst_mem_force_pd",
                &format_args!("{}", self.i2c_mst_mem_force_pd().bit()),
            )
            .field(
                "chan_freq_mem_force_pu",
                &format_args!("{}", self.chan_freq_mem_force_pu().bit()),
            )
            .field(
                "chan_freq_mem_force_pd",
                &format_args!("{}", self.chan_freq_mem_force_pd().bit()),
            )
            .field(
                "modem_pwr_mem_wp",
                &format_args!("{}", self.modem_pwr_mem_wp().bits()),
            )
            .field(
                "modem_pwr_mem_wa",
                &format_args!("{}", self.modem_pwr_mem_wa().bits()),
            )
            .field(
                "modem_pwr_mem_ra",
                &format_args!("{}", self.modem_pwr_mem_ra().bits()),
            )
            .field(
                "modem_pwr_mem_rm",
                &format_args!("{}", self.modem_pwr_mem_rm().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MEM_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn agc_mem_force_pu(&mut self) -> AGC_MEM_FORCE_PU_W<2> {
        AGC_MEM_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn agc_mem_force_pd(&mut self) -> AGC_MEM_FORCE_PD_W<3> {
        AGC_MEM_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn pbus_mem_force_pu(&mut self) -> PBUS_MEM_FORCE_PU_W<4> {
        PBUS_MEM_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn pbus_mem_force_pd(&mut self) -> PBUS_MEM_FORCE_PD_W<5> {
        PBUS_MEM_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_mst_mem_force_pu(&mut self) -> I2C_MST_MEM_FORCE_PU_W<8> {
        I2C_MST_MEM_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_mst_mem_force_pd(&mut self) -> I2C_MST_MEM_FORCE_PD_W<9> {
        I2C_MST_MEM_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn chan_freq_mem_force_pu(&mut self) -> CHAN_FREQ_MEM_FORCE_PU_W<10> {
        CHAN_FREQ_MEM_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn chan_freq_mem_force_pd(&mut self) -> CHAN_FREQ_MEM_FORCE_PD_W<11> {
        CHAN_FREQ_MEM_FORCE_PD_W::new(self)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    #[must_use]
    pub fn modem_pwr_mem_wp(&mut self) -> MODEM_PWR_MEM_WP_W<12> {
        MODEM_PWR_MEM_WP_W::new(self)
    }
    #[doc = "Bits 15:17"]
    #[inline(always)]
    #[must_use]
    pub fn modem_pwr_mem_wa(&mut self) -> MODEM_PWR_MEM_WA_W<15> {
        MODEM_PWR_MEM_WA_W::new(self)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    #[must_use]
    pub fn modem_pwr_mem_ra(&mut self) -> MODEM_PWR_MEM_RA_W<18> {
        MODEM_PWR_MEM_RA_W::new(self)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    #[must_use]
    pub fn modem_pwr_mem_rm(&mut self) -> MODEM_PWR_MEM_RM_W<20> {
        MODEM_PWR_MEM_RM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_conf](index.html) module"]
pub struct MEM_CONF_SPEC;
impl crate::RegisterSpec for MEM_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mem_conf::R](R) reader structure"]
impl crate::Readable for MEM_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mem_conf::W](W) writer structure"]
impl crate::Writable for MEM_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MEM_CONF to value 0x0022_8014"]
impl crate::Resettable for MEM_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0022_8014;
}
