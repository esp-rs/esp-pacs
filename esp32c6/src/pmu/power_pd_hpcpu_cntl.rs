#[doc = "Register `POWER_PD_HPCPU_CNTL` reader"]
pub type R = crate::R<POWER_PD_HPCPU_CNTL_SPEC>;
#[doc = "Register `POWER_PD_HPCPU_CNTL` writer"]
pub type W = crate::W<POWER_PD_HPCPU_CNTL_SPEC>;
#[doc = "Field `FORCE_HP_CPU_RESET` reader - need_des"]
pub type FORCE_HP_CPU_RESET_R = crate::BitReader;
#[doc = "Field `FORCE_HP_CPU_RESET` writer - need_des"]
pub type FORCE_HP_CPU_RESET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FORCE_HP_CPU_ISO` reader - need_des"]
pub type FORCE_HP_CPU_ISO_R = crate::BitReader;
#[doc = "Field `FORCE_HP_CPU_ISO` writer - need_des"]
pub type FORCE_HP_CPU_ISO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FORCE_HP_CPU_PU` reader - need_des"]
pub type FORCE_HP_CPU_PU_R = crate::BitReader;
#[doc = "Field `FORCE_HP_CPU_PU` writer - need_des"]
pub type FORCE_HP_CPU_PU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FORCE_HP_CPU_NO_RESET` reader - need_des"]
pub type FORCE_HP_CPU_NO_RESET_R = crate::BitReader;
#[doc = "Field `FORCE_HP_CPU_NO_RESET` writer - need_des"]
pub type FORCE_HP_CPU_NO_RESET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FORCE_HP_CPU_NO_ISO` reader - need_des"]
pub type FORCE_HP_CPU_NO_ISO_R = crate::BitReader;
#[doc = "Field `FORCE_HP_CPU_NO_ISO` writer - need_des"]
pub type FORCE_HP_CPU_NO_ISO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FORCE_HP_CPU_PD` reader - need_des"]
pub type FORCE_HP_CPU_PD_R = crate::BitReader;
#[doc = "Field `FORCE_HP_CPU_PD` writer - need_des"]
pub type FORCE_HP_CPU_PD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PD_HP_CPU_MASK` reader - need_des"]
pub type PD_HP_CPU_MASK_R = crate::FieldReader;
#[doc = "Field `PD_HP_CPU_MASK` writer - need_des"]
pub type PD_HP_CPU_MASK_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `PD_HP_CPU_PD_MASK` reader - need_des"]
pub type PD_HP_CPU_PD_MASK_R = crate::FieldReader;
#[doc = "Field `PD_HP_CPU_PD_MASK` writer - need_des"]
pub type PD_HP_CPU_PD_MASK_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn force_hp_cpu_reset(&self) -> FORCE_HP_CPU_RESET_R {
        FORCE_HP_CPU_RESET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn force_hp_cpu_iso(&self) -> FORCE_HP_CPU_ISO_R {
        FORCE_HP_CPU_ISO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn force_hp_cpu_pu(&self) -> FORCE_HP_CPU_PU_R {
        FORCE_HP_CPU_PU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn force_hp_cpu_no_reset(&self) -> FORCE_HP_CPU_NO_RESET_R {
        FORCE_HP_CPU_NO_RESET_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn force_hp_cpu_no_iso(&self) -> FORCE_HP_CPU_NO_ISO_R {
        FORCE_HP_CPU_NO_ISO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn force_hp_cpu_pd(&self) -> FORCE_HP_CPU_PD_R {
        FORCE_HP_CPU_PD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:10 - need_des"]
    #[inline(always)]
    pub fn pd_hp_cpu_mask(&self) -> PD_HP_CPU_MASK_R {
        PD_HP_CPU_MASK_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - need_des"]
    #[inline(always)]
    pub fn pd_hp_cpu_pd_mask(&self) -> PD_HP_CPU_PD_MASK_R {
        PD_HP_CPU_PD_MASK_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POWER_PD_HPCPU_CNTL")
            .field(
                "force_hp_cpu_reset",
                &format_args!("{}", self.force_hp_cpu_reset().bit()),
            )
            .field(
                "force_hp_cpu_iso",
                &format_args!("{}", self.force_hp_cpu_iso().bit()),
            )
            .field(
                "force_hp_cpu_pu",
                &format_args!("{}", self.force_hp_cpu_pu().bit()),
            )
            .field(
                "force_hp_cpu_no_reset",
                &format_args!("{}", self.force_hp_cpu_no_reset().bit()),
            )
            .field(
                "force_hp_cpu_no_iso",
                &format_args!("{}", self.force_hp_cpu_no_iso().bit()),
            )
            .field(
                "force_hp_cpu_pd",
                &format_args!("{}", self.force_hp_cpu_pd().bit()),
            )
            .field(
                "pd_hp_cpu_mask",
                &format_args!("{}", self.pd_hp_cpu_mask().bits()),
            )
            .field(
                "pd_hp_cpu_pd_mask",
                &format_args!("{}", self.pd_hp_cpu_pd_mask().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<POWER_PD_HPCPU_CNTL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn force_hp_cpu_reset(&mut self) -> FORCE_HP_CPU_RESET_W<POWER_PD_HPCPU_CNTL_SPEC, 0> {
        FORCE_HP_CPU_RESET_W::new(self)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn force_hp_cpu_iso(&mut self) -> FORCE_HP_CPU_ISO_W<POWER_PD_HPCPU_CNTL_SPEC, 1> {
        FORCE_HP_CPU_ISO_W::new(self)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn force_hp_cpu_pu(&mut self) -> FORCE_HP_CPU_PU_W<POWER_PD_HPCPU_CNTL_SPEC, 2> {
        FORCE_HP_CPU_PU_W::new(self)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn force_hp_cpu_no_reset(
        &mut self,
    ) -> FORCE_HP_CPU_NO_RESET_W<POWER_PD_HPCPU_CNTL_SPEC, 3> {
        FORCE_HP_CPU_NO_RESET_W::new(self)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn force_hp_cpu_no_iso(&mut self) -> FORCE_HP_CPU_NO_ISO_W<POWER_PD_HPCPU_CNTL_SPEC, 4> {
        FORCE_HP_CPU_NO_ISO_W::new(self)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn force_hp_cpu_pd(&mut self) -> FORCE_HP_CPU_PD_W<POWER_PD_HPCPU_CNTL_SPEC, 5> {
        FORCE_HP_CPU_PD_W::new(self)
    }
    #[doc = "Bits 6:10 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn pd_hp_cpu_mask(&mut self) -> PD_HP_CPU_MASK_W<POWER_PD_HPCPU_CNTL_SPEC, 6> {
        PD_HP_CPU_MASK_W::new(self)
    }
    #[doc = "Bits 27:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn pd_hp_cpu_pd_mask(&mut self) -> PD_HP_CPU_PD_MASK_W<POWER_PD_HPCPU_CNTL_SPEC, 27> {
        PD_HP_CPU_PD_MASK_W::new(self)
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
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power_pd_hpcpu_cntl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power_pd_hpcpu_cntl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POWER_PD_HPCPU_CNTL_SPEC;
impl crate::RegisterSpec for POWER_PD_HPCPU_CNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`power_pd_hpcpu_cntl::R`](R) reader structure"]
impl crate::Readable for POWER_PD_HPCPU_CNTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`power_pd_hpcpu_cntl::W`](W) writer structure"]
impl crate::Writable for POWER_PD_HPCPU_CNTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets POWER_PD_HPCPU_CNTL to value 0x1c"]
impl crate::Resettable for POWER_PD_HPCPU_CNTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x1c;
}
