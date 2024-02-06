#[doc = "Register `ICM_CPU_H2X_CFG` reader"]
pub type R = crate::R<ICM_CPU_H2X_CFG_SPEC>;
#[doc = "Register `ICM_CPU_H2X_CFG` writer"]
pub type W = crate::W<ICM_CPU_H2X_CFG_SPEC>;
#[doc = "Field `CPU_ICM_H2X_POST_WR_EN` reader - need_des"]
pub type CPU_ICM_H2X_POST_WR_EN_R = crate::BitReader;
#[doc = "Field `CPU_ICM_H2X_POST_WR_EN` writer - need_des"]
pub type CPU_ICM_H2X_POST_WR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU_ICM_H2X_CUT_THROUGH_EN` reader - need_des"]
pub type CPU_ICM_H2X_CUT_THROUGH_EN_R = crate::BitReader;
#[doc = "Field `CPU_ICM_H2X_CUT_THROUGH_EN` writer - need_des"]
pub type CPU_ICM_H2X_CUT_THROUGH_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU_ICM_H2X_BRIDGE_BUSY` reader - need_des"]
pub type CPU_ICM_H2X_BRIDGE_BUSY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn cpu_icm_h2x_post_wr_en(&self) -> CPU_ICM_H2X_POST_WR_EN_R {
        CPU_ICM_H2X_POST_WR_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn cpu_icm_h2x_cut_through_en(&self) -> CPU_ICM_H2X_CUT_THROUGH_EN_R {
        CPU_ICM_H2X_CUT_THROUGH_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn cpu_icm_h2x_bridge_busy(&self) -> CPU_ICM_H2X_BRIDGE_BUSY_R {
        CPU_ICM_H2X_BRIDGE_BUSY_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICM_CPU_H2X_CFG")
            .field(
                "cpu_icm_h2x_post_wr_en",
                &format_args!("{}", self.cpu_icm_h2x_post_wr_en().bit()),
            )
            .field(
                "cpu_icm_h2x_cut_through_en",
                &format_args!("{}", self.cpu_icm_h2x_cut_through_en().bit()),
            )
            .field(
                "cpu_icm_h2x_bridge_busy",
                &format_args!("{}", self.cpu_icm_h2x_bridge_busy().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ICM_CPU_H2X_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn cpu_icm_h2x_post_wr_en(&mut self) -> CPU_ICM_H2X_POST_WR_EN_W<ICM_CPU_H2X_CFG_SPEC> {
        CPU_ICM_H2X_POST_WR_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn cpu_icm_h2x_cut_through_en(
        &mut self,
    ) -> CPU_ICM_H2X_CUT_THROUGH_EN_W<ICM_CPU_H2X_CFG_SPEC> {
        CPU_ICM_H2X_CUT_THROUGH_EN_W::new(self, 1)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icm_cpu_h2x_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icm_cpu_h2x_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICM_CPU_H2X_CFG_SPEC;
impl crate::RegisterSpec for ICM_CPU_H2X_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icm_cpu_h2x_cfg::R`](R) reader structure"]
impl crate::Readable for ICM_CPU_H2X_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`icm_cpu_h2x_cfg::W`](W) writer structure"]
impl crate::Writable for ICM_CPU_H2X_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICM_CPU_H2X_CFG to value 0x03"]
impl crate::Resettable for ICM_CPU_H2X_CFG_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
