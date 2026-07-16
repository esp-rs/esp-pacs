#[doc = "Register `SYS_CNNT_IOMUX_CTRL0` reader"]
pub type R = crate::R<SYS_CNNT_IOMUX_CTRL0_SPEC>;
#[doc = "Register `SYS_CNNT_IOMUX_CTRL0` writer"]
pub type W = crate::W<SYS_CNNT_IOMUX_CTRL0_SPEC>;
#[doc = "Field `SYS_REG_CNNT_IOMUX_APB_RST_EN` reader - "]
pub type SYS_REG_CNNT_IOMUX_APB_RST_EN_R = crate::BitReader;
#[doc = "Field `SYS_REG_CNNT_IOMUX_APB_RST_EN` writer - "]
pub type SYS_REG_CNNT_IOMUX_APB_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sys_reg_cnnt_iomux_apb_rst_en(&self) -> SYS_REG_CNNT_IOMUX_APB_RST_EN_R {
        SYS_REG_CNNT_IOMUX_APB_RST_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYS_CNNT_IOMUX_CTRL0")
            .field(
                "sys_reg_cnnt_iomux_apb_rst_en",
                &self.sys_reg_cnnt_iomux_apb_rst_en(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sys_reg_cnnt_iomux_apb_rst_en(
        &mut self,
    ) -> SYS_REG_CNNT_IOMUX_APB_RST_EN_W<'_, SYS_CNNT_IOMUX_CTRL0_SPEC> {
        SYS_REG_CNNT_IOMUX_APB_RST_EN_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_cnnt_iomux_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_cnnt_iomux_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_CNNT_IOMUX_CTRL0_SPEC;
impl crate::RegisterSpec for SYS_CNNT_IOMUX_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_cnnt_iomux_ctrl0::R`](R) reader structure"]
impl crate::Readable for SYS_CNNT_IOMUX_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_cnnt_iomux_ctrl0::W`](W) writer structure"]
impl crate::Writable for SYS_CNNT_IOMUX_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYS_CNNT_IOMUX_CTRL0 to value 0"]
impl crate::Resettable for SYS_CNNT_IOMUX_CTRL0_SPEC {}
