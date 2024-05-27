///Register `CLKMGR_CFG` reader
pub type R = crate::R<CLKMGR_CFG_SPEC>;
///Register `CLKMGR_CFG` writer
pub type W = crate::W<CLKMGR_CFG_SPEC>;
///Field `TX_ESC_CLK_DIVISION` reader - NA
pub type TX_ESC_CLK_DIVISION_R = crate::FieldReader;
///Field `TX_ESC_CLK_DIVISION` writer - NA
pub type TX_ESC_CLK_DIVISION_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `TO_CLK_DIVISION` reader - NA
pub type TO_CLK_DIVISION_R = crate::FieldReader;
///Field `TO_CLK_DIVISION` writer - NA
pub type TO_CLK_DIVISION_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - NA
    #[inline(always)]
    pub fn tx_esc_clk_division(&self) -> TX_ESC_CLK_DIVISION_R {
        TX_ESC_CLK_DIVISION_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - NA
    #[inline(always)]
    pub fn to_clk_division(&self) -> TO_CLK_DIVISION_R {
        TO_CLK_DIVISION_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKMGR_CFG")
            .field("tx_esc_clk_division", &self.tx_esc_clk_division())
            .field("to_clk_division", &self.to_clk_division())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - NA
    #[inline(always)]
    #[must_use]
    pub fn tx_esc_clk_division(&mut self) -> TX_ESC_CLK_DIVISION_W<CLKMGR_CFG_SPEC> {
        TX_ESC_CLK_DIVISION_W::new(self, 0)
    }
    ///Bits 8:15 - NA
    #[inline(always)]
    #[must_use]
    pub fn to_clk_division(&mut self) -> TO_CLK_DIVISION_W<CLKMGR_CFG_SPEC> {
        TO_CLK_DIVISION_W::new(self, 8)
    }
}
/**NA

You can [`read`](crate::generic::Reg::read) this register and get [`clkmgr_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkmgr_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CLKMGR_CFG_SPEC;
impl crate::RegisterSpec for CLKMGR_CFG_SPEC {
    type Ux = u32;
}
///`read()` method returns [`clkmgr_cfg::R`](R) reader structure
impl crate::Readable for CLKMGR_CFG_SPEC {}
///`write(|w| ..)` method takes [`clkmgr_cfg::W`](W) writer structure
impl crate::Writable for CLKMGR_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CLKMGR_CFG to value 0
impl crate::Resettable for CLKMGR_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
