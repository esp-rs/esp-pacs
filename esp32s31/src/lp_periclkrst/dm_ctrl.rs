#[doc = "Register `DM_CTRL` reader"]
pub type R = crate::R<DM_CTRL_SPEC>;
#[doc = "Register `DM_CTRL` writer"]
pub type W = crate::W<DM_CTRL_SPEC>;
#[doc = "Field `LP_DM_CLK_EN` reader - need_des"]
pub type LP_DM_CLK_EN_R = crate::BitReader;
#[doc = "Field `LP_DM_CLK_EN` writer - need_des"]
pub type LP_DM_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_dm_clk_en(&self) -> LP_DM_CLK_EN_R {
        LP_DM_CLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DM_CTRL")
            .field("lp_dm_clk_en", &self.lp_dm_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_dm_clk_en(&mut self) -> LP_DM_CLK_EN_W<'_, DM_CTRL_SPEC> {
        LP_DM_CLK_EN_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`dm_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dm_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DM_CTRL_SPEC;
impl crate::RegisterSpec for DM_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dm_ctrl::R`](R) reader structure"]
impl crate::Readable for DM_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dm_ctrl::W`](W) writer structure"]
impl crate::Writable for DM_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DM_CTRL to value 0x8000_0000"]
impl crate::Resettable for DM_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x8000_0000;
}
