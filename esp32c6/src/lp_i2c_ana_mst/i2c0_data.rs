///Register `I2C0_DATA` reader
pub type R = crate::R<I2C0_DATA_SPEC>;
///Register `I2C0_DATA` writer
pub type W = crate::W<I2C0_DATA_SPEC>;
///Field `LP_I2C_ANA_MAST_I2C0_RDATA` reader - need_des
pub type LP_I2C_ANA_MAST_I2C0_RDATA_R = crate::FieldReader;
///Field `LP_I2C_ANA_MAST_I2C0_CLK_SEL` reader - need_des
pub type LP_I2C_ANA_MAST_I2C0_CLK_SEL_R = crate::FieldReader;
///Field `LP_I2C_ANA_MAST_I2C0_CLK_SEL` writer - need_des
pub type LP_I2C_ANA_MAST_I2C0_CLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `LP_I2C_ANA_MAST_I2C_MST_SEL` reader - need des
pub type LP_I2C_ANA_MAST_I2C_MST_SEL_R = crate::BitReader;
///Field `LP_I2C_ANA_MAST_I2C_MST_SEL` writer - need des
pub type LP_I2C_ANA_MAST_I2C_MST_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - need_des
    #[inline(always)]
    pub fn lp_i2c_ana_mast_i2c0_rdata(&self) -> LP_I2C_ANA_MAST_I2C0_RDATA_R {
        LP_I2C_ANA_MAST_I2C0_RDATA_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:10 - need_des
    #[inline(always)]
    pub fn lp_i2c_ana_mast_i2c0_clk_sel(&self) -> LP_I2C_ANA_MAST_I2C0_CLK_SEL_R {
        LP_I2C_ANA_MAST_I2C0_CLK_SEL_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bit 11 - need des
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
                &self.lp_i2c_ana_mast_i2c0_rdata(),
            )
            .field(
                "lp_i2c_ana_mast_i2c0_clk_sel",
                &self.lp_i2c_ana_mast_i2c0_clk_sel(),
            )
            .field(
                "lp_i2c_ana_mast_i2c_mst_sel",
                &self.lp_i2c_ana_mast_i2c_mst_sel(),
            )
            .finish()
    }
}
impl W {
    ///Bits 8:10 - need_des
    #[inline(always)]
    #[must_use]
    pub fn lp_i2c_ana_mast_i2c0_clk_sel(
        &mut self,
    ) -> LP_I2C_ANA_MAST_I2C0_CLK_SEL_W<I2C0_DATA_SPEC> {
        LP_I2C_ANA_MAST_I2C0_CLK_SEL_W::new(self, 8)
    }
    ///Bit 11 - need des
    #[inline(always)]
    #[must_use]
    pub fn lp_i2c_ana_mast_i2c_mst_sel(&mut self) -> LP_I2C_ANA_MAST_I2C_MST_SEL_W<I2C0_DATA_SPEC> {
        LP_I2C_ANA_MAST_I2C_MST_SEL_W::new(self, 11)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`i2c0_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c0_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct I2C0_DATA_SPEC;
impl crate::RegisterSpec for I2C0_DATA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`i2c0_data::R`](R) reader structure
impl crate::Readable for I2C0_DATA_SPEC {}
///`write(|w| ..)` method takes [`i2c0_data::W`](W) writer structure
impl crate::Writable for I2C0_DATA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets I2C0_DATA to value 0x0900
impl crate::Resettable for I2C0_DATA_SPEC {
    const RESET_VALUE: u32 = 0x0900;
}
