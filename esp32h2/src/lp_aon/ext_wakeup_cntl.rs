#[doc = "Register `EXT_WAKEUP_CNTL` reader"]
pub type R = crate::R<EXT_WAKEUP_CNTL_SPEC>;
#[doc = "Register `EXT_WAKEUP_CNTL` writer"]
pub type W = crate::W<EXT_WAKEUP_CNTL_SPEC>;
#[doc = "Field `EXT_WAKEUP_STATUS` reader - need_des"]
pub type EXT_WAKEUP_STATUS_R = crate::FieldReader;
#[doc = "Field `EXT_WAKEUP_STATUS_CLR` writer - need_des"]
pub type EXT_WAKEUP_STATUS_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXT_WAKEUP_SEL` reader - need_des"]
pub type EXT_WAKEUP_SEL_R = crate::FieldReader;
#[doc = "Field `EXT_WAKEUP_SEL` writer - need_des"]
pub type EXT_WAKEUP_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `EXT_WAKEUP_LV` reader - need_des"]
pub type EXT_WAKEUP_LV_R = crate::FieldReader;
#[doc = "Field `EXT_WAKEUP_LV` writer - need_des"]
pub type EXT_WAKEUP_LV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `EXT_WAKEUP_FILTER` reader - need_des"]
pub type EXT_WAKEUP_FILTER_R = crate::BitReader;
#[doc = "Field `EXT_WAKEUP_FILTER` writer - need_des"]
pub type EXT_WAKEUP_FILTER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    pub fn ext_wakeup_status(&self) -> EXT_WAKEUP_STATUS_R {
        EXT_WAKEUP_STATUS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 15:22 - need_des"]
    #[inline(always)]
    pub fn ext_wakeup_sel(&self) -> EXT_WAKEUP_SEL_R {
        EXT_WAKEUP_SEL_R::new(((self.bits >> 15) & 0xff) as u8)
    }
    #[doc = "Bits 23:30 - need_des"]
    #[inline(always)]
    pub fn ext_wakeup_lv(&self) -> EXT_WAKEUP_LV_R {
        EXT_WAKEUP_LV_R::new(((self.bits >> 23) & 0xff) as u8)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn ext_wakeup_filter(&self) -> EXT_WAKEUP_FILTER_R {
        EXT_WAKEUP_FILTER_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXT_WAKEUP_CNTL")
            .field(
                "ext_wakeup_status",
                &format_args!("{}", self.ext_wakeup_status().bits()),
            )
            .field(
                "ext_wakeup_sel",
                &format_args!("{}", self.ext_wakeup_sel().bits()),
            )
            .field(
                "ext_wakeup_lv",
                &format_args!("{}", self.ext_wakeup_lv().bits()),
            )
            .field(
                "ext_wakeup_filter",
                &format_args!("{}", self.ext_wakeup_filter().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EXT_WAKEUP_CNTL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 14 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn ext_wakeup_status_clr(&mut self) -> EXT_WAKEUP_STATUS_CLR_W<EXT_WAKEUP_CNTL_SPEC, 14> {
        EXT_WAKEUP_STATUS_CLR_W::new(self)
    }
    #[doc = "Bits 15:22 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn ext_wakeup_sel(&mut self) -> EXT_WAKEUP_SEL_W<EXT_WAKEUP_CNTL_SPEC, 15> {
        EXT_WAKEUP_SEL_W::new(self)
    }
    #[doc = "Bits 23:30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn ext_wakeup_lv(&mut self) -> EXT_WAKEUP_LV_W<EXT_WAKEUP_CNTL_SPEC, 23> {
        EXT_WAKEUP_LV_W::new(self)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn ext_wakeup_filter(&mut self) -> EXT_WAKEUP_FILTER_W<EXT_WAKEUP_CNTL_SPEC, 31> {
        EXT_WAKEUP_FILTER_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_wakeup_cntl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_wakeup_cntl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXT_WAKEUP_CNTL_SPEC;
impl crate::RegisterSpec for EXT_WAKEUP_CNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ext_wakeup_cntl::R`](R) reader structure"]
impl crate::Readable for EXT_WAKEUP_CNTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ext_wakeup_cntl::W`](W) writer structure"]
impl crate::Writable for EXT_WAKEUP_CNTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXT_WAKEUP_CNTL to value 0"]
impl crate::Resettable for EXT_WAKEUP_CNTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
