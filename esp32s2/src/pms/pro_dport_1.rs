#[doc = "Register `PRO_DPORT_1` reader"]
pub struct R(crate::R<PRO_DPORT_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_DPORT_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_DPORT_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_DPORT_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_DPORT_1` writer"]
pub struct W(crate::W<PRO_DPORT_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_DPORT_1_SPEC>;
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
impl From<crate::W<PRO_DPORT_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_DPORT_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_DPORT_APB_PERIPHERAL_FORBID` reader - Setting to 1 denies PeriBus1 bus???s access to APB peripheral."]
pub type PRO_DPORT_APB_PERIPHERAL_FORBID_R = crate::BitReader;
#[doc = "Field `PRO_DPORT_APB_PERIPHERAL_FORBID` writer - Setting to 1 denies PeriBus1 bus???s access to APB peripheral."]
pub type PRO_DPORT_APB_PERIPHERAL_FORBID_W<'a, const O: u8> =
    crate::BitWriter<'a, PRO_DPORT_1_SPEC, O>;
#[doc = "Field `PRO_DPORT_RTCSLOW_SPLTADDR` reader - Configure the split address of RTC FAST for PeriBus1 access."]
pub type PRO_DPORT_RTCSLOW_SPLTADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PRO_DPORT_RTCSLOW_SPLTADDR` writer - Configure the split address of RTC FAST for PeriBus1 access."]
pub type PRO_DPORT_RTCSLOW_SPLTADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, PRO_DPORT_1_SPEC, 11, O, u16, u16>;
#[doc = "Field `PRO_DPORT_RTCSLOW_L_R` reader - Setting to 1 grants PeriBus1 permission to read RTC FAST low address region."]
pub type PRO_DPORT_RTCSLOW_L_R_R = crate::BitReader;
#[doc = "Field `PRO_DPORT_RTCSLOW_L_R` writer - Setting to 1 grants PeriBus1 permission to read RTC FAST low address region."]
pub type PRO_DPORT_RTCSLOW_L_R_W<'a, const O: u8> = crate::BitWriter<'a, PRO_DPORT_1_SPEC, O>;
#[doc = "Field `PRO_DPORT_RTCSLOW_L_W` reader - Setting to 1 grants PeriBus1 permission to write RTC FAST low address region."]
pub type PRO_DPORT_RTCSLOW_L_W_R = crate::BitReader;
#[doc = "Field `PRO_DPORT_RTCSLOW_L_W` writer - Setting to 1 grants PeriBus1 permission to write RTC FAST low address region."]
pub type PRO_DPORT_RTCSLOW_L_W_W<'a, const O: u8> = crate::BitWriter<'a, PRO_DPORT_1_SPEC, O>;
#[doc = "Field `PRO_DPORT_RTCSLOW_H_R` reader - Setting to 1 grants PeriBus1 permission to read RTC FAST high address region."]
pub type PRO_DPORT_RTCSLOW_H_R_R = crate::BitReader;
#[doc = "Field `PRO_DPORT_RTCSLOW_H_R` writer - Setting to 1 grants PeriBus1 permission to read RTC FAST high address region."]
pub type PRO_DPORT_RTCSLOW_H_R_W<'a, const O: u8> = crate::BitWriter<'a, PRO_DPORT_1_SPEC, O>;
#[doc = "Field `PRO_DPORT_RTCSLOW_H_W` reader - Setting to 1 grants PeriBus1 permission to write RTC FAST high address region."]
pub type PRO_DPORT_RTCSLOW_H_W_R = crate::BitReader;
#[doc = "Field `PRO_DPORT_RTCSLOW_H_W` writer - Setting to 1 grants PeriBus1 permission to write RTC FAST high address region."]
pub type PRO_DPORT_RTCSLOW_H_W_W<'a, const O: u8> = crate::BitWriter<'a, PRO_DPORT_1_SPEC, O>;
#[doc = "Field `PRO_DPORT_RESERVE_FIFO_VALID` reader - Configure whether to enable read protection for user-configured FIFO address."]
pub type PRO_DPORT_RESERVE_FIFO_VALID_R = crate::FieldReader;
#[doc = "Field `PRO_DPORT_RESERVE_FIFO_VALID` writer - Configure whether to enable read protection for user-configured FIFO address."]
pub type PRO_DPORT_RESERVE_FIFO_VALID_W<'a, const O: u8> =
    crate::FieldWriter<'a, PRO_DPORT_1_SPEC, 4, O>;
impl R {
    #[doc = "Bit 0 - Setting to 1 denies PeriBus1 bus???s access to APB peripheral."]
    #[inline(always)]
    pub fn pro_dport_apb_peripheral_forbid(&self) -> PRO_DPORT_APB_PERIPHERAL_FORBID_R {
        PRO_DPORT_APB_PERIPHERAL_FORBID_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:11 - Configure the split address of RTC FAST for PeriBus1 access."]
    #[inline(always)]
    pub fn pro_dport_rtcslow_spltaddr(&self) -> PRO_DPORT_RTCSLOW_SPLTADDR_R {
        PRO_DPORT_RTCSLOW_SPLTADDR_R::new(((self.bits >> 1) & 0x07ff) as u16)
    }
    #[doc = "Bit 12 - Setting to 1 grants PeriBus1 permission to read RTC FAST low address region."]
    #[inline(always)]
    pub fn pro_dport_rtcslow_l_r(&self) -> PRO_DPORT_RTCSLOW_L_R_R {
        PRO_DPORT_RTCSLOW_L_R_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Setting to 1 grants PeriBus1 permission to write RTC FAST low address region."]
    #[inline(always)]
    pub fn pro_dport_rtcslow_l_w(&self) -> PRO_DPORT_RTCSLOW_L_W_R {
        PRO_DPORT_RTCSLOW_L_W_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Setting to 1 grants PeriBus1 permission to read RTC FAST high address region."]
    #[inline(always)]
    pub fn pro_dport_rtcslow_h_r(&self) -> PRO_DPORT_RTCSLOW_H_R_R {
        PRO_DPORT_RTCSLOW_H_R_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Setting to 1 grants PeriBus1 permission to write RTC FAST high address region."]
    #[inline(always)]
    pub fn pro_dport_rtcslow_h_w(&self) -> PRO_DPORT_RTCSLOW_H_W_R {
        PRO_DPORT_RTCSLOW_H_W_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Configure whether to enable read protection for user-configured FIFO address."]
    #[inline(always)]
    pub fn pro_dport_reserve_fifo_valid(&self) -> PRO_DPORT_RESERVE_FIFO_VALID_R {
        PRO_DPORT_RESERVE_FIFO_VALID_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_DPORT_1")
            .field(
                "pro_dport_apb_peripheral_forbid",
                &format_args!("{}", self.pro_dport_apb_peripheral_forbid().bit()),
            )
            .field(
                "pro_dport_rtcslow_spltaddr",
                &format_args!("{}", self.pro_dport_rtcslow_spltaddr().bits()),
            )
            .field(
                "pro_dport_rtcslow_l_r",
                &format_args!("{}", self.pro_dport_rtcslow_l_r().bit()),
            )
            .field(
                "pro_dport_rtcslow_l_w",
                &format_args!("{}", self.pro_dport_rtcslow_l_w().bit()),
            )
            .field(
                "pro_dport_rtcslow_h_r",
                &format_args!("{}", self.pro_dport_rtcslow_h_r().bit()),
            )
            .field(
                "pro_dport_rtcslow_h_w",
                &format_args!("{}", self.pro_dport_rtcslow_h_w().bit()),
            )
            .field(
                "pro_dport_reserve_fifo_valid",
                &format_args!("{}", self.pro_dport_reserve_fifo_valid().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_DPORT_1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Setting to 1 denies PeriBus1 bus???s access to APB peripheral."]
    #[inline(always)]
    #[must_use]
    pub fn pro_dport_apb_peripheral_forbid(&mut self) -> PRO_DPORT_APB_PERIPHERAL_FORBID_W<0> {
        PRO_DPORT_APB_PERIPHERAL_FORBID_W::new(self)
    }
    #[doc = "Bits 1:11 - Configure the split address of RTC FAST for PeriBus1 access."]
    #[inline(always)]
    #[must_use]
    pub fn pro_dport_rtcslow_spltaddr(&mut self) -> PRO_DPORT_RTCSLOW_SPLTADDR_W<1> {
        PRO_DPORT_RTCSLOW_SPLTADDR_W::new(self)
    }
    #[doc = "Bit 12 - Setting to 1 grants PeriBus1 permission to read RTC FAST low address region."]
    #[inline(always)]
    #[must_use]
    pub fn pro_dport_rtcslow_l_r(&mut self) -> PRO_DPORT_RTCSLOW_L_R_W<12> {
        PRO_DPORT_RTCSLOW_L_R_W::new(self)
    }
    #[doc = "Bit 13 - Setting to 1 grants PeriBus1 permission to write RTC FAST low address region."]
    #[inline(always)]
    #[must_use]
    pub fn pro_dport_rtcslow_l_w(&mut self) -> PRO_DPORT_RTCSLOW_L_W_W<13> {
        PRO_DPORT_RTCSLOW_L_W_W::new(self)
    }
    #[doc = "Bit 14 - Setting to 1 grants PeriBus1 permission to read RTC FAST high address region."]
    #[inline(always)]
    #[must_use]
    pub fn pro_dport_rtcslow_h_r(&mut self) -> PRO_DPORT_RTCSLOW_H_R_W<14> {
        PRO_DPORT_RTCSLOW_H_R_W::new(self)
    }
    #[doc = "Bit 15 - Setting to 1 grants PeriBus1 permission to write RTC FAST high address region."]
    #[inline(always)]
    #[must_use]
    pub fn pro_dport_rtcslow_h_w(&mut self) -> PRO_DPORT_RTCSLOW_H_W_W<15> {
        PRO_DPORT_RTCSLOW_H_W_W::new(self)
    }
    #[doc = "Bits 16:19 - Configure whether to enable read protection for user-configured FIFO address."]
    #[inline(always)]
    #[must_use]
    pub fn pro_dport_reserve_fifo_valid(&mut self) -> PRO_DPORT_RESERVE_FIFO_VALID_W<16> {
        PRO_DPORT_RESERVE_FIFO_VALID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PeriBus1 permission control register 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_dport_1](index.html) module"]
pub struct PRO_DPORT_1_SPEC;
impl crate::RegisterSpec for PRO_DPORT_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_dport_1::R](R) reader structure"]
impl crate::Readable for PRO_DPORT_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_dport_1::W](W) writer structure"]
impl crate::Writable for PRO_DPORT_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRO_DPORT_1 to value 0xf000"]
impl crate::Resettable for PRO_DPORT_1_SPEC {
    const RESET_VALUE: Self::Ux = 0xf000;
}
