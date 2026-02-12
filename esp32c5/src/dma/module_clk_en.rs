#[doc = "Register `MODULE_CLK_EN` reader"]
pub type R = crate::R<MODULE_CLK_EN_SPEC>;
#[doc = "Register `MODULE_CLK_EN` writer"]
pub type W = crate::W<MODULE_CLK_EN_SPEC>;
#[doc = "Field `AHB_APB_SYNC_CLK_EN` reader - Configures whether to force on ahb_apb_sync 2~0 module clock."]
pub type AHB_APB_SYNC_CLK_EN_R = crate::FieldReader;
#[doc = "Field `AHB_APB_SYNC_CLK_EN` writer - Configures whether to force on ahb_apb_sync 2~0 module clock."]
pub type AHB_APB_SYNC_CLK_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OUT_DSCR_CLK_EN` reader - Configures whether to force on out_dscr 2~0 module clock."]
pub type OUT_DSCR_CLK_EN_R = crate::FieldReader;
#[doc = "Field `OUT_DSCR_CLK_EN` writer - Configures whether to force on out_dscr 2~0 module clock."]
pub type OUT_DSCR_CLK_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OUT_CTRL_CLK_EN` reader - Configures whether to force on out_ctrl 2~0 module clock."]
pub type OUT_CTRL_CLK_EN_R = crate::FieldReader;
#[doc = "Field `OUT_CTRL_CLK_EN` writer - Configures whether to force on out_ctrl 2~0 module clock."]
pub type OUT_CTRL_CLK_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IN_DSCR_CLK_EN` reader - Configures whether to force on in_dscr 2~0 module clock."]
pub type IN_DSCR_CLK_EN_R = crate::FieldReader;
#[doc = "Field `IN_DSCR_CLK_EN` writer - Configures whether to force on in_dscr 2~0 module clock."]
pub type IN_DSCR_CLK_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IN_CTRL_CLK_EN` reader - Configures whether to force on in_ctrl 2~0 module clock."]
pub type IN_CTRL_CLK_EN_R = crate::FieldReader;
#[doc = "Field `IN_CTRL_CLK_EN` writer - Configures whether to force on in_ctrl 2~0 module clock."]
pub type IN_CTRL_CLK_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CMD_ARB_CLK_EN` reader - Configures whether to force on cmd_arb module clock."]
pub type CMD_ARB_CLK_EN_R = crate::BitReader;
#[doc = "Field `CMD_ARB_CLK_EN` writer - Configures whether to force on cmd_arb module clock."]
pub type CMD_ARB_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBINF_CLK_EN` reader - Configures whether to force on arbinf module clock."]
pub type AHBINF_CLK_EN_R = crate::BitReader;
#[doc = "Field `AHBINF_CLK_EN` writer - Configures whether to force on arbinf module clock."]
pub type AHBINF_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Configures whether to force on ahb_apb_sync 2~0 module clock."]
    #[inline(always)]
    pub fn ahb_apb_sync_clk_en(&self) -> AHB_APB_SYNC_CLK_EN_R {
        AHB_APB_SYNC_CLK_EN_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Configures whether to force on out_dscr 2~0 module clock."]
    #[inline(always)]
    pub fn out_dscr_clk_en(&self) -> OUT_DSCR_CLK_EN_R {
        OUT_DSCR_CLK_EN_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Configures whether to force on out_ctrl 2~0 module clock."]
    #[inline(always)]
    pub fn out_ctrl_clk_en(&self) -> OUT_CTRL_CLK_EN_R {
        OUT_CTRL_CLK_EN_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - Configures whether to force on in_dscr 2~0 module clock."]
    #[inline(always)]
    pub fn in_dscr_clk_en(&self) -> IN_DSCR_CLK_EN_R {
        IN_DSCR_CLK_EN_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Configures whether to force on in_ctrl 2~0 module clock."]
    #[inline(always)]
    pub fn in_ctrl_clk_en(&self) -> IN_CTRL_CLK_EN_R {
        IN_CTRL_CLK_EN_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 27 - Configures whether to force on cmd_arb module clock."]
    #[inline(always)]
    pub fn cmd_arb_clk_en(&self) -> CMD_ARB_CLK_EN_R {
        CMD_ARB_CLK_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Configures whether to force on arbinf module clock."]
    #[inline(always)]
    pub fn ahbinf_clk_en(&self) -> AHBINF_CLK_EN_R {
        AHBINF_CLK_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODULE_CLK_EN")
            .field("ahb_apb_sync_clk_en", &self.ahb_apb_sync_clk_en())
            .field("out_dscr_clk_en", &self.out_dscr_clk_en())
            .field("out_ctrl_clk_en", &self.out_ctrl_clk_en())
            .field("in_dscr_clk_en", &self.in_dscr_clk_en())
            .field("in_ctrl_clk_en", &self.in_ctrl_clk_en())
            .field("cmd_arb_clk_en", &self.cmd_arb_clk_en())
            .field("ahbinf_clk_en", &self.ahbinf_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Configures whether to force on ahb_apb_sync 2~0 module clock."]
    #[inline(always)]
    pub fn ahb_apb_sync_clk_en(&mut self) -> AHB_APB_SYNC_CLK_EN_W<'_, MODULE_CLK_EN_SPEC> {
        AHB_APB_SYNC_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - Configures whether to force on out_dscr 2~0 module clock."]
    #[inline(always)]
    pub fn out_dscr_clk_en(&mut self) -> OUT_DSCR_CLK_EN_W<'_, MODULE_CLK_EN_SPEC> {
        OUT_DSCR_CLK_EN_W::new(self, 3)
    }
    #[doc = "Bits 6:8 - Configures whether to force on out_ctrl 2~0 module clock."]
    #[inline(always)]
    pub fn out_ctrl_clk_en(&mut self) -> OUT_CTRL_CLK_EN_W<'_, MODULE_CLK_EN_SPEC> {
        OUT_CTRL_CLK_EN_W::new(self, 6)
    }
    #[doc = "Bits 9:11 - Configures whether to force on in_dscr 2~0 module clock."]
    #[inline(always)]
    pub fn in_dscr_clk_en(&mut self) -> IN_DSCR_CLK_EN_W<'_, MODULE_CLK_EN_SPEC> {
        IN_DSCR_CLK_EN_W::new(self, 9)
    }
    #[doc = "Bits 12:14 - Configures whether to force on in_ctrl 2~0 module clock."]
    #[inline(always)]
    pub fn in_ctrl_clk_en(&mut self) -> IN_CTRL_CLK_EN_W<'_, MODULE_CLK_EN_SPEC> {
        IN_CTRL_CLK_EN_W::new(self, 12)
    }
    #[doc = "Bit 27 - Configures whether to force on cmd_arb module clock."]
    #[inline(always)]
    pub fn cmd_arb_clk_en(&mut self) -> CMD_ARB_CLK_EN_W<'_, MODULE_CLK_EN_SPEC> {
        CMD_ARB_CLK_EN_W::new(self, 27)
    }
    #[doc = "Bit 28 - Configures whether to force on arbinf module clock."]
    #[inline(always)]
    pub fn ahbinf_clk_en(&mut self) -> AHBINF_CLK_EN_W<'_, MODULE_CLK_EN_SPEC> {
        AHBINF_CLK_EN_W::new(self, 28)
    }
}
#[doc = "Module clock force on register\n\nYou can [`read`](crate::Reg::read) this register and get [`module_clk_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`module_clk_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MODULE_CLK_EN_SPEC;
impl crate::RegisterSpec for MODULE_CLK_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`module_clk_en::R`](R) reader structure"]
impl crate::Readable for MODULE_CLK_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`module_clk_en::W`](W) writer structure"]
impl crate::Writable for MODULE_CLK_EN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MODULE_CLK_EN to value 0"]
impl crate::Resettable for MODULE_CLK_EN_SPEC {}
