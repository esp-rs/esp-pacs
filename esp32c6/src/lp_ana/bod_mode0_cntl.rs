#[doc = "Register `BOD_MODE0_CNTL` reader"]
pub struct R(crate::R<BOD_MODE0_CNTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BOD_MODE0_CNTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BOD_MODE0_CNTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BOD_MODE0_CNTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BOD_MODE0_CNTL` writer"]
pub struct W(crate::W<BOD_MODE0_CNTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BOD_MODE0_CNTL_SPEC>;
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
impl From<crate::W<BOD_MODE0_CNTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BOD_MODE0_CNTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BOD_MODE0_CLOSE_FLASH_ENA` reader - need_des"]
pub type BOD_MODE0_CLOSE_FLASH_ENA_R = crate::BitReader;
#[doc = "Field `BOD_MODE0_CLOSE_FLASH_ENA` writer - need_des"]
pub type BOD_MODE0_CLOSE_FLASH_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, BOD_MODE0_CNTL_SPEC, O>;
#[doc = "Field `BOD_MODE0_PD_RF_ENA` reader - need_des"]
pub type BOD_MODE0_PD_RF_ENA_R = crate::BitReader;
#[doc = "Field `BOD_MODE0_PD_RF_ENA` writer - need_des"]
pub type BOD_MODE0_PD_RF_ENA_W<'a, const O: u8> = crate::BitWriter<'a, BOD_MODE0_CNTL_SPEC, O>;
#[doc = "Field `BOD_MODE0_INTR_WAIT` reader - need_des"]
pub type BOD_MODE0_INTR_WAIT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BOD_MODE0_INTR_WAIT` writer - need_des"]
pub type BOD_MODE0_INTR_WAIT_W<'a, const O: u8> =
    crate::FieldWriter<'a, BOD_MODE0_CNTL_SPEC, 10, O, u16, u16>;
#[doc = "Field `BOD_MODE0_RESET_WAIT` reader - need_des"]
pub type BOD_MODE0_RESET_WAIT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BOD_MODE0_RESET_WAIT` writer - need_des"]
pub type BOD_MODE0_RESET_WAIT_W<'a, const O: u8> =
    crate::FieldWriter<'a, BOD_MODE0_CNTL_SPEC, 10, O, u16, u16>;
#[doc = "Field `BOD_MODE0_CNT_CLR` reader - need_des"]
pub type BOD_MODE0_CNT_CLR_R = crate::BitReader;
#[doc = "Field `BOD_MODE0_CNT_CLR` writer - need_des"]
pub type BOD_MODE0_CNT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, BOD_MODE0_CNTL_SPEC, O>;
#[doc = "Field `BOD_MODE0_INTR_ENA` reader - need_des"]
pub type BOD_MODE0_INTR_ENA_R = crate::BitReader;
#[doc = "Field `BOD_MODE0_INTR_ENA` writer - need_des"]
pub type BOD_MODE0_INTR_ENA_W<'a, const O: u8> = crate::BitWriter<'a, BOD_MODE0_CNTL_SPEC, O>;
#[doc = "Field `BOD_MODE0_RESET_SEL` reader - need_des"]
pub type BOD_MODE0_RESET_SEL_R = crate::BitReader;
#[doc = "Field `BOD_MODE0_RESET_SEL` writer - need_des"]
pub type BOD_MODE0_RESET_SEL_W<'a, const O: u8> = crate::BitWriter<'a, BOD_MODE0_CNTL_SPEC, O>;
#[doc = "Field `BOD_MODE0_RESET_ENA` reader - need_des"]
pub type BOD_MODE0_RESET_ENA_R = crate::BitReader;
#[doc = "Field `BOD_MODE0_RESET_ENA` writer - need_des"]
pub type BOD_MODE0_RESET_ENA_W<'a, const O: u8> = crate::BitWriter<'a, BOD_MODE0_CNTL_SPEC, O>;
impl R {
    #[doc = "Bit 6 - need_des"]
    #[inline(always)]
    pub fn bod_mode0_close_flash_ena(&self) -> BOD_MODE0_CLOSE_FLASH_ENA_R {
        BOD_MODE0_CLOSE_FLASH_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - need_des"]
    #[inline(always)]
    pub fn bod_mode0_pd_rf_ena(&self) -> BOD_MODE0_PD_RF_ENA_R {
        BOD_MODE0_PD_RF_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:17 - need_des"]
    #[inline(always)]
    pub fn bod_mode0_intr_wait(&self) -> BOD_MODE0_INTR_WAIT_R {
        BOD_MODE0_INTR_WAIT_R::new(((self.bits >> 8) & 0x03ff) as u16)
    }
    #[doc = "Bits 18:27 - need_des"]
    #[inline(always)]
    pub fn bod_mode0_reset_wait(&self) -> BOD_MODE0_RESET_WAIT_R {
        BOD_MODE0_RESET_WAIT_R::new(((self.bits >> 18) & 0x03ff) as u16)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn bod_mode0_cnt_clr(&self) -> BOD_MODE0_CNT_CLR_R {
        BOD_MODE0_CNT_CLR_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn bod_mode0_intr_ena(&self) -> BOD_MODE0_INTR_ENA_R {
        BOD_MODE0_INTR_ENA_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn bod_mode0_reset_sel(&self) -> BOD_MODE0_RESET_SEL_R {
        BOD_MODE0_RESET_SEL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn bod_mode0_reset_ena(&self) -> BOD_MODE0_RESET_ENA_R {
        BOD_MODE0_RESET_ENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BOD_MODE0_CNTL")
            .field(
                "bod_mode0_close_flash_ena",
                &format_args!("{}", self.bod_mode0_close_flash_ena().bit()),
            )
            .field(
                "bod_mode0_pd_rf_ena",
                &format_args!("{}", self.bod_mode0_pd_rf_ena().bit()),
            )
            .field(
                "bod_mode0_intr_wait",
                &format_args!("{}", self.bod_mode0_intr_wait().bits()),
            )
            .field(
                "bod_mode0_reset_wait",
                &format_args!("{}", self.bod_mode0_reset_wait().bits()),
            )
            .field(
                "bod_mode0_cnt_clr",
                &format_args!("{}", self.bod_mode0_cnt_clr().bit()),
            )
            .field(
                "bod_mode0_intr_ena",
                &format_args!("{}", self.bod_mode0_intr_ena().bit()),
            )
            .field(
                "bod_mode0_reset_sel",
                &format_args!("{}", self.bod_mode0_reset_sel().bit()),
            )
            .field(
                "bod_mode0_reset_ena",
                &format_args!("{}", self.bod_mode0_reset_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BOD_MODE0_CNTL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 6 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn bod_mode0_close_flash_ena(&mut self) -> BOD_MODE0_CLOSE_FLASH_ENA_W<6> {
        BOD_MODE0_CLOSE_FLASH_ENA_W::new(self)
    }
    #[doc = "Bit 7 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn bod_mode0_pd_rf_ena(&mut self) -> BOD_MODE0_PD_RF_ENA_W<7> {
        BOD_MODE0_PD_RF_ENA_W::new(self)
    }
    #[doc = "Bits 8:17 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn bod_mode0_intr_wait(&mut self) -> BOD_MODE0_INTR_WAIT_W<8> {
        BOD_MODE0_INTR_WAIT_W::new(self)
    }
    #[doc = "Bits 18:27 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn bod_mode0_reset_wait(&mut self) -> BOD_MODE0_RESET_WAIT_W<18> {
        BOD_MODE0_RESET_WAIT_W::new(self)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn bod_mode0_cnt_clr(&mut self) -> BOD_MODE0_CNT_CLR_W<28> {
        BOD_MODE0_CNT_CLR_W::new(self)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn bod_mode0_intr_ena(&mut self) -> BOD_MODE0_INTR_ENA_W<29> {
        BOD_MODE0_INTR_ENA_W::new(self)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn bod_mode0_reset_sel(&mut self) -> BOD_MODE0_RESET_SEL_W<30> {
        BOD_MODE0_RESET_SEL_W::new(self)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn bod_mode0_reset_ena(&mut self) -> BOD_MODE0_RESET_ENA_W<31> {
        BOD_MODE0_RESET_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bod_mode0_cntl](index.html) module"]
pub struct BOD_MODE0_CNTL_SPEC;
impl crate::RegisterSpec for BOD_MODE0_CNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bod_mode0_cntl::R](R) reader structure"]
impl crate::Readable for BOD_MODE0_CNTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bod_mode0_cntl::W](W) writer structure"]
impl crate::Writable for BOD_MODE0_CNTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BOD_MODE0_CNTL to value 0x0ffc_0100"]
impl crate::Resettable for BOD_MODE0_CNTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0ffc_0100;
}
