#[doc = "Register `CNNT_IOMUX_CTRL0` reader"]
pub type R = crate::R<CNNT_IOMUX_CTRL0_SPEC>;
#[doc = "Register `CNNT_IOMUX_CTRL0` writer"]
pub type W = crate::W<CNNT_IOMUX_CTRL0_SPEC>;
#[doc = "Field `CNNT_IOMUX_APB_CLK_EN` reader - cnnt_iomux clk en"]
pub type CNNT_IOMUX_APB_CLK_EN_R = crate::BitReader;
#[doc = "Field `CNNT_IOMUX_APB_CLK_EN` writer - cnnt_iomux clk en"]
pub type CNNT_IOMUX_APB_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - cnnt_iomux clk en"]
    #[inline(always)]
    pub fn cnnt_iomux_apb_clk_en(&self) -> CNNT_IOMUX_APB_CLK_EN_R {
        CNNT_IOMUX_APB_CLK_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNNT_IOMUX_CTRL0")
            .field("cnnt_iomux_apb_clk_en", &self.cnnt_iomux_apb_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - cnnt_iomux clk en"]
    #[inline(always)]
    pub fn cnnt_iomux_apb_clk_en(&mut self) -> CNNT_IOMUX_APB_CLK_EN_W<'_, CNNT_IOMUX_CTRL0_SPEC> {
        CNNT_IOMUX_APB_CLK_EN_W::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`cnnt_iomux_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnnt_iomux_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNNT_IOMUX_CTRL0_SPEC;
impl crate::RegisterSpec for CNNT_IOMUX_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cnnt_iomux_ctrl0::R`](R) reader structure"]
impl crate::Readable for CNNT_IOMUX_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cnnt_iomux_ctrl0::W`](W) writer structure"]
impl crate::Writable for CNNT_IOMUX_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CNNT_IOMUX_CTRL0 to value 0x01"]
impl crate::Resettable for CNNT_IOMUX_CTRL0_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
