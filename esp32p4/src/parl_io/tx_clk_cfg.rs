///Register `TX_CLK_CFG` reader
pub type R = crate::R<TX_CLK_CFG_SPEC>;
///Register `TX_CLK_CFG` writer
pub type W = crate::W<TX_CLK_CFG_SPEC>;
///Field `TX_CLK_I_INV` reader - Set this bit to invert the input Tx core clock.
pub type TX_CLK_I_INV_R = crate::BitReader;
///Field `TX_CLK_I_INV` writer - Set this bit to invert the input Tx core clock.
pub type TX_CLK_I_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_CLK_O_INV` reader - Set this bit to invert the output Tx core clock.
pub type TX_CLK_O_INV_R = crate::BitReader;
///Field `TX_CLK_O_INV` writer - Set this bit to invert the output Tx core clock.
pub type TX_CLK_O_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 30 - Set this bit to invert the input Tx core clock.
    #[inline(always)]
    pub fn tx_clk_i_inv(&self) -> TX_CLK_I_INV_R {
        TX_CLK_I_INV_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Set this bit to invert the output Tx core clock.
    #[inline(always)]
    pub fn tx_clk_o_inv(&self) -> TX_CLK_O_INV_R {
        TX_CLK_O_INV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_CLK_CFG")
            .field("tx_clk_i_inv", &self.tx_clk_i_inv())
            .field("tx_clk_o_inv", &self.tx_clk_o_inv())
            .finish()
    }
}
impl W {
    ///Bit 30 - Set this bit to invert the input Tx core clock.
    #[inline(always)]
    #[must_use]
    pub fn tx_clk_i_inv(&mut self) -> TX_CLK_I_INV_W<TX_CLK_CFG_SPEC> {
        TX_CLK_I_INV_W::new(self, 30)
    }
    ///Bit 31 - Set this bit to invert the output Tx core clock.
    #[inline(always)]
    #[must_use]
    pub fn tx_clk_o_inv(&mut self) -> TX_CLK_O_INV_W<TX_CLK_CFG_SPEC> {
        TX_CLK_O_INV_W::new(self, 31)
    }
}
/**Parallel IO TX clk configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`tx_clk_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_clk_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TX_CLK_CFG_SPEC;
impl crate::RegisterSpec for TX_CLK_CFG_SPEC {
    type Ux = u32;
}
///`read()` method returns [`tx_clk_cfg::R`](R) reader structure
impl crate::Readable for TX_CLK_CFG_SPEC {}
///`write(|w| ..)` method takes [`tx_clk_cfg::W`](W) writer structure
impl crate::Writable for TX_CLK_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TX_CLK_CFG to value 0
impl crate::Resettable for TX_CLK_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
