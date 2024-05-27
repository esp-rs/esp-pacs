///Register `POWER_PD_HPCPU_CNTL` reader
pub type R = crate::R<POWER_PD_HPCPU_CNTL_SPEC>;
///Register `POWER_PD_HPCPU_CNTL` writer
pub type W = crate::W<POWER_PD_HPCPU_CNTL_SPEC>;
///Field `FORCE_HP_CPU_RESET` reader - need_des
pub type FORCE_HP_CPU_RESET_R = crate::BitReader;
///Field `FORCE_HP_CPU_RESET` writer - need_des
pub type FORCE_HP_CPU_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FORCE_HP_CPU_ISO` reader - need_des
pub type FORCE_HP_CPU_ISO_R = crate::BitReader;
///Field `FORCE_HP_CPU_ISO` writer - need_des
pub type FORCE_HP_CPU_ISO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FORCE_HP_CPU_PU` reader - need_des
pub type FORCE_HP_CPU_PU_R = crate::BitReader;
///Field `FORCE_HP_CPU_PU` writer - need_des
pub type FORCE_HP_CPU_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FORCE_HP_CPU_NO_RESET` reader - need_des
pub type FORCE_HP_CPU_NO_RESET_R = crate::BitReader;
///Field `FORCE_HP_CPU_NO_RESET` writer - need_des
pub type FORCE_HP_CPU_NO_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FORCE_HP_CPU_NO_ISO` reader - need_des
pub type FORCE_HP_CPU_NO_ISO_R = crate::BitReader;
///Field `FORCE_HP_CPU_NO_ISO` writer - need_des
pub type FORCE_HP_CPU_NO_ISO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FORCE_HP_CPU_PD` reader - need_des
pub type FORCE_HP_CPU_PD_R = crate::BitReader;
///Field `FORCE_HP_CPU_PD` writer - need_des
pub type FORCE_HP_CPU_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PD_HP_CPU_MASK` reader - need_des
pub type PD_HP_CPU_MASK_R = crate::FieldReader;
///Field `PD_HP_CPU_MASK` writer - need_des
pub type PD_HP_CPU_MASK_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `PD_HP_CPU_PD_MASK` reader - need_des
pub type PD_HP_CPU_PD_MASK_R = crate::FieldReader;
///Field `PD_HP_CPU_PD_MASK` writer - need_des
pub type PD_HP_CPU_PD_MASK_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bit 0 - need_des
    #[inline(always)]
    pub fn force_hp_cpu_reset(&self) -> FORCE_HP_CPU_RESET_R {
        FORCE_HP_CPU_RESET_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - need_des
    #[inline(always)]
    pub fn force_hp_cpu_iso(&self) -> FORCE_HP_CPU_ISO_R {
        FORCE_HP_CPU_ISO_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - need_des
    #[inline(always)]
    pub fn force_hp_cpu_pu(&self) -> FORCE_HP_CPU_PU_R {
        FORCE_HP_CPU_PU_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - need_des
    #[inline(always)]
    pub fn force_hp_cpu_no_reset(&self) -> FORCE_HP_CPU_NO_RESET_R {
        FORCE_HP_CPU_NO_RESET_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - need_des
    #[inline(always)]
    pub fn force_hp_cpu_no_iso(&self) -> FORCE_HP_CPU_NO_ISO_R {
        FORCE_HP_CPU_NO_ISO_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - need_des
    #[inline(always)]
    pub fn force_hp_cpu_pd(&self) -> FORCE_HP_CPU_PD_R {
        FORCE_HP_CPU_PD_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:10 - need_des
    #[inline(always)]
    pub fn pd_hp_cpu_mask(&self) -> PD_HP_CPU_MASK_R {
        PD_HP_CPU_MASK_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    ///Bits 27:31 - need_des
    #[inline(always)]
    pub fn pd_hp_cpu_pd_mask(&self) -> PD_HP_CPU_PD_MASK_R {
        PD_HP_CPU_PD_MASK_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POWER_PD_HPCPU_CNTL")
            .field("force_hp_cpu_reset", &self.force_hp_cpu_reset())
            .field("force_hp_cpu_iso", &self.force_hp_cpu_iso())
            .field("force_hp_cpu_pu", &self.force_hp_cpu_pu())
            .field("force_hp_cpu_no_reset", &self.force_hp_cpu_no_reset())
            .field("force_hp_cpu_no_iso", &self.force_hp_cpu_no_iso())
            .field("force_hp_cpu_pd", &self.force_hp_cpu_pd())
            .field("pd_hp_cpu_mask", &self.pd_hp_cpu_mask())
            .field("pd_hp_cpu_pd_mask", &self.pd_hp_cpu_pd_mask())
            .finish()
    }
}
impl W {
    ///Bit 0 - need_des
    #[inline(always)]
    #[must_use]
    pub fn force_hp_cpu_reset(&mut self) -> FORCE_HP_CPU_RESET_W<POWER_PD_HPCPU_CNTL_SPEC> {
        FORCE_HP_CPU_RESET_W::new(self, 0)
    }
    ///Bit 1 - need_des
    #[inline(always)]
    #[must_use]
    pub fn force_hp_cpu_iso(&mut self) -> FORCE_HP_CPU_ISO_W<POWER_PD_HPCPU_CNTL_SPEC> {
        FORCE_HP_CPU_ISO_W::new(self, 1)
    }
    ///Bit 2 - need_des
    #[inline(always)]
    #[must_use]
    pub fn force_hp_cpu_pu(&mut self) -> FORCE_HP_CPU_PU_W<POWER_PD_HPCPU_CNTL_SPEC> {
        FORCE_HP_CPU_PU_W::new(self, 2)
    }
    ///Bit 3 - need_des
    #[inline(always)]
    #[must_use]
    pub fn force_hp_cpu_no_reset(&mut self) -> FORCE_HP_CPU_NO_RESET_W<POWER_PD_HPCPU_CNTL_SPEC> {
        FORCE_HP_CPU_NO_RESET_W::new(self, 3)
    }
    ///Bit 4 - need_des
    #[inline(always)]
    #[must_use]
    pub fn force_hp_cpu_no_iso(&mut self) -> FORCE_HP_CPU_NO_ISO_W<POWER_PD_HPCPU_CNTL_SPEC> {
        FORCE_HP_CPU_NO_ISO_W::new(self, 4)
    }
    ///Bit 5 - need_des
    #[inline(always)]
    #[must_use]
    pub fn force_hp_cpu_pd(&mut self) -> FORCE_HP_CPU_PD_W<POWER_PD_HPCPU_CNTL_SPEC> {
        FORCE_HP_CPU_PD_W::new(self, 5)
    }
    ///Bits 6:10 - need_des
    #[inline(always)]
    #[must_use]
    pub fn pd_hp_cpu_mask(&mut self) -> PD_HP_CPU_MASK_W<POWER_PD_HPCPU_CNTL_SPEC> {
        PD_HP_CPU_MASK_W::new(self, 6)
    }
    ///Bits 27:31 - need_des
    #[inline(always)]
    #[must_use]
    pub fn pd_hp_cpu_pd_mask(&mut self) -> PD_HP_CPU_PD_MASK_W<POWER_PD_HPCPU_CNTL_SPEC> {
        PD_HP_CPU_PD_MASK_W::new(self, 27)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`power_pd_hpcpu_cntl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power_pd_hpcpu_cntl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct POWER_PD_HPCPU_CNTL_SPEC;
impl crate::RegisterSpec for POWER_PD_HPCPU_CNTL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`power_pd_hpcpu_cntl::R`](R) reader structure
impl crate::Readable for POWER_PD_HPCPU_CNTL_SPEC {}
///`write(|w| ..)` method takes [`power_pd_hpcpu_cntl::W`](W) writer structure
impl crate::Writable for POWER_PD_HPCPU_CNTL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets POWER_PD_HPCPU_CNTL to value 0x1c
impl crate::Resettable for POWER_PD_HPCPU_CNTL_SPEC {
    const RESET_VALUE: u32 = 0x1c;
}
