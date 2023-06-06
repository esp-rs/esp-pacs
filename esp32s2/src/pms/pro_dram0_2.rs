#[doc = "Register `PRO_DRAM0_2` reader"]
pub struct R(crate::R<PRO_DRAM0_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_DRAM0_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_DRAM0_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_DRAM0_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_DRAM0_2` writer"]
pub struct W(crate::W<PRO_DRAM0_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_DRAM0_2_SPEC>;
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
impl From<crate::W<PRO_DRAM0_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_DRAM0_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_DRAM0_RTCFAST_SPLTADDR` reader - Configure the split address of RTC FAST for DBUS0 access."]
pub type PRO_DRAM0_RTCFAST_SPLTADDR_R = crate::FieldReader<u16>;
#[doc = "Field `PRO_DRAM0_RTCFAST_SPLTADDR` writer - Configure the split address of RTC FAST for DBUS0 access."]
pub type PRO_DRAM0_RTCFAST_SPLTADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, PRO_DRAM0_2_SPEC, 11, O, u16>;
#[doc = "Field `PRO_DRAM0_RTCFAST_L_R` reader - Setting to 1 grants DBUS0 permission to read RTC FAST low address region."]
pub type PRO_DRAM0_RTCFAST_L_R_R = crate::BitReader;
#[doc = "Field `PRO_DRAM0_RTCFAST_L_R` writer - Setting to 1 grants DBUS0 permission to read RTC FAST low address region."]
pub type PRO_DRAM0_RTCFAST_L_R_W<'a, const O: u8> = crate::BitWriter<'a, PRO_DRAM0_2_SPEC, O>;
#[doc = "Field `PRO_DRAM0_RTCFAST_L_W` reader - Setting to 1 grants DBUS0 permission to write RTC FAST low address region."]
pub type PRO_DRAM0_RTCFAST_L_W_R = crate::BitReader;
#[doc = "Field `PRO_DRAM0_RTCFAST_L_W` writer - Setting to 1 grants DBUS0 permission to write RTC FAST low address region."]
pub type PRO_DRAM0_RTCFAST_L_W_W<'a, const O: u8> = crate::BitWriter<'a, PRO_DRAM0_2_SPEC, O>;
#[doc = "Field `PRO_DRAM0_RTCFAST_H_R` reader - Setting to 1 grants DBUS0 permission to read RTC FAST high address region."]
pub type PRO_DRAM0_RTCFAST_H_R_R = crate::BitReader;
#[doc = "Field `PRO_DRAM0_RTCFAST_H_R` writer - Setting to 1 grants DBUS0 permission to read RTC FAST high address region."]
pub type PRO_DRAM0_RTCFAST_H_R_W<'a, const O: u8> = crate::BitWriter<'a, PRO_DRAM0_2_SPEC, O>;
#[doc = "Field `PRO_DRAM0_RTCFAST_H_W` reader - Setting to 1 grants DBUS0 permission to write RTC FAST high address region."]
pub type PRO_DRAM0_RTCFAST_H_W_R = crate::BitReader;
#[doc = "Field `PRO_DRAM0_RTCFAST_H_W` writer - Setting to 1 grants DBUS0 permission to write RTC FAST high address region."]
pub type PRO_DRAM0_RTCFAST_H_W_W<'a, const O: u8> = crate::BitWriter<'a, PRO_DRAM0_2_SPEC, O>;
impl R {
    #[doc = "Bits 0:10 - Configure the split address of RTC FAST for DBUS0 access."]
    #[inline(always)]
    pub fn pro_dram0_rtcfast_spltaddr(&self) -> PRO_DRAM0_RTCFAST_SPLTADDR_R {
        PRO_DRAM0_RTCFAST_SPLTADDR_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 11 - Setting to 1 grants DBUS0 permission to read RTC FAST low address region."]
    #[inline(always)]
    pub fn pro_dram0_rtcfast_l_r(&self) -> PRO_DRAM0_RTCFAST_L_R_R {
        PRO_DRAM0_RTCFAST_L_R_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Setting to 1 grants DBUS0 permission to write RTC FAST low address region."]
    #[inline(always)]
    pub fn pro_dram0_rtcfast_l_w(&self) -> PRO_DRAM0_RTCFAST_L_W_R {
        PRO_DRAM0_RTCFAST_L_W_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Setting to 1 grants DBUS0 permission to read RTC FAST high address region."]
    #[inline(always)]
    pub fn pro_dram0_rtcfast_h_r(&self) -> PRO_DRAM0_RTCFAST_H_R_R {
        PRO_DRAM0_RTCFAST_H_R_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Setting to 1 grants DBUS0 permission to write RTC FAST high address region."]
    #[inline(always)]
    pub fn pro_dram0_rtcfast_h_w(&self) -> PRO_DRAM0_RTCFAST_H_W_R {
        PRO_DRAM0_RTCFAST_H_W_R::new(((self.bits >> 14) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_DRAM0_2")
            .field(
                "pro_dram0_rtcfast_spltaddr",
                &format_args!("{}", self.pro_dram0_rtcfast_spltaddr().bits()),
            )
            .field(
                "pro_dram0_rtcfast_l_r",
                &format_args!("{}", self.pro_dram0_rtcfast_l_r().bit()),
            )
            .field(
                "pro_dram0_rtcfast_l_w",
                &format_args!("{}", self.pro_dram0_rtcfast_l_w().bit()),
            )
            .field(
                "pro_dram0_rtcfast_h_r",
                &format_args!("{}", self.pro_dram0_rtcfast_h_r().bit()),
            )
            .field(
                "pro_dram0_rtcfast_h_w",
                &format_args!("{}", self.pro_dram0_rtcfast_h_w().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_DRAM0_2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:10 - Configure the split address of RTC FAST for DBUS0 access."]
    #[inline(always)]
    #[must_use]
    pub fn pro_dram0_rtcfast_spltaddr(&mut self) -> PRO_DRAM0_RTCFAST_SPLTADDR_W<0> {
        PRO_DRAM0_RTCFAST_SPLTADDR_W::new(self)
    }
    #[doc = "Bit 11 - Setting to 1 grants DBUS0 permission to read RTC FAST low address region."]
    #[inline(always)]
    #[must_use]
    pub fn pro_dram0_rtcfast_l_r(&mut self) -> PRO_DRAM0_RTCFAST_L_R_W<11> {
        PRO_DRAM0_RTCFAST_L_R_W::new(self)
    }
    #[doc = "Bit 12 - Setting to 1 grants DBUS0 permission to write RTC FAST low address region."]
    #[inline(always)]
    #[must_use]
    pub fn pro_dram0_rtcfast_l_w(&mut self) -> PRO_DRAM0_RTCFAST_L_W_W<12> {
        PRO_DRAM0_RTCFAST_L_W_W::new(self)
    }
    #[doc = "Bit 13 - Setting to 1 grants DBUS0 permission to read RTC FAST high address region."]
    #[inline(always)]
    #[must_use]
    pub fn pro_dram0_rtcfast_h_r(&mut self) -> PRO_DRAM0_RTCFAST_H_R_W<13> {
        PRO_DRAM0_RTCFAST_H_R_W::new(self)
    }
    #[doc = "Bit 14 - Setting to 1 grants DBUS0 permission to write RTC FAST high address region."]
    #[inline(always)]
    #[must_use]
    pub fn pro_dram0_rtcfast_h_w(&mut self) -> PRO_DRAM0_RTCFAST_H_W_W<14> {
        PRO_DRAM0_RTCFAST_H_W_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DBUS permission control register 2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_dram0_2](index.html) module"]
pub struct PRO_DRAM0_2_SPEC;
impl crate::RegisterSpec for PRO_DRAM0_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_dram0_2::R](R) reader structure"]
impl crate::Readable for PRO_DRAM0_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_dram0_2::W](W) writer structure"]
impl crate::Writable for PRO_DRAM0_2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRO_DRAM0_2 to value 0x7800"]
impl crate::Resettable for PRO_DRAM0_2_SPEC {
    const RESET_VALUE: Self::Ux = 0x7800;
}
