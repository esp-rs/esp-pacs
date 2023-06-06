#[doc = "Register `ULP_CP_TIMER` reader"]
pub struct R(crate::R<ULP_CP_TIMER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ULP_CP_TIMER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ULP_CP_TIMER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ULP_CP_TIMER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ULP_CP_TIMER` writer"]
pub struct W(crate::W<ULP_CP_TIMER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ULP_CP_TIMER_SPEC>;
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
impl From<crate::W<ULP_CP_TIMER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ULP_CP_TIMER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ULP_CP_PC_INIT` reader - ULP-coprocessor PC initial address"]
pub type ULP_CP_PC_INIT_R = crate::FieldReader<u16>;
#[doc = "Field `ULP_CP_PC_INIT` writer - ULP-coprocessor PC initial address"]
pub type ULP_CP_PC_INIT_W<'a, const O: u8> = crate::FieldWriter<'a, ULP_CP_TIMER_SPEC, 11, O, u16>;
#[doc = "Field `ULP_CP_GPIO_WAKEUP_ENA` reader - ULP-coprocessor wakeup by GPIO enable"]
pub type ULP_CP_GPIO_WAKEUP_ENA_R = crate::BitReader;
#[doc = "Field `ULP_CP_GPIO_WAKEUP_ENA` writer - ULP-coprocessor wakeup by GPIO enable"]
pub type ULP_CP_GPIO_WAKEUP_ENA_W<'a, const O: u8> = crate::BitWriter<'a, ULP_CP_TIMER_SPEC, O>;
#[doc = "Field `ULP_CP_GPIO_WAKEUP_CLR` writer - ULP-coprocessor wakeup by GPIO state clear"]
pub type ULP_CP_GPIO_WAKEUP_CLR_W<'a, const O: u8> = crate::BitWriter<'a, ULP_CP_TIMER_SPEC, O>;
#[doc = "Field `ULP_CP_SLP_TIMER_EN` reader - ULP-coprocessor timer enable bit"]
pub type ULP_CP_SLP_TIMER_EN_R = crate::BitReader;
#[doc = "Field `ULP_CP_SLP_TIMER_EN` writer - ULP-coprocessor timer enable bit"]
pub type ULP_CP_SLP_TIMER_EN_W<'a, const O: u8> = crate::BitWriter<'a, ULP_CP_TIMER_SPEC, O>;
impl R {
    #[doc = "Bits 0:10 - ULP-coprocessor PC initial address"]
    #[inline(always)]
    pub fn ulp_cp_pc_init(&self) -> ULP_CP_PC_INIT_R {
        ULP_CP_PC_INIT_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 29 - ULP-coprocessor wakeup by GPIO enable"]
    #[inline(always)]
    pub fn ulp_cp_gpio_wakeup_ena(&self) -> ULP_CP_GPIO_WAKEUP_ENA_R {
        ULP_CP_GPIO_WAKEUP_ENA_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - ULP-coprocessor timer enable bit"]
    #[inline(always)]
    pub fn ulp_cp_slp_timer_en(&self) -> ULP_CP_SLP_TIMER_EN_R {
        ULP_CP_SLP_TIMER_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ULP_CP_TIMER")
            .field(
                "ulp_cp_pc_init",
                &format_args!("{}", self.ulp_cp_pc_init().bits()),
            )
            .field(
                "ulp_cp_gpio_wakeup_ena",
                &format_args!("{}", self.ulp_cp_gpio_wakeup_ena().bit()),
            )
            .field(
                "ulp_cp_slp_timer_en",
                &format_args!("{}", self.ulp_cp_slp_timer_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ULP_CP_TIMER_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:10 - ULP-coprocessor PC initial address"]
    #[inline(always)]
    #[must_use]
    pub fn ulp_cp_pc_init(&mut self) -> ULP_CP_PC_INIT_W<0> {
        ULP_CP_PC_INIT_W::new(self)
    }
    #[doc = "Bit 29 - ULP-coprocessor wakeup by GPIO enable"]
    #[inline(always)]
    #[must_use]
    pub fn ulp_cp_gpio_wakeup_ena(&mut self) -> ULP_CP_GPIO_WAKEUP_ENA_W<29> {
        ULP_CP_GPIO_WAKEUP_ENA_W::new(self)
    }
    #[doc = "Bit 30 - ULP-coprocessor wakeup by GPIO state clear"]
    #[inline(always)]
    #[must_use]
    pub fn ulp_cp_gpio_wakeup_clr(&mut self) -> ULP_CP_GPIO_WAKEUP_CLR_W<30> {
        ULP_CP_GPIO_WAKEUP_CLR_W::new(self)
    }
    #[doc = "Bit 31 - ULP-coprocessor timer enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn ulp_cp_slp_timer_en(&mut self) -> ULP_CP_SLP_TIMER_EN_W<31> {
        ULP_CP_SLP_TIMER_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure ulp\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ulp_cp_timer](index.html) module"]
pub struct ULP_CP_TIMER_SPEC;
impl crate::RegisterSpec for ULP_CP_TIMER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ulp_cp_timer::R](R) reader structure"]
impl crate::Readable for ULP_CP_TIMER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ulp_cp_timer::W](W) writer structure"]
impl crate::Writable for ULP_CP_TIMER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ULP_CP_TIMER to value 0"]
impl crate::Resettable for ULP_CP_TIMER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
