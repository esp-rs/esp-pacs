#[doc = "Register `MEM_CONF` reader"]
pub struct R(crate::R<MEM_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEM_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEM_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEM_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MEM_CONF` writer"]
pub struct W(crate::W<MEM_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEM_CONF_SPEC>;
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
impl From<crate::W<MEM_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEM_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MEM_FORCE_PD` reader - Set this bit to force power down UART memory."]
pub type MEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `MEM_FORCE_PD` writer - Set this bit to force power down UART memory."]
pub type MEM_FORCE_PD_W<'a, const O: u8> = crate::BitWriter<'a, MEM_CONF_SPEC, O>;
#[doc = "Field `MEM_FORCE_PU` reader - Set this bit to force power up UART memory."]
pub type MEM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `MEM_FORCE_PU` writer - Set this bit to force power up UART memory."]
pub type MEM_FORCE_PU_W<'a, const O: u8> = crate::BitWriter<'a, MEM_CONF_SPEC, O>;
impl R {
    #[doc = "Bit 25 - Set this bit to force power down UART memory."]
    #[inline(always)]
    pub fn mem_force_pd(&self) -> MEM_FORCE_PD_R {
        MEM_FORCE_PD_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Set this bit to force power up UART memory."]
    #[inline(always)]
    pub fn mem_force_pu(&self) -> MEM_FORCE_PU_R {
        MEM_FORCE_PU_R::new(((self.bits >> 26) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_CONF")
            .field(
                "mem_force_pd",
                &format_args!("{}", self.mem_force_pd().bit()),
            )
            .field(
                "mem_force_pu",
                &format_args!("{}", self.mem_force_pu().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MEM_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 25 - Set this bit to force power down UART memory."]
    #[inline(always)]
    #[must_use]
    pub fn mem_force_pd(&mut self) -> MEM_FORCE_PD_W<25> {
        MEM_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 26 - Set this bit to force power up UART memory."]
    #[inline(always)]
    #[must_use]
    pub fn mem_force_pu(&mut self) -> MEM_FORCE_PU_W<26> {
        MEM_FORCE_PU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART memory power configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_conf](index.html) module"]
pub struct MEM_CONF_SPEC;
impl crate::RegisterSpec for MEM_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mem_conf::R](R) reader structure"]
impl crate::Readable for MEM_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mem_conf::W](W) writer structure"]
impl crate::Writable for MEM_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MEM_CONF to value 0"]
impl crate::Resettable for MEM_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
