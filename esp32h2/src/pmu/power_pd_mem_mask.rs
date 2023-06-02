#[doc = "Register `POWER_PD_MEM_MASK` reader"]
pub struct R(crate::R<POWER_PD_MEM_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POWER_PD_MEM_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POWER_PD_MEM_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POWER_PD_MEM_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POWER_PD_MEM_MASK` writer"]
pub struct W(crate::W<POWER_PD_MEM_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POWER_PD_MEM_MASK_SPEC>;
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
impl From<crate::W<POWER_PD_MEM_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POWER_PD_MEM_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PD_HP_MEM2_PD_MASK` reader - need_des"]
pub type PD_HP_MEM2_PD_MASK_R = crate::FieldReader;
#[doc = "Field `PD_HP_MEM2_PD_MASK` writer - need_des"]
pub type PD_HP_MEM2_PD_MASK_W<'a, const O: u8> =
    crate::FieldWriter<'a, POWER_PD_MEM_MASK_SPEC, 5, O>;
#[doc = "Field `PD_HP_MEM1_PD_MASK` reader - need_des"]
pub type PD_HP_MEM1_PD_MASK_R = crate::FieldReader;
#[doc = "Field `PD_HP_MEM1_PD_MASK` writer - need_des"]
pub type PD_HP_MEM1_PD_MASK_W<'a, const O: u8> =
    crate::FieldWriter<'a, POWER_PD_MEM_MASK_SPEC, 5, O>;
#[doc = "Field `PD_HP_MEM0_PD_MASK` reader - need_des"]
pub type PD_HP_MEM0_PD_MASK_R = crate::FieldReader;
#[doc = "Field `PD_HP_MEM0_PD_MASK` writer - need_des"]
pub type PD_HP_MEM0_PD_MASK_W<'a, const O: u8> =
    crate::FieldWriter<'a, POWER_PD_MEM_MASK_SPEC, 5, O>;
#[doc = "Field `PD_HP_MEM2_MASK` reader - need_des"]
pub type PD_HP_MEM2_MASK_R = crate::FieldReader;
#[doc = "Field `PD_HP_MEM2_MASK` writer - need_des"]
pub type PD_HP_MEM2_MASK_W<'a, const O: u8> = crate::FieldWriter<'a, POWER_PD_MEM_MASK_SPEC, 5, O>;
#[doc = "Field `PD_HP_MEM1_MASK` reader - need_des"]
pub type PD_HP_MEM1_MASK_R = crate::FieldReader;
#[doc = "Field `PD_HP_MEM1_MASK` writer - need_des"]
pub type PD_HP_MEM1_MASK_W<'a, const O: u8> = crate::FieldWriter<'a, POWER_PD_MEM_MASK_SPEC, 5, O>;
#[doc = "Field `PD_HP_MEM0_MASK` reader - need_des"]
pub type PD_HP_MEM0_MASK_R = crate::FieldReader;
#[doc = "Field `PD_HP_MEM0_MASK` writer - need_des"]
pub type PD_HP_MEM0_MASK_W<'a, const O: u8> = crate::FieldWriter<'a, POWER_PD_MEM_MASK_SPEC, 5, O>;
impl R {
    #[doc = "Bits 0:4 - need_des"]
    #[inline(always)]
    pub fn pd_hp_mem2_pd_mask(&self) -> PD_HP_MEM2_PD_MASK_R {
        PD_HP_MEM2_PD_MASK_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - need_des"]
    #[inline(always)]
    pub fn pd_hp_mem1_pd_mask(&self) -> PD_HP_MEM1_PD_MASK_R {
        PD_HP_MEM1_PD_MASK_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - need_des"]
    #[inline(always)]
    pub fn pd_hp_mem0_pd_mask(&self) -> PD_HP_MEM0_PD_MASK_R {
        PD_HP_MEM0_PD_MASK_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 17:21 - need_des"]
    #[inline(always)]
    pub fn pd_hp_mem2_mask(&self) -> PD_HP_MEM2_MASK_R {
        PD_HP_MEM2_MASK_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bits 22:26 - need_des"]
    #[inline(always)]
    pub fn pd_hp_mem1_mask(&self) -> PD_HP_MEM1_MASK_R {
        PD_HP_MEM1_MASK_R::new(((self.bits >> 22) & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - need_des"]
    #[inline(always)]
    pub fn pd_hp_mem0_mask(&self) -> PD_HP_MEM0_MASK_R {
        PD_HP_MEM0_MASK_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POWER_PD_MEM_MASK")
            .field(
                "pd_hp_mem2_pd_mask",
                &format_args!("{}", self.pd_hp_mem2_pd_mask().bits()),
            )
            .field(
                "pd_hp_mem1_pd_mask",
                &format_args!("{}", self.pd_hp_mem1_pd_mask().bits()),
            )
            .field(
                "pd_hp_mem0_pd_mask",
                &format_args!("{}", self.pd_hp_mem0_pd_mask().bits()),
            )
            .field(
                "pd_hp_mem2_mask",
                &format_args!("{}", self.pd_hp_mem2_mask().bits()),
            )
            .field(
                "pd_hp_mem1_mask",
                &format_args!("{}", self.pd_hp_mem1_mask().bits()),
            )
            .field(
                "pd_hp_mem0_mask",
                &format_args!("{}", self.pd_hp_mem0_mask().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<POWER_PD_MEM_MASK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:4 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn pd_hp_mem2_pd_mask(&mut self) -> PD_HP_MEM2_PD_MASK_W<0> {
        PD_HP_MEM2_PD_MASK_W::new(self)
    }
    #[doc = "Bits 5:9 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn pd_hp_mem1_pd_mask(&mut self) -> PD_HP_MEM1_PD_MASK_W<5> {
        PD_HP_MEM1_PD_MASK_W::new(self)
    }
    #[doc = "Bits 10:14 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn pd_hp_mem0_pd_mask(&mut self) -> PD_HP_MEM0_PD_MASK_W<10> {
        PD_HP_MEM0_PD_MASK_W::new(self)
    }
    #[doc = "Bits 17:21 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn pd_hp_mem2_mask(&mut self) -> PD_HP_MEM2_MASK_W<17> {
        PD_HP_MEM2_MASK_W::new(self)
    }
    #[doc = "Bits 22:26 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn pd_hp_mem1_mask(&mut self) -> PD_HP_MEM1_MASK_W<22> {
        PD_HP_MEM1_MASK_W::new(self)
    }
    #[doc = "Bits 27:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn pd_hp_mem0_mask(&mut self) -> PD_HP_MEM0_MASK_W<27> {
        PD_HP_MEM0_MASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [power_pd_mem_mask](index.html) module"]
pub struct POWER_PD_MEM_MASK_SPEC;
impl crate::RegisterSpec for POWER_PD_MEM_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [power_pd_mem_mask::R](R) reader structure"]
impl crate::Readable for POWER_PD_MEM_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [power_pd_mem_mask::W](W) writer structure"]
impl crate::Writable for POWER_PD_MEM_MASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets POWER_PD_MEM_MASK to value 0"]
impl crate::Resettable for POWER_PD_MEM_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
