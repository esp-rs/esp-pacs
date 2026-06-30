#[doc = "Register `REMOVE_TMP0` reader"]
pub type R = crate::R<REMOVE_TMP0_SPEC>;
#[doc = "Register `REMOVE_TMP0` writer"]
pub type W = crate::W<REMOVE_TMP0_SPEC>;
#[doc = "Field `REG_GDMA_CPU_CLK_EN` reader - need_des"]
pub type REG_GDMA_CPU_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_GDMA_CPU_CLK_EN` writer - need_des"]
pub type REG_GDMA_CPU_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_VPU_CPU_CLK_EN` reader - need_des"]
pub type REG_VPU_CPU_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_VPU_CPU_CLK_EN` writer - need_des"]
pub type REG_VPU_CPU_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_VPU_SYS_CLK_EN` reader - need_des"]
pub type REG_VPU_SYS_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_VPU_SYS_CLK_EN` writer - need_des"]
pub type REG_VPU_SYS_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn reg_gdma_cpu_clk_en(&self) -> REG_GDMA_CPU_CLK_EN_R {
        REG_GDMA_CPU_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn reg_vpu_cpu_clk_en(&self) -> REG_VPU_CPU_CLK_EN_R {
        REG_VPU_CPU_CLK_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn reg_vpu_sys_clk_en(&self) -> REG_VPU_SYS_CLK_EN_R {
        REG_VPU_SYS_CLK_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REMOVE_TMP0")
            .field("reg_gdma_cpu_clk_en", &self.reg_gdma_cpu_clk_en())
            .field("reg_vpu_cpu_clk_en", &self.reg_vpu_cpu_clk_en())
            .field("reg_vpu_sys_clk_en", &self.reg_vpu_sys_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn reg_gdma_cpu_clk_en(&mut self) -> REG_GDMA_CPU_CLK_EN_W<'_, REMOVE_TMP0_SPEC> {
        REG_GDMA_CPU_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn reg_vpu_cpu_clk_en(&mut self) -> REG_VPU_CPU_CLK_EN_W<'_, REMOVE_TMP0_SPEC> {
        REG_VPU_CPU_CLK_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn reg_vpu_sys_clk_en(&mut self) -> REG_VPU_SYS_CLK_EN_W<'_, REMOVE_TMP0_SPEC> {
        REG_VPU_SYS_CLK_EN_W::new(self, 2)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`remove_tmp0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`remove_tmp0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REMOVE_TMP0_SPEC;
impl crate::RegisterSpec for REMOVE_TMP0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`remove_tmp0::R`](R) reader structure"]
impl crate::Readable for REMOVE_TMP0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`remove_tmp0::W`](W) writer structure"]
impl crate::Writable for REMOVE_TMP0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REMOVE_TMP0 to value 0"]
impl crate::Resettable for REMOVE_TMP0_SPEC {}
