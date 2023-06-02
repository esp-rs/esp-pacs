#[doc = "Register `CLK_CONF_FORCE_ON` reader"]
pub struct R(crate::R<CLK_CONF_FORCE_ON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_CONF_FORCE_ON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_CONF_FORCE_ON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_CONF_FORCE_ON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_CONF_FORCE_ON` writer"]
pub struct W(crate::W<CLK_CONF_FORCE_ON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_CONF_FORCE_ON_SPEC>;
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
impl From<crate::W<CLK_CONF_FORCE_ON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_CONF_FORCE_ON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLK_WIFIPWR_FO` reader - "]
pub type CLK_WIFIPWR_FO_R = crate::BitReader;
#[doc = "Field `CLK_WIFIPWR_FO` writer - "]
pub type CLK_WIFIPWR_FO_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF_FORCE_ON_SPEC, O>;
#[doc = "Field `CLK_COEX_FO` reader - "]
pub type CLK_COEX_FO_R = crate::BitReader;
#[doc = "Field `CLK_COEX_FO` writer - "]
pub type CLK_COEX_FO_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF_FORCE_ON_SPEC, O>;
#[doc = "Field `CLK_I2C_MST_FO` reader - "]
pub type CLK_I2C_MST_FO_R = crate::BitReader;
#[doc = "Field `CLK_I2C_MST_FO` writer - "]
pub type CLK_I2C_MST_FO_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF_FORCE_ON_SPEC, O>;
#[doc = "Field `CLK_LP_TIMER_FO` reader - "]
pub type CLK_LP_TIMER_FO_R = crate::BitReader;
#[doc = "Field `CLK_LP_TIMER_FO` writer - "]
pub type CLK_LP_TIMER_FO_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF_FORCE_ON_SPEC, O>;
#[doc = "Field `CLK_BCMEM_FO` reader - "]
pub type CLK_BCMEM_FO_R = crate::BitReader;
#[doc = "Field `CLK_BCMEM_FO` writer - "]
pub type CLK_BCMEM_FO_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF_FORCE_ON_SPEC, O>;
#[doc = "Field `CLK_I2C_MST_MEM_FO` reader - "]
pub type CLK_I2C_MST_MEM_FO_R = crate::BitReader;
#[doc = "Field `CLK_I2C_MST_MEM_FO` writer - "]
pub type CLK_I2C_MST_MEM_FO_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF_FORCE_ON_SPEC, O>;
#[doc = "Field `CLK_CHAN_FREQ_MEM_FO` reader - "]
pub type CLK_CHAN_FREQ_MEM_FO_R = crate::BitReader;
#[doc = "Field `CLK_CHAN_FREQ_MEM_FO` writer - "]
pub type CLK_CHAN_FREQ_MEM_FO_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF_FORCE_ON_SPEC, O>;
#[doc = "Field `CLK_PBUS_MEM_FO` reader - "]
pub type CLK_PBUS_MEM_FO_R = crate::BitReader;
#[doc = "Field `CLK_PBUS_MEM_FO` writer - "]
pub type CLK_PBUS_MEM_FO_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF_FORCE_ON_SPEC, O>;
#[doc = "Field `CLK_AGC_MEM_FO` reader - "]
pub type CLK_AGC_MEM_FO_R = crate::BitReader;
#[doc = "Field `CLK_AGC_MEM_FO` writer - "]
pub type CLK_AGC_MEM_FO_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF_FORCE_ON_SPEC, O>;
#[doc = "Field `CLK_DC_MEM_FO` reader - "]
pub type CLK_DC_MEM_FO_R = crate::BitReader;
#[doc = "Field `CLK_DC_MEM_FO` writer - "]
pub type CLK_DC_MEM_FO_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF_FORCE_ON_SPEC, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clk_wifipwr_fo(&self) -> CLK_WIFIPWR_FO_R {
        CLK_WIFIPWR_FO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn clk_coex_fo(&self) -> CLK_COEX_FO_R {
        CLK_COEX_FO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn clk_i2c_mst_fo(&self) -> CLK_I2C_MST_FO_R {
        CLK_I2C_MST_FO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn clk_lp_timer_fo(&self) -> CLK_LP_TIMER_FO_R {
        CLK_LP_TIMER_FO_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn clk_bcmem_fo(&self) -> CLK_BCMEM_FO_R {
        CLK_BCMEM_FO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn clk_i2c_mst_mem_fo(&self) -> CLK_I2C_MST_MEM_FO_R {
        CLK_I2C_MST_MEM_FO_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn clk_chan_freq_mem_fo(&self) -> CLK_CHAN_FREQ_MEM_FO_R {
        CLK_CHAN_FREQ_MEM_FO_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn clk_pbus_mem_fo(&self) -> CLK_PBUS_MEM_FO_R {
        CLK_PBUS_MEM_FO_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn clk_agc_mem_fo(&self) -> CLK_AGC_MEM_FO_R {
        CLK_AGC_MEM_FO_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn clk_dc_mem_fo(&self) -> CLK_DC_MEM_FO_R {
        CLK_DC_MEM_FO_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK_CONF_FORCE_ON")
            .field(
                "clk_wifipwr_fo",
                &format_args!("{}", self.clk_wifipwr_fo().bit()),
            )
            .field("clk_coex_fo", &format_args!("{}", self.clk_coex_fo().bit()))
            .field(
                "clk_i2c_mst_fo",
                &format_args!("{}", self.clk_i2c_mst_fo().bit()),
            )
            .field(
                "clk_lp_timer_fo",
                &format_args!("{}", self.clk_lp_timer_fo().bit()),
            )
            .field(
                "clk_bcmem_fo",
                &format_args!("{}", self.clk_bcmem_fo().bit()),
            )
            .field(
                "clk_i2c_mst_mem_fo",
                &format_args!("{}", self.clk_i2c_mst_mem_fo().bit()),
            )
            .field(
                "clk_chan_freq_mem_fo",
                &format_args!("{}", self.clk_chan_freq_mem_fo().bit()),
            )
            .field(
                "clk_pbus_mem_fo",
                &format_args!("{}", self.clk_pbus_mem_fo().bit()),
            )
            .field(
                "clk_agc_mem_fo",
                &format_args!("{}", self.clk_agc_mem_fo().bit()),
            )
            .field(
                "clk_dc_mem_fo",
                &format_args!("{}", self.clk_dc_mem_fo().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CLK_CONF_FORCE_ON_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn clk_wifipwr_fo(&mut self) -> CLK_WIFIPWR_FO_W<0> {
        CLK_WIFIPWR_FO_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn clk_coex_fo(&mut self) -> CLK_COEX_FO_W<1> {
        CLK_COEX_FO_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn clk_i2c_mst_fo(&mut self) -> CLK_I2C_MST_FO_W<2> {
        CLK_I2C_MST_FO_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn clk_lp_timer_fo(&mut self) -> CLK_LP_TIMER_FO_W<3> {
        CLK_LP_TIMER_FO_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn clk_bcmem_fo(&mut self) -> CLK_BCMEM_FO_W<4> {
        CLK_BCMEM_FO_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn clk_i2c_mst_mem_fo(&mut self) -> CLK_I2C_MST_MEM_FO_W<5> {
        CLK_I2C_MST_MEM_FO_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn clk_chan_freq_mem_fo(&mut self) -> CLK_CHAN_FREQ_MEM_FO_W<6> {
        CLK_CHAN_FREQ_MEM_FO_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn clk_pbus_mem_fo(&mut self) -> CLK_PBUS_MEM_FO_W<7> {
        CLK_PBUS_MEM_FO_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn clk_agc_mem_fo(&mut self) -> CLK_AGC_MEM_FO_W<8> {
        CLK_AGC_MEM_FO_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn clk_dc_mem_fo(&mut self) -> CLK_DC_MEM_FO_W<9> {
        CLK_DC_MEM_FO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_conf_force_on](index.html) module"]
pub struct CLK_CONF_FORCE_ON_SPEC;
impl crate::RegisterSpec for CLK_CONF_FORCE_ON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_conf_force_on::R](R) reader structure"]
impl crate::Readable for CLK_CONF_FORCE_ON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_conf_force_on::W](W) writer structure"]
impl crate::Writable for CLK_CONF_FORCE_ON_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_CONF_FORCE_ON to value 0"]
impl crate::Resettable for CLK_CONF_FORCE_ON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
