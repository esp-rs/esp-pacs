#[doc = "Register `REGDMA_CONF` reader"]
pub struct R(crate::R<REGDMA_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REGDMA_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REGDMA_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REGDMA_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REGDMA_CONF` writer"]
pub struct W(crate::W<REGDMA_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REGDMA_CONF_SPEC>;
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
impl From<crate::W<REGDMA_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REGDMA_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLOW_ERR` reader - backup error type"]
pub type FLOW_ERR_R = crate::FieldReader;
#[doc = "Field `START` writer - backup start signal"]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, REGDMA_CONF_SPEC, O>;
#[doc = "Field `TO_MEM` reader - backup direction(reg to mem / mem to reg)"]
pub type TO_MEM_R = crate::BitReader;
#[doc = "Field `TO_MEM` writer - backup direction(reg to mem / mem to reg)"]
pub type TO_MEM_W<'a, const O: u8> = crate::BitWriter<'a, REGDMA_CONF_SPEC, O>;
#[doc = "Field `LINK_SEL` reader - Link select"]
pub type LINK_SEL_R = crate::FieldReader;
#[doc = "Field `LINK_SEL` writer - Link select"]
pub type LINK_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, REGDMA_CONF_SPEC, 2, O>;
#[doc = "Field `START_MAC` writer - mac sw backup start signal"]
pub type START_MAC_W<'a, const O: u8> = crate::BitWriter<'a, REGDMA_CONF_SPEC, O>;
#[doc = "Field `TO_MEM_MAC` reader - mac sw backup direction(reg to mem / mem to reg)"]
pub type TO_MEM_MAC_R = crate::BitReader;
#[doc = "Field `TO_MEM_MAC` writer - mac sw backup direction(reg to mem / mem to reg)"]
pub type TO_MEM_MAC_W<'a, const O: u8> = crate::BitWriter<'a, REGDMA_CONF_SPEC, O>;
#[doc = "Field `SEL_MAC` reader - mac hw/sw select"]
pub type SEL_MAC_R = crate::BitReader;
#[doc = "Field `SEL_MAC` writer - mac hw/sw select"]
pub type SEL_MAC_W<'a, const O: u8> = crate::BitWriter<'a, REGDMA_CONF_SPEC, O>;
impl R {
    #[doc = "Bits 0:2 - backup error type"]
    #[inline(always)]
    pub fn flow_err(&self) -> FLOW_ERR_R {
        FLOW_ERR_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - backup direction(reg to mem / mem to reg)"]
    #[inline(always)]
    pub fn to_mem(&self) -> TO_MEM_R {
        TO_MEM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Link select"]
    #[inline(always)]
    pub fn link_sel(&self) -> LINK_SEL_R {
        LINK_SEL_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 8 - mac sw backup direction(reg to mem / mem to reg)"]
    #[inline(always)]
    pub fn to_mem_mac(&self) -> TO_MEM_MAC_R {
        TO_MEM_MAC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - mac hw/sw select"]
    #[inline(always)]
    pub fn sel_mac(&self) -> SEL_MAC_R {
        SEL_MAC_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGDMA_CONF")
            .field("flow_err", &format_args!("{}", self.flow_err().bits()))
            .field("to_mem", &format_args!("{}", self.to_mem().bit()))
            .field("link_sel", &format_args!("{}", self.link_sel().bits()))
            .field("to_mem_mac", &format_args!("{}", self.to_mem_mac().bit()))
            .field("sel_mac", &format_args!("{}", self.sel_mac().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REGDMA_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 3 - backup start signal"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<3> {
        START_W::new(self)
    }
    #[doc = "Bit 4 - backup direction(reg to mem / mem to reg)"]
    #[inline(always)]
    #[must_use]
    pub fn to_mem(&mut self) -> TO_MEM_W<4> {
        TO_MEM_W::new(self)
    }
    #[doc = "Bits 5:6 - Link select"]
    #[inline(always)]
    #[must_use]
    pub fn link_sel(&mut self) -> LINK_SEL_W<5> {
        LINK_SEL_W::new(self)
    }
    #[doc = "Bit 7 - mac sw backup start signal"]
    #[inline(always)]
    #[must_use]
    pub fn start_mac(&mut self) -> START_MAC_W<7> {
        START_MAC_W::new(self)
    }
    #[doc = "Bit 8 - mac sw backup direction(reg to mem / mem to reg)"]
    #[inline(always)]
    #[must_use]
    pub fn to_mem_mac(&mut self) -> TO_MEM_MAC_W<8> {
        TO_MEM_MAC_W::new(self)
    }
    #[doc = "Bit 9 - mac hw/sw select"]
    #[inline(always)]
    #[must_use]
    pub fn sel_mac(&mut self) -> SEL_MAC_W<9> {
        SEL_MAC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peri backup control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [regdma_conf](index.html) module"]
pub struct REGDMA_CONF_SPEC;
impl crate::RegisterSpec for REGDMA_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [regdma_conf::R](R) reader structure"]
impl crate::Readable for REGDMA_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [regdma_conf::W](W) writer structure"]
impl crate::Writable for REGDMA_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REGDMA_CONF to value 0"]
impl crate::Resettable for REGDMA_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
