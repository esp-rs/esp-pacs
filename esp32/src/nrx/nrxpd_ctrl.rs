#[doc = "Register `NRXPD_CTRL` reader"]
pub struct R(crate::R<NRXPD_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NRXPD_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NRXPD_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NRXPD_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NRXPD_CTRL` writer"]
pub struct W(crate::W<NRXPD_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NRXPD_CTRL_SPEC>;
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
impl From<crate::W<NRXPD_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NRXPD_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEMAP_FORCE_PD` reader - "]
pub type DEMAP_FORCE_PD_R = crate::BitReader;
#[doc = "Field `DEMAP_FORCE_PD` writer - "]
pub type DEMAP_FORCE_PD_W<'a, const O: u8> = crate::BitWriter<'a, NRXPD_CTRL_SPEC, O>;
#[doc = "Field `DEMAP_FORCE_PU` reader - "]
pub type DEMAP_FORCE_PU_R = crate::BitReader;
#[doc = "Field `DEMAP_FORCE_PU` writer - "]
pub type DEMAP_FORCE_PU_W<'a, const O: u8> = crate::BitWriter<'a, NRXPD_CTRL_SPEC, O>;
#[doc = "Field `VIT_FORCE_PD` reader - "]
pub type VIT_FORCE_PD_R = crate::BitReader;
#[doc = "Field `VIT_FORCE_PD` writer - "]
pub type VIT_FORCE_PD_W<'a, const O: u8> = crate::BitWriter<'a, NRXPD_CTRL_SPEC, O>;
#[doc = "Field `VIT_FORCE_PU` reader - "]
pub type VIT_FORCE_PU_R = crate::BitReader;
#[doc = "Field `VIT_FORCE_PU` writer - "]
pub type VIT_FORCE_PU_W<'a, const O: u8> = crate::BitWriter<'a, NRXPD_CTRL_SPEC, O>;
#[doc = "Field `RX_ROT_FORCE_PD` reader - "]
pub type RX_ROT_FORCE_PD_R = crate::BitReader;
#[doc = "Field `RX_ROT_FORCE_PD` writer - "]
pub type RX_ROT_FORCE_PD_W<'a, const O: u8> = crate::BitWriter<'a, NRXPD_CTRL_SPEC, O>;
#[doc = "Field `RX_ROT_FORCE_PU` reader - "]
pub type RX_ROT_FORCE_PU_R = crate::BitReader;
#[doc = "Field `RX_ROT_FORCE_PU` writer - "]
pub type RX_ROT_FORCE_PU_W<'a, const O: u8> = crate::BitWriter<'a, NRXPD_CTRL_SPEC, O>;
#[doc = "Field `CHAN_EST_FORCE_PD` reader - "]
pub type CHAN_EST_FORCE_PD_R = crate::BitReader;
#[doc = "Field `CHAN_EST_FORCE_PD` writer - "]
pub type CHAN_EST_FORCE_PD_W<'a, const O: u8> = crate::BitWriter<'a, NRXPD_CTRL_SPEC, O>;
#[doc = "Field `CHAN_EST_FORCE_PU` reader - "]
pub type CHAN_EST_FORCE_PU_R = crate::BitReader;
#[doc = "Field `CHAN_EST_FORCE_PU` writer - "]
pub type CHAN_EST_FORCE_PU_W<'a, const O: u8> = crate::BitWriter<'a, NRXPD_CTRL_SPEC, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn demap_force_pd(&self) -> DEMAP_FORCE_PD_R {
        DEMAP_FORCE_PD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn demap_force_pu(&self) -> DEMAP_FORCE_PU_R {
        DEMAP_FORCE_PU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn vit_force_pd(&self) -> VIT_FORCE_PD_R {
        VIT_FORCE_PD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn vit_force_pu(&self) -> VIT_FORCE_PU_R {
        VIT_FORCE_PU_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rx_rot_force_pd(&self) -> RX_ROT_FORCE_PD_R {
        RX_ROT_FORCE_PD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rx_rot_force_pu(&self) -> RX_ROT_FORCE_PU_R {
        RX_ROT_FORCE_PU_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn chan_est_force_pd(&self) -> CHAN_EST_FORCE_PD_R {
        CHAN_EST_FORCE_PD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn chan_est_force_pu(&self) -> CHAN_EST_FORCE_PU_R {
        CHAN_EST_FORCE_PU_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NRXPD_CTRL")
            .field(
                "demap_force_pd",
                &format_args!("{}", self.demap_force_pd().bit()),
            )
            .field(
                "demap_force_pu",
                &format_args!("{}", self.demap_force_pu().bit()),
            )
            .field(
                "vit_force_pd",
                &format_args!("{}", self.vit_force_pd().bit()),
            )
            .field(
                "vit_force_pu",
                &format_args!("{}", self.vit_force_pu().bit()),
            )
            .field(
                "rx_rot_force_pd",
                &format_args!("{}", self.rx_rot_force_pd().bit()),
            )
            .field(
                "rx_rot_force_pu",
                &format_args!("{}", self.rx_rot_force_pu().bit()),
            )
            .field(
                "chan_est_force_pd",
                &format_args!("{}", self.chan_est_force_pd().bit()),
            )
            .field(
                "chan_est_force_pu",
                &format_args!("{}", self.chan_est_force_pu().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<NRXPD_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn demap_force_pd(&mut self) -> DEMAP_FORCE_PD_W<0> {
        DEMAP_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn demap_force_pu(&mut self) -> DEMAP_FORCE_PU_W<1> {
        DEMAP_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn vit_force_pd(&mut self) -> VIT_FORCE_PD_W<2> {
        VIT_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn vit_force_pu(&mut self) -> VIT_FORCE_PU_W<3> {
        VIT_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn rx_rot_force_pd(&mut self) -> RX_ROT_FORCE_PD_W<4> {
        RX_ROT_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn rx_rot_force_pu(&mut self) -> RX_ROT_FORCE_PU_W<5> {
        RX_ROT_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn chan_est_force_pd(&mut self) -> CHAN_EST_FORCE_PD_W<6> {
        CHAN_EST_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn chan_est_force_pu(&mut self) -> CHAN_EST_FORCE_PU_W<7> {
        CHAN_EST_FORCE_PU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WiFi RX control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nrxpd_ctrl](index.html) module"]
pub struct NRXPD_CTRL_SPEC;
impl crate::RegisterSpec for NRXPD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nrxpd_ctrl::R](R) reader structure"]
impl crate::Readable for NRXPD_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nrxpd_ctrl::W](W) writer structure"]
impl crate::Writable for NRXPD_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NRXPD_CTRL to value 0"]
impl crate::Resettable for NRXPD_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
