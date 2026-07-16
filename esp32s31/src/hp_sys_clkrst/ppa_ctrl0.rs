#[doc = "Register `PPA_CTRL0` reader"]
pub type R = crate::R<PPA_CTRL0_SPEC>;
#[doc = "Register `PPA_CTRL0` writer"]
pub type W = crate::W<PPA_CTRL0_SPEC>;
#[doc = "Field `PPA_SYS_CLK_EN` reader - need_des"]
pub type PPA_SYS_CLK_EN_R = crate::BitReader;
#[doc = "Field `PPA_SYS_CLK_EN` writer - need_des"]
pub type PPA_SYS_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PPA_RST_EN` reader - need_des"]
pub type PPA_RST_EN_R = crate::BitReader;
#[doc = "Field `PPA_RST_EN` writer - need_des"]
pub type PPA_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PPA_FORCE_NORST` reader - need_des"]
pub type PPA_FORCE_NORST_R = crate::BitReader;
#[doc = "Field `PPA_FORCE_NORST` writer - need_des"]
pub type PPA_FORCE_NORST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn ppa_sys_clk_en(&self) -> PPA_SYS_CLK_EN_R {
        PPA_SYS_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn ppa_rst_en(&self) -> PPA_RST_EN_R {
        PPA_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn ppa_force_norst(&self) -> PPA_FORCE_NORST_R {
        PPA_FORCE_NORST_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PPA_CTRL0")
            .field("ppa_sys_clk_en", &self.ppa_sys_clk_en())
            .field("ppa_rst_en", &self.ppa_rst_en())
            .field("ppa_force_norst", &self.ppa_force_norst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn ppa_sys_clk_en(&mut self) -> PPA_SYS_CLK_EN_W<'_, PPA_CTRL0_SPEC> {
        PPA_SYS_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn ppa_rst_en(&mut self) -> PPA_RST_EN_W<'_, PPA_CTRL0_SPEC> {
        PPA_RST_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn ppa_force_norst(&mut self) -> PPA_FORCE_NORST_W<'_, PPA_CTRL0_SPEC> {
        PPA_FORCE_NORST_W::new(self, 2)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ppa_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppa_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PPA_CTRL0_SPEC;
impl crate::RegisterSpec for PPA_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppa_ctrl0::R`](R) reader structure"]
impl crate::Readable for PPA_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ppa_ctrl0::W`](W) writer structure"]
impl crate::Writable for PPA_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PPA_CTRL0 to value 0"]
impl crate::Resettable for PPA_CTRL0_SPEC {}
