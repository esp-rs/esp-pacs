#[doc = "Register `UPDATE` reader"]
pub struct R(crate::R<UPDATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UPDATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UPDATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UPDATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UPDATE` writer"]
pub struct W(crate::W<UPDATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UPDATE_SPEC>;
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
impl From<crate::W<UPDATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UPDATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAIN_TIMER_UPDATE` writer - need_des"]
pub type MAIN_TIMER_UPDATE_W<'a, const O: u8> = crate::BitWriter<'a, UPDATE_SPEC, O>;
#[doc = "Field `MAIN_TIMER_XTAL_OFF` reader - need_des"]
pub type MAIN_TIMER_XTAL_OFF_R = crate::BitReader;
#[doc = "Field `MAIN_TIMER_XTAL_OFF` writer - need_des"]
pub type MAIN_TIMER_XTAL_OFF_W<'a, const O: u8> = crate::BitWriter<'a, UPDATE_SPEC, O>;
#[doc = "Field `MAIN_TIMER_SYS_STALL` reader - need_des"]
pub type MAIN_TIMER_SYS_STALL_R = crate::BitReader;
#[doc = "Field `MAIN_TIMER_SYS_STALL` writer - need_des"]
pub type MAIN_TIMER_SYS_STALL_W<'a, const O: u8> = crate::BitWriter<'a, UPDATE_SPEC, O>;
#[doc = "Field `MAIN_TIMER_SYS_RST` reader - need_des"]
pub type MAIN_TIMER_SYS_RST_R = crate::BitReader;
#[doc = "Field `MAIN_TIMER_SYS_RST` writer - need_des"]
pub type MAIN_TIMER_SYS_RST_W<'a, const O: u8> = crate::BitWriter<'a, UPDATE_SPEC, O>;
impl R {
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn main_timer_xtal_off(&self) -> MAIN_TIMER_XTAL_OFF_R {
        MAIN_TIMER_XTAL_OFF_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn main_timer_sys_stall(&self) -> MAIN_TIMER_SYS_STALL_R {
        MAIN_TIMER_SYS_STALL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn main_timer_sys_rst(&self) -> MAIN_TIMER_SYS_RST_R {
        MAIN_TIMER_SYS_RST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UPDATE")
            .field(
                "main_timer_xtal_off",
                &format_args!("{}", self.main_timer_xtal_off().bit()),
            )
            .field(
                "main_timer_sys_stall",
                &format_args!("{}", self.main_timer_sys_stall().bit()),
            )
            .field(
                "main_timer_sys_rst",
                &format_args!("{}", self.main_timer_sys_rst().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<UPDATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn main_timer_update(&mut self) -> MAIN_TIMER_UPDATE_W<28> {
        MAIN_TIMER_UPDATE_W::new(self)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn main_timer_xtal_off(&mut self) -> MAIN_TIMER_XTAL_OFF_W<29> {
        MAIN_TIMER_XTAL_OFF_W::new(self)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn main_timer_sys_stall(&mut self) -> MAIN_TIMER_SYS_STALL_W<30> {
        MAIN_TIMER_SYS_STALL_W::new(self)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn main_timer_sys_rst(&mut self) -> MAIN_TIMER_SYS_RST_W<31> {
        MAIN_TIMER_SYS_RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [update](index.html) module"]
pub struct UPDATE_SPEC;
impl crate::RegisterSpec for UPDATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [update::R](R) reader structure"]
impl crate::Readable for UPDATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [update::W](W) writer structure"]
impl crate::Writable for UPDATE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UPDATE to value 0"]
impl crate::Resettable for UPDATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
