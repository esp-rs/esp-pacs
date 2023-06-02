#[doc = "Register `MEM_CTRL` reader"]
pub struct R(crate::R<MEM_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEM_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEM_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEM_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MEM_CTRL` writer"]
pub struct W(crate::W<MEM_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEM_CTRL_SPEC>;
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
impl From<crate::W<MEM_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEM_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UART_WAKEUP_FLAG_CLR` writer - need_des"]
pub type UART_WAKEUP_FLAG_CLR_W<'a, const O: u8> = crate::BitWriter<'a, MEM_CTRL_SPEC, O>;
#[doc = "Field `UART_WAKEUP_FLAG` reader - need_des"]
pub type UART_WAKEUP_FLAG_R = crate::BitReader;
#[doc = "Field `UART_WAKEUP_FLAG` writer - need_des"]
pub type UART_WAKEUP_FLAG_W<'a, const O: u8> = crate::BitWriter<'a, MEM_CTRL_SPEC, O>;
#[doc = "Field `UART_WAKEUP_EN` reader - need_des"]
pub type UART_WAKEUP_EN_R = crate::BitReader;
#[doc = "Field `UART_WAKEUP_EN` writer - need_des"]
pub type UART_WAKEUP_EN_W<'a, const O: u8> = crate::BitWriter<'a, MEM_CTRL_SPEC, O>;
#[doc = "Field `UART_MEM_FORCE_PD` reader - need_des"]
pub type UART_MEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `UART_MEM_FORCE_PD` writer - need_des"]
pub type UART_MEM_FORCE_PD_W<'a, const O: u8> = crate::BitWriter<'a, MEM_CTRL_SPEC, O>;
#[doc = "Field `UART_MEM_FORCE_PU` reader - need_des"]
pub type UART_MEM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `UART_MEM_FORCE_PU` writer - need_des"]
pub type UART_MEM_FORCE_PU_W<'a, const O: u8> = crate::BitWriter<'a, MEM_CTRL_SPEC, O>;
impl R {
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn uart_wakeup_flag(&self) -> UART_WAKEUP_FLAG_R {
        UART_WAKEUP_FLAG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn uart_wakeup_en(&self) -> UART_WAKEUP_EN_R {
        UART_WAKEUP_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn uart_mem_force_pd(&self) -> UART_MEM_FORCE_PD_R {
        UART_MEM_FORCE_PD_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn uart_mem_force_pu(&self) -> UART_MEM_FORCE_PU_R {
        UART_MEM_FORCE_PU_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_CTRL")
            .field(
                "uart_wakeup_flag",
                &format_args!("{}", self.uart_wakeup_flag().bit()),
            )
            .field(
                "uart_wakeup_en",
                &format_args!("{}", self.uart_wakeup_en().bit()),
            )
            .field(
                "uart_mem_force_pd",
                &format_args!("{}", self.uart_mem_force_pd().bit()),
            )
            .field(
                "uart_mem_force_pu",
                &format_args!("{}", self.uart_mem_force_pu().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MEM_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn uart_wakeup_flag_clr(&mut self) -> UART_WAKEUP_FLAG_CLR_W<0> {
        UART_WAKEUP_FLAG_CLR_W::new(self)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn uart_wakeup_flag(&mut self) -> UART_WAKEUP_FLAG_W<1> {
        UART_WAKEUP_FLAG_W::new(self)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn uart_wakeup_en(&mut self) -> UART_WAKEUP_EN_W<29> {
        UART_WAKEUP_EN_W::new(self)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn uart_mem_force_pd(&mut self) -> UART_MEM_FORCE_PD_W<30> {
        UART_MEM_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn uart_mem_force_pu(&mut self) -> UART_MEM_FORCE_PU_W<31> {
        UART_MEM_FORCE_PU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_ctrl](index.html) module"]
pub struct MEM_CTRL_SPEC;
impl crate::RegisterSpec for MEM_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mem_ctrl::R](R) reader structure"]
impl crate::Readable for MEM_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mem_ctrl::W](W) writer structure"]
impl crate::Writable for MEM_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MEM_CTRL to value 0x8000_0000"]
impl crate::Resettable for MEM_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_0000;
}
