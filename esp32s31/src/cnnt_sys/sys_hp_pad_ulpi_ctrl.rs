#[doc = "Register `SYS_HP_PAD_ULPI_CTRL` reader"]
pub type R = crate::R<SYS_HP_PAD_ULPI_CTRL_SPEC>;
#[doc = "Register `SYS_HP_PAD_ULPI_CTRL` writer"]
pub type W = crate::W<SYS_HP_PAD_ULPI_CTRL_SPEC>;
#[doc = "Field `SYS_HP_PAD_ULPI_CLK_OUT_INV_EN` reader - "]
pub type SYS_HP_PAD_ULPI_CLK_OUT_INV_EN_R = crate::BitReader;
#[doc = "Field `SYS_HP_PAD_ULPI_CLK_OUT_INV_EN` writer - "]
pub type SYS_HP_PAD_ULPI_CLK_OUT_INV_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYS_HP_PAD_ULPI_CLK_OUT_EN` reader - "]
pub type SYS_HP_PAD_ULPI_CLK_OUT_EN_R = crate::BitReader;
#[doc = "Field `SYS_HP_PAD_ULPI_CLK_OUT_EN` writer - "]
pub type SYS_HP_PAD_ULPI_CLK_OUT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sys_hp_pad_ulpi_clk_out_inv_en(&self) -> SYS_HP_PAD_ULPI_CLK_OUT_INV_EN_R {
        SYS_HP_PAD_ULPI_CLK_OUT_INV_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sys_hp_pad_ulpi_clk_out_en(&self) -> SYS_HP_PAD_ULPI_CLK_OUT_EN_R {
        SYS_HP_PAD_ULPI_CLK_OUT_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYS_HP_PAD_ULPI_CTRL")
            .field(
                "sys_hp_pad_ulpi_clk_out_inv_en",
                &self.sys_hp_pad_ulpi_clk_out_inv_en(),
            )
            .field(
                "sys_hp_pad_ulpi_clk_out_en",
                &self.sys_hp_pad_ulpi_clk_out_en(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sys_hp_pad_ulpi_clk_out_inv_en(
        &mut self,
    ) -> SYS_HP_PAD_ULPI_CLK_OUT_INV_EN_W<'_, SYS_HP_PAD_ULPI_CTRL_SPEC> {
        SYS_HP_PAD_ULPI_CLK_OUT_INV_EN_W::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sys_hp_pad_ulpi_clk_out_en(
        &mut self,
    ) -> SYS_HP_PAD_ULPI_CLK_OUT_EN_W<'_, SYS_HP_PAD_ULPI_CTRL_SPEC> {
        SYS_HP_PAD_ULPI_CLK_OUT_EN_W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_hp_pad_ulpi_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_hp_pad_ulpi_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_HP_PAD_ULPI_CTRL_SPEC;
impl crate::RegisterSpec for SYS_HP_PAD_ULPI_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_hp_pad_ulpi_ctrl::R`](R) reader structure"]
impl crate::Readable for SYS_HP_PAD_ULPI_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_hp_pad_ulpi_ctrl::W`](W) writer structure"]
impl crate::Writable for SYS_HP_PAD_ULPI_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYS_HP_PAD_ULPI_CTRL to value 0"]
impl crate::Resettable for SYS_HP_PAD_ULPI_CTRL_SPEC {}
