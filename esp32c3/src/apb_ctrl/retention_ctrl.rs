#[doc = "Register `RETENTION_CTRL` reader"]
pub struct R(crate::R<RETENTION_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RETENTION_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RETENTION_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RETENTION_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RETENTION_CTRL` writer"]
pub struct W(crate::W<RETENTION_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RETENTION_CTRL_SPEC>;
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
impl From<crate::W<RETENTION_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RETENTION_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RETENTION_LINK_ADDR` reader - reg_retention_link_addr"]
pub type RETENTION_LINK_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `RETENTION_LINK_ADDR` writer - reg_retention_link_addr"]
pub type RETENTION_LINK_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, RETENTION_CTRL_SPEC, 27, O, u32>;
#[doc = "Field `NOBYPASS_CPU_ISO_RST` reader - reg_nobypass_cpu_iso_rst"]
pub type NOBYPASS_CPU_ISO_RST_R = crate::BitReader;
#[doc = "Field `NOBYPASS_CPU_ISO_RST` writer - reg_nobypass_cpu_iso_rst"]
pub type NOBYPASS_CPU_ISO_RST_W<'a, const O: u8> = crate::BitWriter<'a, RETENTION_CTRL_SPEC, O>;
impl R {
    #[doc = "Bits 0:26 - reg_retention_link_addr"]
    #[inline(always)]
    pub fn retention_link_addr(&self) -> RETENTION_LINK_ADDR_R {
        RETENTION_LINK_ADDR_R::new(self.bits & 0x07ff_ffff)
    }
    #[doc = "Bit 27 - reg_nobypass_cpu_iso_rst"]
    #[inline(always)]
    pub fn nobypass_cpu_iso_rst(&self) -> NOBYPASS_CPU_ISO_RST_R {
        NOBYPASS_CPU_ISO_RST_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RETENTION_CTRL")
            .field(
                "retention_link_addr",
                &format_args!("{}", self.retention_link_addr().bits()),
            )
            .field(
                "nobypass_cpu_iso_rst",
                &format_args!("{}", self.nobypass_cpu_iso_rst().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RETENTION_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:26 - reg_retention_link_addr"]
    #[inline(always)]
    #[must_use]
    pub fn retention_link_addr(&mut self) -> RETENTION_LINK_ADDR_W<0> {
        RETENTION_LINK_ADDR_W::new(self)
    }
    #[doc = "Bit 27 - reg_nobypass_cpu_iso_rst"]
    #[inline(always)]
    #[must_use]
    pub fn nobypass_cpu_iso_rst(&mut self) -> NOBYPASS_CPU_ISO_RST_W<27> {
        NOBYPASS_CPU_ISO_RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB_CTRL_RETENTION_CTRL_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [retention_ctrl](index.html) module"]
pub struct RETENTION_CTRL_SPEC;
impl crate::RegisterSpec for RETENTION_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [retention_ctrl::R](R) reader structure"]
impl crate::Readable for RETENTION_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [retention_ctrl::W](W) writer structure"]
impl crate::Writable for RETENTION_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RETENTION_CTRL to value 0"]
impl crate::Resettable for RETENTION_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
