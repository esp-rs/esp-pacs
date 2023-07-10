#[doc = "Register `XTAL32K` reader"]
pub struct R(crate::R<XTAL32K_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XTAL32K_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XTAL32K_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XTAL32K_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XTAL32K` writer"]
pub struct W(crate::W<XTAL32K_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XTAL32K_SPEC>;
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
impl From<crate::W<XTAL32K_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XTAL32K_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DRES_XTAL32K` reader - need_des"]
pub type DRES_XTAL32K_R = crate::FieldReader;
#[doc = "Field `DRES_XTAL32K` writer - need_des"]
pub type DRES_XTAL32K_W<'a, const O: u8> = crate::FieldWriter<'a, XTAL32K_SPEC, 3, O>;
#[doc = "Field `DGM_XTAL32K` reader - need_des"]
pub type DGM_XTAL32K_R = crate::FieldReader;
#[doc = "Field `DGM_XTAL32K` writer - need_des"]
pub type DGM_XTAL32K_W<'a, const O: u8> = crate::FieldWriter<'a, XTAL32K_SPEC, 3, O>;
#[doc = "Field `DBUF_XTAL32K` reader - need_des"]
pub type DBUF_XTAL32K_R = crate::BitReader;
#[doc = "Field `DBUF_XTAL32K` writer - need_des"]
pub type DBUF_XTAL32K_W<'a, const O: u8> = crate::BitWriter<'a, XTAL32K_SPEC, O>;
#[doc = "Field `DAC_XTAL32K` reader - need_des"]
pub type DAC_XTAL32K_R = crate::FieldReader;
#[doc = "Field `DAC_XTAL32K` writer - need_des"]
pub type DAC_XTAL32K_W<'a, const O: u8> = crate::FieldWriter<'a, XTAL32K_SPEC, 3, O>;
impl R {
    #[doc = "Bits 22:24 - need_des"]
    #[inline(always)]
    pub fn dres_xtal32k(&self) -> DRES_XTAL32K_R {
        DRES_XTAL32K_R::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bits 25:27 - need_des"]
    #[inline(always)]
    pub fn dgm_xtal32k(&self) -> DGM_XTAL32K_R {
        DGM_XTAL32K_R::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn dbuf_xtal32k(&self) -> DBUF_XTAL32K_R {
        DBUF_XTAL32K_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:31 - need_des"]
    #[inline(always)]
    pub fn dac_xtal32k(&self) -> DAC_XTAL32K_R {
        DAC_XTAL32K_R::new(((self.bits >> 29) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XTAL32K")
            .field(
                "dres_xtal32k",
                &format_args!("{}", self.dres_xtal32k().bits()),
            )
            .field(
                "dgm_xtal32k",
                &format_args!("{}", self.dgm_xtal32k().bits()),
            )
            .field(
                "dbuf_xtal32k",
                &format_args!("{}", self.dbuf_xtal32k().bit()),
            )
            .field(
                "dac_xtal32k",
                &format_args!("{}", self.dac_xtal32k().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<XTAL32K_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 22:24 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn dres_xtal32k(&mut self) -> DRES_XTAL32K_W<22> {
        DRES_XTAL32K_W::new(self)
    }
    #[doc = "Bits 25:27 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn dgm_xtal32k(&mut self) -> DGM_XTAL32K_W<25> {
        DGM_XTAL32K_W::new(self)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn dbuf_xtal32k(&mut self) -> DBUF_XTAL32K_W<28> {
        DBUF_XTAL32K_W::new(self)
    }
    #[doc = "Bits 29:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn dac_xtal32k(&mut self) -> DAC_XTAL32K_W<29> {
        DAC_XTAL32K_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xtal32k](index.html) module"]
pub struct XTAL32K_SPEC;
impl crate::RegisterSpec for XTAL32K_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xtal32k::R](R) reader structure"]
impl crate::Readable for XTAL32K_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xtal32k::W](W) writer structure"]
impl crate::Writable for XTAL32K_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets XTAL32K to value 0x66c0_0000"]
impl crate::Resettable for XTAL32K_SPEC {
    const RESET_VALUE: Self::Ux = 0x66c0_0000;
}
