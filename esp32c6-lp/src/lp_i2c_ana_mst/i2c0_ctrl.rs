///Register `I2C0_CTRL` reader
pub type R = crate::R<I2C0_CTRL_SPEC>;
///Register `I2C0_CTRL` writer
pub type W = crate::W<I2C0_CTRL_SPEC>;
///Field `LP_I2C_ANA_MAST_I2C0_CTRL` reader - need_des
pub type LP_I2C_ANA_MAST_I2C0_CTRL_R = crate::FieldReader<u32>;
///Field `LP_I2C_ANA_MAST_I2C0_CTRL` writer - need_des
pub type LP_I2C_ANA_MAST_I2C0_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
///Field `LP_I2C_ANA_MAST_I2C0_BUSY` reader - need_des
pub type LP_I2C_ANA_MAST_I2C0_BUSY_R = crate::BitReader;
impl R {
    ///Bits 0:24 - need_des
    #[inline(always)]
    pub fn lp_i2c_ana_mast_i2c0_ctrl(&self) -> LP_I2C_ANA_MAST_I2C0_CTRL_R {
        LP_I2C_ANA_MAST_I2C0_CTRL_R::new(self.bits & 0x01ff_ffff)
    }
    ///Bit 25 - need_des
    #[inline(always)]
    pub fn lp_i2c_ana_mast_i2c0_busy(&self) -> LP_I2C_ANA_MAST_I2C0_BUSY_R {
        LP_I2C_ANA_MAST_I2C0_BUSY_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C0_CTRL")
            .field(
                "lp_i2c_ana_mast_i2c0_ctrl",
                &self.lp_i2c_ana_mast_i2c0_ctrl(),
            )
            .field(
                "lp_i2c_ana_mast_i2c0_busy",
                &self.lp_i2c_ana_mast_i2c0_busy(),
            )
            .finish()
    }
}
impl W {
    ///Bits 0:24 - need_des
    #[inline(always)]
    #[must_use]
    pub fn lp_i2c_ana_mast_i2c0_ctrl(&mut self) -> LP_I2C_ANA_MAST_I2C0_CTRL_W<I2C0_CTRL_SPEC> {
        LP_I2C_ANA_MAST_I2C0_CTRL_W::new(self, 0)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`i2c0_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c0_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct I2C0_CTRL_SPEC;
impl crate::RegisterSpec for I2C0_CTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`i2c0_ctrl::R`](R) reader structure
impl crate::Readable for I2C0_CTRL_SPEC {}
///`write(|w| ..)` method takes [`i2c0_ctrl::W`](W) writer structure
impl crate::Writable for I2C0_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets I2C0_CTRL to value 0
impl crate::Resettable for I2C0_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
