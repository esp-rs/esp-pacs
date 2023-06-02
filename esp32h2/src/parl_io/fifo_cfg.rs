#[doc = "Register `FIFO_CFG` reader"]
pub struct R(crate::R<FIFO_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFO_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFO_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFO_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFO_CFG` writer"]
pub struct W(crate::W<FIFO_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFO_CFG_SPEC>;
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
impl From<crate::W<FIFO_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFO_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_FIFO_SRST` reader - Set this bit to reset async fifo in tx module."]
pub type TX_FIFO_SRST_R = crate::BitReader;
#[doc = "Field `TX_FIFO_SRST` writer - Set this bit to reset async fifo in tx module."]
pub type TX_FIFO_SRST_W<'a, const O: u8> = crate::BitWriter<'a, FIFO_CFG_SPEC, O>;
#[doc = "Field `RX_FIFO_SRST` reader - Set this bit to reset async fifo in rx module."]
pub type RX_FIFO_SRST_R = crate::BitReader;
#[doc = "Field `RX_FIFO_SRST` writer - Set this bit to reset async fifo in rx module."]
pub type RX_FIFO_SRST_W<'a, const O: u8> = crate::BitWriter<'a, FIFO_CFG_SPEC, O>;
impl R {
    #[doc = "Bit 30 - Set this bit to reset async fifo in tx module."]
    #[inline(always)]
    pub fn tx_fifo_srst(&self) -> TX_FIFO_SRST_R {
        TX_FIFO_SRST_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Set this bit to reset async fifo in rx module."]
    #[inline(always)]
    pub fn rx_fifo_srst(&self) -> RX_FIFO_SRST_R {
        RX_FIFO_SRST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFO_CFG")
            .field(
                "tx_fifo_srst",
                &format_args!("{}", self.tx_fifo_srst().bit()),
            )
            .field(
                "rx_fifo_srst",
                &format_args!("{}", self.rx_fifo_srst().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FIFO_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 30 - Set this bit to reset async fifo in tx module."]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_srst(&mut self) -> TX_FIFO_SRST_W<30> {
        TX_FIFO_SRST_W::new(self)
    }
    #[doc = "Bit 31 - Set this bit to reset async fifo in rx module."]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_srst(&mut self) -> RX_FIFO_SRST_W<31> {
        RX_FIFO_SRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Parallel IO FIFO configuration register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo_cfg](index.html) module"]
pub struct FIFO_CFG_SPEC;
impl crate::RegisterSpec for FIFO_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifo_cfg::R](R) reader structure"]
impl crate::Readable for FIFO_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifo_cfg::W](W) writer structure"]
impl crate::Writable for FIFO_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FIFO_CFG to value 0"]
impl crate::Resettable for FIFO_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
