#[doc = "Register `PD_CONF` reader"]
pub struct R(crate::R<PD_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PD_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PD_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PD_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PD_CONF` writer"]
pub struct W(crate::W<PD_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PD_CONF_SPEC>;
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
impl From<crate::W<PD_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PD_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFO_FORCE_PD` reader - "]
pub type FIFO_FORCE_PD_R = crate::BitReader;
#[doc = "Field `FIFO_FORCE_PD` writer - "]
pub type FIFO_FORCE_PD_W<'a, const O: u8> = crate::BitWriter<'a, PD_CONF_SPEC, O>;
#[doc = "Field `FIFO_FORCE_PU` reader - "]
pub type FIFO_FORCE_PU_R = crate::BitReader;
#[doc = "Field `FIFO_FORCE_PU` writer - "]
pub type FIFO_FORCE_PU_W<'a, const O: u8> = crate::BitWriter<'a, PD_CONF_SPEC, O>;
#[doc = "Field `PLC_MEM_FORCE_PD` reader - "]
pub type PLC_MEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `PLC_MEM_FORCE_PD` writer - "]
pub type PLC_MEM_FORCE_PD_W<'a, const O: u8> = crate::BitWriter<'a, PD_CONF_SPEC, O>;
#[doc = "Field `PLC_MEM_FORCE_PU` reader - "]
pub type PLC_MEM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `PLC_MEM_FORCE_PU` writer - "]
pub type PLC_MEM_FORCE_PU_W<'a, const O: u8> = crate::BitWriter<'a, PD_CONF_SPEC, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn fifo_force_pd(&self) -> FIFO_FORCE_PD_R {
        FIFO_FORCE_PD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn fifo_force_pu(&self) -> FIFO_FORCE_PU_R {
        FIFO_FORCE_PU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn plc_mem_force_pd(&self) -> PLC_MEM_FORCE_PD_R {
        PLC_MEM_FORCE_PD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn plc_mem_force_pu(&self) -> PLC_MEM_FORCE_PU_R {
        PLC_MEM_FORCE_PU_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PD_CONF")
            .field(
                "fifo_force_pd",
                &format_args!("{}", self.fifo_force_pd().bit()),
            )
            .field(
                "fifo_force_pu",
                &format_args!("{}", self.fifo_force_pu().bit()),
            )
            .field(
                "plc_mem_force_pd",
                &format_args!("{}", self.plc_mem_force_pd().bit()),
            )
            .field(
                "plc_mem_force_pu",
                &format_args!("{}", self.plc_mem_force_pu().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PD_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_force_pd(&mut self) -> FIFO_FORCE_PD_W<0> {
        FIFO_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_force_pu(&mut self) -> FIFO_FORCE_PU_W<1> {
        FIFO_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn plc_mem_force_pd(&mut self) -> PLC_MEM_FORCE_PD_W<2> {
        PLC_MEM_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn plc_mem_force_pu(&mut self) -> PLC_MEM_FORCE_PU_W<3> {
        PLC_MEM_FORCE_PU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pd_conf](index.html) module"]
pub struct PD_CONF_SPEC;
impl crate::RegisterSpec for PD_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pd_conf::R](R) reader structure"]
impl crate::Readable for PD_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pd_conf::W](W) writer structure"]
impl crate::Writable for PD_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PD_CONF to value 0x0a"]
impl crate::Resettable for PD_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0a;
}
