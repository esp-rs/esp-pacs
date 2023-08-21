#[doc = "Register `I2C0_DATA` reader"]
pub type R = crate::R<I2C0_DATA_SPEC>;
#[doc = "Register `I2C0_DATA` writer"]
pub type W = crate::W<I2C0_DATA_SPEC>;
#[doc = "Field `LP_I2C_ANA_MAST_I2C0_RDATA` reader - need_des"]
pub type LP_I2C_ANA_MAST_I2C0_RDATA_R = crate::FieldReader;
#[doc = "Field `LP_I2C_ANA_MAST_I2C0_CLK_SEL` reader - need_des"]
pub type LP_I2C_ANA_MAST_I2C0_CLK_SEL_R = crate::FieldReader;
#[doc = "Field `LP_I2C_ANA_MAST_I2C0_CLK_SEL` writer - need_des"]
pub type LP_I2C_ANA_MAST_I2C0_CLK_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `LP_I2C_ANA_MAST_I2C_MST_SEL` reader - need des"]
pub type LP_I2C_ANA_MAST_I2C_MST_SEL_R = crate::BitReader;
#[doc = "Field `LP_I2C_ANA_MAST_I2C_MST_SEL` writer - need des"]
pub type LP_I2C_ANA_MAST_I2C_MST_SEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    pub fn lp_i2c_ana_mast_i2c0_rdata(&self) -> LP_I2C_ANA_MAST_I2C0_RDATA_R {
        LP_I2C_ANA_MAST_I2C0_RDATA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - need_des"]
    #[inline(always)]
    pub fn lp_i2c_ana_mast_i2c0_clk_sel(&self) -> LP_I2C_ANA_MAST_I2C0_CLK_SEL_R {
        LP_I2C_ANA_MAST_I2C0_CLK_SEL_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - need des"]
    #[inline(always)]
    pub fn lp_i2c_ana_mast_i2c_mst_sel(&self) -> LP_I2C_ANA_MAST_I2C_MST_SEL_R {
        LP_I2C_ANA_MAST_I2C_MST_SEL_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C0_DATA")
            .field(
                "lp_i2c_ana_mast_i2c0_rdata",
                &format_args!("{}", self.lp_i2c_ana_mast_i2c0_rdata().bits()),
            )
            .field(
                "lp_i2c_ana_mast_i2c0_clk_sel",
                &format_args!("{}", self.lp_i2c_ana_mast_i2c0_clk_sel().bits()),
            )
            .field(
                "lp_i2c_ana_mast_i2c_mst_sel",
                &format_args!("{}", self.lp_i2c_ana_mast_i2c_mst_sel().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<I2C0_DATA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 8:10 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_i2c_ana_mast_i2c0_clk_sel(
        &mut self,
    ) -> LP_I2C_ANA_MAST_I2C0_CLK_SEL_W<I2C0_DATA_SPEC, 8> {
        LP_I2C_ANA_MAST_I2C0_CLK_SEL_W::new(self)
    }
    #[doc = "Bit 11 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_i2c_ana_mast_i2c_mst_sel(
        &mut self,
    ) -> LP_I2C_ANA_MAST_I2C_MST_SEL_W<I2C0_DATA_SPEC, 11> {
        LP_I2C_ANA_MAST_I2C_MST_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c0_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c0_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C0_DATA_SPEC;
impl crate::RegisterSpec for I2C0_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c0_data::R`](R) reader structure"]
impl crate::Readable for I2C0_DATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2c0_data::W`](W) writer structure"]
impl crate::Writable for I2C0_DATA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2C0_DATA to value 0x0900"]
impl crate::Resettable for I2C0_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0x0900;
}
