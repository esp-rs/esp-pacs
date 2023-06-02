#[doc = "Register `GUSBCFG` reader"]
pub struct R(crate::R<GUSBCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GUSBCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GUSBCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GUSBCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GUSBCFG` writer"]
pub struct W(crate::W<GUSBCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GUSBCFG_SPEC>;
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
impl From<crate::W<GUSBCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GUSBCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOUTCAL` reader - "]
pub type TOUTCAL_R = crate::FieldReader;
#[doc = "Field `TOUTCAL` writer - "]
pub type TOUTCAL_W<'a, const O: u8> = crate::FieldWriter<'a, GUSBCFG_SPEC, 3, O>;
#[doc = "Field `PHYIF` reader - "]
pub type PHYIF_R = crate::BitReader;
#[doc = "Field `PHYIF` writer - "]
pub type PHYIF_W<'a, const O: u8> = crate::BitWriter<'a, GUSBCFG_SPEC, O>;
#[doc = "Field `ULPI_UTMI_SEL` reader - "]
pub type ULPI_UTMI_SEL_R = crate::BitReader;
#[doc = "Field `FSINTF` reader - "]
pub type FSINTF_R = crate::BitReader;
#[doc = "Field `FSINTF` writer - "]
pub type FSINTF_W<'a, const O: u8> = crate::BitWriter<'a, GUSBCFG_SPEC, O>;
#[doc = "Field `PHYSEL` reader - "]
pub type PHYSEL_R = crate::BitReader;
#[doc = "Field `SRPCAP` reader - "]
pub type SRPCAP_R = crate::BitReader;
#[doc = "Field `SRPCAP` writer - "]
pub type SRPCAP_W<'a, const O: u8> = crate::BitWriter<'a, GUSBCFG_SPEC, O>;
#[doc = "Field `HNPCAP` reader - "]
pub type HNPCAP_R = crate::BitReader;
#[doc = "Field `HNPCAP` writer - "]
pub type HNPCAP_W<'a, const O: u8> = crate::BitWriter<'a, GUSBCFG_SPEC, O>;
#[doc = "Field `USBTRDTIM` reader - "]
pub type USBTRDTIM_R = crate::FieldReader;
#[doc = "Field `USBTRDTIM` writer - "]
pub type USBTRDTIM_W<'a, const O: u8> = crate::FieldWriter<'a, GUSBCFG_SPEC, 4, O>;
#[doc = "Field `TERMSELDLPULSE` reader - "]
pub type TERMSELDLPULSE_R = crate::BitReader;
#[doc = "Field `TERMSELDLPULSE` writer - "]
pub type TERMSELDLPULSE_W<'a, const O: u8> = crate::BitWriter<'a, GUSBCFG_SPEC, O>;
#[doc = "Field `TXENDDELAY` reader - "]
pub type TXENDDELAY_R = crate::BitReader;
#[doc = "Field `TXENDDELAY` writer - "]
pub type TXENDDELAY_W<'a, const O: u8> = crate::BitWriter<'a, GUSBCFG_SPEC, O>;
#[doc = "Field `FORCEHSTMODE` reader - "]
pub type FORCEHSTMODE_R = crate::BitReader;
#[doc = "Field `FORCEHSTMODE` writer - "]
pub type FORCEHSTMODE_W<'a, const O: u8> = crate::BitWriter<'a, GUSBCFG_SPEC, O>;
#[doc = "Field `FORCEDEVMODE` reader - "]
pub type FORCEDEVMODE_R = crate::BitReader;
#[doc = "Field `FORCEDEVMODE` writer - "]
pub type FORCEDEVMODE_W<'a, const O: u8> = crate::BitWriter<'a, GUSBCFG_SPEC, O>;
#[doc = "Field `CORRUPTTXPKT` reader - "]
pub type CORRUPTTXPKT_R = crate::BitReader;
#[doc = "Field `CORRUPTTXPKT` writer - "]
pub type CORRUPTTXPKT_W<'a, const O: u8> = crate::BitWriter<'a, GUSBCFG_SPEC, O>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn toutcal(&self) -> TOUTCAL_R {
        TOUTCAL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn phyif(&self) -> PHYIF_R {
        PHYIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ulpi_utmi_sel(&self) -> ULPI_UTMI_SEL_R {
        ULPI_UTMI_SEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn fsintf(&self) -> FSINTF_R {
        FSINTF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn physel(&self) -> PHYSEL_R {
        PHYSEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn srpcap(&self) -> SRPCAP_R {
        SRPCAP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn hnpcap(&self) -> HNPCAP_R {
        HNPCAP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:13"]
    #[inline(always)]
    pub fn usbtrdtim(&self) -> USBTRDTIM_R {
        USBTRDTIM_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn termseldlpulse(&self) -> TERMSELDLPULSE_R {
        TERMSELDLPULSE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn txenddelay(&self) -> TXENDDELAY_R {
        TXENDDELAY_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn forcehstmode(&self) -> FORCEHSTMODE_R {
        FORCEHSTMODE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn forcedevmode(&self) -> FORCEDEVMODE_R {
        FORCEDEVMODE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn corrupttxpkt(&self) -> CORRUPTTXPKT_R {
        CORRUPTTXPKT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GUSBCFG")
            .field("toutcal", &format_args!("{}", self.toutcal().bits()))
            .field("phyif", &format_args!("{}", self.phyif().bit()))
            .field(
                "ulpi_utmi_sel",
                &format_args!("{}", self.ulpi_utmi_sel().bit()),
            )
            .field("fsintf", &format_args!("{}", self.fsintf().bit()))
            .field("physel", &format_args!("{}", self.physel().bit()))
            .field("srpcap", &format_args!("{}", self.srpcap().bit()))
            .field("hnpcap", &format_args!("{}", self.hnpcap().bit()))
            .field("usbtrdtim", &format_args!("{}", self.usbtrdtim().bits()))
            .field(
                "termseldlpulse",
                &format_args!("{}", self.termseldlpulse().bit()),
            )
            .field("txenddelay", &format_args!("{}", self.txenddelay().bit()))
            .field(
                "forcehstmode",
                &format_args!("{}", self.forcehstmode().bit()),
            )
            .field(
                "forcedevmode",
                &format_args!("{}", self.forcedevmode().bit()),
            )
            .field(
                "corrupttxpkt",
                &format_args!("{}", self.corrupttxpkt().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GUSBCFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn toutcal(&mut self) -> TOUTCAL_W<0> {
        TOUTCAL_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn phyif(&mut self) -> PHYIF_W<3> {
        PHYIF_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn fsintf(&mut self) -> FSINTF_W<5> {
        FSINTF_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn srpcap(&mut self) -> SRPCAP_W<8> {
        SRPCAP_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn hnpcap(&mut self) -> HNPCAP_W<9> {
        HNPCAP_W::new(self)
    }
    #[doc = "Bits 10:13"]
    #[inline(always)]
    #[must_use]
    pub fn usbtrdtim(&mut self) -> USBTRDTIM_W<10> {
        USBTRDTIM_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn termseldlpulse(&mut self) -> TERMSELDLPULSE_W<22> {
        TERMSELDLPULSE_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn txenddelay(&mut self) -> TXENDDELAY_W<28> {
        TXENDDELAY_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn forcehstmode(&mut self) -> FORCEHSTMODE_W<29> {
        FORCEHSTMODE_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn forcedevmode(&mut self) -> FORCEDEVMODE_W<30> {
        FORCEDEVMODE_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn corrupttxpkt(&mut self) -> CORRUPTTXPKT_W<31> {
        CORRUPTTXPKT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gusbcfg](index.html) module"]
pub struct GUSBCFG_SPEC;
impl crate::RegisterSpec for GUSBCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gusbcfg::R](R) reader structure"]
impl crate::Readable for GUSBCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gusbcfg::W](W) writer structure"]
impl crate::Writable for GUSBCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GUSBCFG to value 0x1440"]
impl crate::Resettable for GUSBCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x1440;
}
