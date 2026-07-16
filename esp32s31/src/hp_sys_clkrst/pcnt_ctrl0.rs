#[doc = "Register `PCNT_CTRL0` reader"]
pub type R = crate::R<PCNT_CTRL0_SPEC>;
#[doc = "Register `PCNT_CTRL0` writer"]
pub type W = crate::W<PCNT_CTRL0_SPEC>;
#[doc = "Field `PCNT0_APB_CLK_EN` reader - need_des"]
pub type PCNT0_APB_CLK_EN_R = crate::BitReader;
#[doc = "Field `PCNT0_APB_CLK_EN` writer - need_des"]
pub type PCNT0_APB_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCNT0_RST_EN` reader - need_des"]
pub type PCNT0_RST_EN_R = crate::BitReader;
#[doc = "Field `PCNT0_RST_EN` writer - need_des"]
pub type PCNT0_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCNT0_FORCE_NORST` reader - need_des"]
pub type PCNT0_FORCE_NORST_R = crate::BitReader;
#[doc = "Field `PCNT0_FORCE_NORST` writer - need_des"]
pub type PCNT0_FORCE_NORST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCNT1_APB_CLK_EN` reader - need_des"]
pub type PCNT1_APB_CLK_EN_R = crate::BitReader;
#[doc = "Field `PCNT1_APB_CLK_EN` writer - need_des"]
pub type PCNT1_APB_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCNT1_RST_EN` reader - need_des"]
pub type PCNT1_RST_EN_R = crate::BitReader;
#[doc = "Field `PCNT1_RST_EN` writer - need_des"]
pub type PCNT1_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCNT1_FORCE_NORST` reader - need_des"]
pub type PCNT1_FORCE_NORST_R = crate::BitReader;
#[doc = "Field `PCNT1_FORCE_NORST` writer - need_des"]
pub type PCNT1_FORCE_NORST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn pcnt0_apb_clk_en(&self) -> PCNT0_APB_CLK_EN_R {
        PCNT0_APB_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn pcnt0_rst_en(&self) -> PCNT0_RST_EN_R {
        PCNT0_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn pcnt0_force_norst(&self) -> PCNT0_FORCE_NORST_R {
        PCNT0_FORCE_NORST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn pcnt1_apb_clk_en(&self) -> PCNT1_APB_CLK_EN_R {
        PCNT1_APB_CLK_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn pcnt1_rst_en(&self) -> PCNT1_RST_EN_R {
        PCNT1_RST_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn pcnt1_force_norst(&self) -> PCNT1_FORCE_NORST_R {
        PCNT1_FORCE_NORST_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCNT_CTRL0")
            .field("pcnt0_apb_clk_en", &self.pcnt0_apb_clk_en())
            .field("pcnt0_rst_en", &self.pcnt0_rst_en())
            .field("pcnt0_force_norst", &self.pcnt0_force_norst())
            .field("pcnt1_apb_clk_en", &self.pcnt1_apb_clk_en())
            .field("pcnt1_rst_en", &self.pcnt1_rst_en())
            .field("pcnt1_force_norst", &self.pcnt1_force_norst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn pcnt0_apb_clk_en(&mut self) -> PCNT0_APB_CLK_EN_W<'_, PCNT_CTRL0_SPEC> {
        PCNT0_APB_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn pcnt0_rst_en(&mut self) -> PCNT0_RST_EN_W<'_, PCNT_CTRL0_SPEC> {
        PCNT0_RST_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn pcnt0_force_norst(&mut self) -> PCNT0_FORCE_NORST_W<'_, PCNT_CTRL0_SPEC> {
        PCNT0_FORCE_NORST_W::new(self, 2)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn pcnt1_apb_clk_en(&mut self) -> PCNT1_APB_CLK_EN_W<'_, PCNT_CTRL0_SPEC> {
        PCNT1_APB_CLK_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn pcnt1_rst_en(&mut self) -> PCNT1_RST_EN_W<'_, PCNT_CTRL0_SPEC> {
        PCNT1_RST_EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn pcnt1_force_norst(&mut self) -> PCNT1_FORCE_NORST_W<'_, PCNT_CTRL0_SPEC> {
        PCNT1_FORCE_NORST_W::new(self, 5)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pcnt_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcnt_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCNT_CTRL0_SPEC;
impl crate::RegisterSpec for PCNT_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcnt_ctrl0::R`](R) reader structure"]
impl crate::Readable for PCNT_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pcnt_ctrl0::W`](W) writer structure"]
impl crate::Writable for PCNT_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PCNT_CTRL0 to value 0"]
impl crate::Resettable for PCNT_CTRL0_SPEC {}
