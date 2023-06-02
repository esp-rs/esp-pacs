#[doc = "Register `POWER_PD_MEM_CNTL` reader"]
pub struct R(crate::R<POWER_PD_MEM_CNTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POWER_PD_MEM_CNTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POWER_PD_MEM_CNTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POWER_PD_MEM_CNTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POWER_PD_MEM_CNTL` writer"]
pub struct W(crate::W<POWER_PD_MEM_CNTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POWER_PD_MEM_CNTL_SPEC>;
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
impl From<crate::W<POWER_PD_MEM_CNTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POWER_PD_MEM_CNTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FORCE_HP_MEM_ISO` reader - need_des"]
pub type FORCE_HP_MEM_ISO_R = crate::FieldReader;
#[doc = "Field `FORCE_HP_MEM_ISO` writer - need_des"]
pub type FORCE_HP_MEM_ISO_W<'a, const O: u8> = crate::FieldWriter<'a, POWER_PD_MEM_CNTL_SPEC, 4, O>;
#[doc = "Field `FORCE_HP_MEM_PD` reader - need_des"]
pub type FORCE_HP_MEM_PD_R = crate::FieldReader;
#[doc = "Field `FORCE_HP_MEM_PD` writer - need_des"]
pub type FORCE_HP_MEM_PD_W<'a, const O: u8> = crate::FieldWriter<'a, POWER_PD_MEM_CNTL_SPEC, 4, O>;
#[doc = "Field `FORCE_HP_MEM_NO_ISO` reader - need_des"]
pub type FORCE_HP_MEM_NO_ISO_R = crate::FieldReader;
#[doc = "Field `FORCE_HP_MEM_NO_ISO` writer - need_des"]
pub type FORCE_HP_MEM_NO_ISO_W<'a, const O: u8> =
    crate::FieldWriter<'a, POWER_PD_MEM_CNTL_SPEC, 4, O>;
#[doc = "Field `FORCE_HP_MEM_PU` reader - need_des"]
pub type FORCE_HP_MEM_PU_R = crate::FieldReader;
#[doc = "Field `FORCE_HP_MEM_PU` writer - need_des"]
pub type FORCE_HP_MEM_PU_W<'a, const O: u8> = crate::FieldWriter<'a, POWER_PD_MEM_CNTL_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:3 - need_des"]
    #[inline(always)]
    pub fn force_hp_mem_iso(&self) -> FORCE_HP_MEM_ISO_R {
        FORCE_HP_MEM_ISO_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - need_des"]
    #[inline(always)]
    pub fn force_hp_mem_pd(&self) -> FORCE_HP_MEM_PD_R {
        FORCE_HP_MEM_PD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - need_des"]
    #[inline(always)]
    pub fn force_hp_mem_no_iso(&self) -> FORCE_HP_MEM_NO_ISO_R {
        FORCE_HP_MEM_NO_ISO_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - need_des"]
    #[inline(always)]
    pub fn force_hp_mem_pu(&self) -> FORCE_HP_MEM_PU_R {
        FORCE_HP_MEM_PU_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POWER_PD_MEM_CNTL")
            .field(
                "force_hp_mem_iso",
                &format_args!("{}", self.force_hp_mem_iso().bits()),
            )
            .field(
                "force_hp_mem_pd",
                &format_args!("{}", self.force_hp_mem_pd().bits()),
            )
            .field(
                "force_hp_mem_no_iso",
                &format_args!("{}", self.force_hp_mem_no_iso().bits()),
            )
            .field(
                "force_hp_mem_pu",
                &format_args!("{}", self.force_hp_mem_pu().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<POWER_PD_MEM_CNTL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:3 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn force_hp_mem_iso(&mut self) -> FORCE_HP_MEM_ISO_W<0> {
        FORCE_HP_MEM_ISO_W::new(self)
    }
    #[doc = "Bits 4:7 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn force_hp_mem_pd(&mut self) -> FORCE_HP_MEM_PD_W<4> {
        FORCE_HP_MEM_PD_W::new(self)
    }
    #[doc = "Bits 24:27 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn force_hp_mem_no_iso(&mut self) -> FORCE_HP_MEM_NO_ISO_W<24> {
        FORCE_HP_MEM_NO_ISO_W::new(self)
    }
    #[doc = "Bits 28:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn force_hp_mem_pu(&mut self) -> FORCE_HP_MEM_PU_W<28> {
        FORCE_HP_MEM_PU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [power_pd_mem_cntl](index.html) module"]
pub struct POWER_PD_MEM_CNTL_SPEC;
impl crate::RegisterSpec for POWER_PD_MEM_CNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [power_pd_mem_cntl::R](R) reader structure"]
impl crate::Readable for POWER_PD_MEM_CNTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [power_pd_mem_cntl::W](W) writer structure"]
impl crate::Writable for POWER_PD_MEM_CNTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets POWER_PD_MEM_CNTL to value 0xff00_0000"]
impl crate::Resettable for POWER_PD_MEM_CNTL_SPEC {
    const RESET_VALUE: Self::Ux = 0xff00_0000;
}
