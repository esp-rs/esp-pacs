#[doc = "Register `CPUCORE0_CFG` reader"]
pub struct R(crate::R<CPUCORE0_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPUCORE0_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPUCORE0_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPUCORE0_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPUCORE0_CFG` writer"]
pub struct W(crate::W<CPUCORE0_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPUCORE0_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CPUCORE0_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPUCORE0_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPU_CORE0_SW_STALL` reader - need_des"]
pub type CPU_CORE0_SW_STALL_R = crate::FieldReader;
#[doc = "Field `CPU_CORE0_SW_STALL` writer - need_des"]
pub type CPU_CORE0_SW_STALL_W<'a, const O: u8> = crate::FieldWriter<'a, CPUCORE0_CFG_SPEC, 8, O>;
#[doc = "Field `CPU_CORE0_SW_RESET` writer - need_des"]
pub type CPU_CORE0_SW_RESET_W<'a, const O: u8> = crate::BitWriter<'a, CPUCORE0_CFG_SPEC, O>;
#[doc = "Field `CPU_CORE0_OCD_HALT_ON_RESET` reader - need_des"]
pub type CPU_CORE0_OCD_HALT_ON_RESET_R = crate::BitReader;
#[doc = "Field `CPU_CORE0_OCD_HALT_ON_RESET` writer - need_des"]
pub type CPU_CORE0_OCD_HALT_ON_RESET_W<'a, const O: u8> =
    crate::BitWriter<'a, CPUCORE0_CFG_SPEC, O>;
#[doc = "Field `CPU_CORE0_STAT_VECTOR_SEL` reader - need_des"]
pub type CPU_CORE0_STAT_VECTOR_SEL_R = crate::BitReader;
#[doc = "Field `CPU_CORE0_STAT_VECTOR_SEL` writer - need_des"]
pub type CPU_CORE0_STAT_VECTOR_SEL_W<'a, const O: u8> = crate::BitWriter<'a, CPUCORE0_CFG_SPEC, O>;
#[doc = "Field `CPU_CORE0_DRESET_MASK` reader - need_des"]
pub type CPU_CORE0_DRESET_MASK_R = crate::BitReader;
#[doc = "Field `CPU_CORE0_DRESET_MASK` writer - need_des"]
pub type CPU_CORE0_DRESET_MASK_W<'a, const O: u8> = crate::BitWriter<'a, CPUCORE0_CFG_SPEC, O>;
impl R {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    pub fn cpu_core0_sw_stall(&self) -> CPU_CORE0_SW_STALL_R {
        CPU_CORE0_SW_STALL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn cpu_core0_ocd_halt_on_reset(&self) -> CPU_CORE0_OCD_HALT_ON_RESET_R {
        CPU_CORE0_OCD_HALT_ON_RESET_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn cpu_core0_stat_vector_sel(&self) -> CPU_CORE0_STAT_VECTOR_SEL_R {
        CPU_CORE0_STAT_VECTOR_SEL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn cpu_core0_dreset_mask(&self) -> CPU_CORE0_DRESET_MASK_R {
        CPU_CORE0_DRESET_MASK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPUCORE0_CFG")
            .field(
                "cpu_core0_sw_stall",
                &format_args!("{}", self.cpu_core0_sw_stall().bits()),
            )
            .field(
                "cpu_core0_ocd_halt_on_reset",
                &format_args!("{}", self.cpu_core0_ocd_halt_on_reset().bit()),
            )
            .field(
                "cpu_core0_stat_vector_sel",
                &format_args!("{}", self.cpu_core0_stat_vector_sel().bit()),
            )
            .field(
                "cpu_core0_dreset_mask",
                &format_args!("{}", self.cpu_core0_dreset_mask().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CPUCORE0_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn cpu_core0_sw_stall(&mut self) -> CPU_CORE0_SW_STALL_W<0> {
        CPU_CORE0_SW_STALL_W::new(self)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn cpu_core0_sw_reset(&mut self) -> CPU_CORE0_SW_RESET_W<28> {
        CPU_CORE0_SW_RESET_W::new(self)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn cpu_core0_ocd_halt_on_reset(&mut self) -> CPU_CORE0_OCD_HALT_ON_RESET_W<29> {
        CPU_CORE0_OCD_HALT_ON_RESET_W::new(self)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn cpu_core0_stat_vector_sel(&mut self) -> CPU_CORE0_STAT_VECTOR_SEL_W<30> {
        CPU_CORE0_STAT_VECTOR_SEL_W::new(self)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn cpu_core0_dreset_mask(&mut self) -> CPU_CORE0_DRESET_MASK_W<31> {
        CPU_CORE0_DRESET_MASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpucore0_cfg](index.html) module"]
pub struct CPUCORE0_CFG_SPEC;
impl crate::RegisterSpec for CPUCORE0_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpucore0_cfg::R](R) reader structure"]
impl crate::Readable for CPUCORE0_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpucore0_cfg::W](W) writer structure"]
impl crate::Writable for CPUCORE0_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPUCORE0_CFG to value 0x4000_0000"]
impl crate::Resettable for CPUCORE0_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x4000_0000;
}
