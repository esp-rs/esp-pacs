#[doc = "Register `SDIO_SLAVE_LDO_CONF` reader"]
pub struct R(crate::R<SDIO_SLAVE_LDO_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDIO_SLAVE_LDO_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDIO_SLAVE_LDO_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDIO_SLAVE_LDO_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDIO_SLAVE_LDO_CONF` writer"]
pub struct W(crate::W<SDIO_SLAVE_LDO_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDIO_SLAVE_LDO_CONF_SPEC>;
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
impl From<crate::W<SDIO_SLAVE_LDO_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDIO_SLAVE_LDO_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LDO_READY_CTL_IN_EN` reader - control ldo ready signal by sdio slave itself"]
pub type LDO_READY_CTL_IN_EN_R = crate::BitReader;
#[doc = "Field `LDO_READY_CTL_IN_EN` writer - control ldo ready signal by sdio slave itself"]
pub type LDO_READY_CTL_IN_EN_W<'a, const O: u8> = crate::BitWriter<'a, SDIO_SLAVE_LDO_CONF_SPEC, O>;
#[doc = "Field `LDO_READY_THRES` reader - configure ldo ready counting threshold value, the actual counting target is 2^(ldo_ready_thres)-1"]
pub type LDO_READY_THRES_R = crate::FieldReader;
#[doc = "Field `LDO_READY_THRES` writer - configure ldo ready counting threshold value, the actual counting target is 2^(ldo_ready_thres)-1"]
pub type LDO_READY_THRES_W<'a, const O: u8> =
    crate::FieldWriter<'a, SDIO_SLAVE_LDO_CONF_SPEC, 5, O>;
#[doc = "Field `LDO_READY_IGNORE_EN` reader - ignore ldo ready signal"]
pub type LDO_READY_IGNORE_EN_R = crate::BitReader;
#[doc = "Field `LDO_READY_IGNORE_EN` writer - ignore ldo ready signal"]
pub type LDO_READY_IGNORE_EN_W<'a, const O: u8> = crate::BitWriter<'a, SDIO_SLAVE_LDO_CONF_SPEC, O>;
impl R {
    #[doc = "Bit 0 - control ldo ready signal by sdio slave itself"]
    #[inline(always)]
    pub fn ldo_ready_ctl_in_en(&self) -> LDO_READY_CTL_IN_EN_R {
        LDO_READY_CTL_IN_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - configure ldo ready counting threshold value, the actual counting target is 2^(ldo_ready_thres)-1"]
    #[inline(always)]
    pub fn ldo_ready_thres(&self) -> LDO_READY_THRES_R {
        LDO_READY_THRES_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bit 6 - ignore ldo ready signal"]
    #[inline(always)]
    pub fn ldo_ready_ignore_en(&self) -> LDO_READY_IGNORE_EN_R {
        LDO_READY_IGNORE_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDIO_SLAVE_LDO_CONF")
            .field(
                "ldo_ready_ctl_in_en",
                &format_args!("{}", self.ldo_ready_ctl_in_en().bit()),
            )
            .field(
                "ldo_ready_thres",
                &format_args!("{}", self.ldo_ready_thres().bits()),
            )
            .field(
                "ldo_ready_ignore_en",
                &format_args!("{}", self.ldo_ready_ignore_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SDIO_SLAVE_LDO_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - control ldo ready signal by sdio slave itself"]
    #[inline(always)]
    #[must_use]
    pub fn ldo_ready_ctl_in_en(&mut self) -> LDO_READY_CTL_IN_EN_W<0> {
        LDO_READY_CTL_IN_EN_W::new(self)
    }
    #[doc = "Bits 1:5 - configure ldo ready counting threshold value, the actual counting target is 2^(ldo_ready_thres)-1"]
    #[inline(always)]
    #[must_use]
    pub fn ldo_ready_thres(&mut self) -> LDO_READY_THRES_W<1> {
        LDO_READY_THRES_W::new(self)
    }
    #[doc = "Bit 6 - ignore ldo ready signal"]
    #[inline(always)]
    #[must_use]
    pub fn ldo_ready_ignore_en(&mut self) -> LDO_READY_IGNORE_EN_W<6> {
        LDO_READY_IGNORE_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sdio slave ldo control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdio_slave_ldo_conf](index.html) module"]
pub struct SDIO_SLAVE_LDO_CONF_SPEC;
impl crate::RegisterSpec for SDIO_SLAVE_LDO_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdio_slave_ldo_conf::R](R) reader structure"]
impl crate::Readable for SDIO_SLAVE_LDO_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdio_slave_ldo_conf::W](W) writer structure"]
impl crate::Writable for SDIO_SLAVE_LDO_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDIO_SLAVE_LDO_CONF to value 0x14"]
impl crate::Resettable for SDIO_SLAVE_LDO_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x14;
}
