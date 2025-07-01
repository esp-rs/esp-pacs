#[doc = "Register `LP_CLK_CTRL` reader"]
pub type R = crate::R<LP_CLK_CTRL_SPEC>;
#[doc = "Register `LP_CLK_CTRL` writer"]
pub type W = crate::W<LP_CLK_CTRL_SPEC>;
#[doc = "Field `CLK_EN` reader - need_des"]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - need_des"]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_FOSC_HP_CKEN` reader - reserved"]
pub type LP_FOSC_HP_CKEN_R = crate::BitReader;
#[doc = "Field `LP_FOSC_HP_CKEN` writer - reserved"]
pub type LP_FOSC_HP_CKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 14 - reserved"]
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
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W<LP_CLK_CTRL_SPEC> {
        CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 14 - reserved"]
    #[inline(always)]
    pub fn lp_fosc_hp_cken(&mut self) -> LP_FOSC_HP_CKEN_W<LP_CLK_CTRL_SPEC> {
        LP_FOSC_HP_CKEN_W::new(self, 14)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_clk_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_clk_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_CLK_CTRL_SPEC;
impl crate::RegisterSpec for LP_CLK_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_clk_ctrl::R`](R) reader structure"]
impl crate::Readable for LP_CLK_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_clk_ctrl::W`](W) writer structure"]
impl crate::Writable for LP_CLK_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_CLK_CTRL to value 0x4001"]
impl crate::Resettable for LP_CLK_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x4001;
}
