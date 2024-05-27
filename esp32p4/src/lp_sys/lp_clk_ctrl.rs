///Register `LP_CLK_CTRL` reader
pub type R = crate::R<LP_CLK_CTRL_SPEC>;
///Register `LP_CLK_CTRL` writer
pub type W = crate::W<LP_CLK_CTRL_SPEC>;
///Field `CLK_EN` reader - need_des
pub type CLK_EN_R = crate::BitReader;
///Field `CLK_EN` writer - need_des
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LP_FOSC_HP_CKEN` reader - reserved
pub type LP_FOSC_HP_CKEN_R = crate::BitReader;
///Field `LP_FOSC_HP_CKEN` writer - reserved
pub type LP_FOSC_HP_CKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - need_des
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 14 - reserved
    #[inline(always)]
    pub fn lp_fosc_hp_cken(&self) -> LP_FOSC_HP_CKEN_R {
        LP_FOSC_HP_CKEN_R::new(((self.bits >> 14) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_CLK_CTRL")
            .field("clk_en", &self.clk_en())
            .field("lp_fosc_hp_cken", &self.lp_fosc_hp_cken())
            .finish()
    }
}
impl W {
    ///Bit 0 - need_des
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<LP_CLK_CTRL_SPEC> {
        CLK_EN_W::new(self, 0)
    }
    ///Bit 14 - reserved
    #[inline(always)]
    #[must_use]
    pub fn lp_fosc_hp_cken(&mut self) -> LP_FOSC_HP_CKEN_W<LP_CLK_CTRL_SPEC> {
        LP_FOSC_HP_CKEN_W::new(self, 14)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`lp_clk_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_clk_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LP_CLK_CTRL_SPEC;
impl crate::RegisterSpec for LP_CLK_CTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`lp_clk_ctrl::R`](R) reader structure
impl crate::Readable for LP_CLK_CTRL_SPEC {}
///`write(|w| ..)` method takes [`lp_clk_ctrl::W`](W) writer structure
impl crate::Writable for LP_CLK_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LP_CLK_CTRL to value 0x4001
impl crate::Resettable for LP_CLK_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x4001;
}
