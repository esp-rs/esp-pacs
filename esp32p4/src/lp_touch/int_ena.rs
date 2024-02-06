#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `SCAN_DONE_INT_ENA` reader - need_des"]
pub type SCAN_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `SCAN_DONE_INT_ENA` writer - need_des"]
pub type SCAN_DONE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DONE_INT_ENA` reader - need_des"]
pub type DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `DONE_INT_ENA` writer - need_des"]
pub type DONE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACTIVE_INT_ENA` reader - need_des"]
pub type ACTIVE_INT_ENA_R = crate::BitReader;
#[doc = "Field `ACTIVE_INT_ENA` writer - need_des"]
pub type ACTIVE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INACTIVE_INT_ENA` reader - need_des"]
pub type INACTIVE_INT_ENA_R = crate::BitReader;
#[doc = "Field `INACTIVE_INT_ENA` writer - need_des"]
pub type INACTIVE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUT_INT_ENA` reader - need_des"]
pub type TIMEOUT_INT_ENA_R = crate::BitReader;
#[doc = "Field `TIMEOUT_INT_ENA` writer - need_des"]
pub type TIMEOUT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APPROACH_LOOP_DONE_INT_ENA` reader - need_des"]
pub type APPROACH_LOOP_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `APPROACH_LOOP_DONE_INT_ENA` writer - need_des"]
pub type APPROACH_LOOP_DONE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn scan_done_int_ena(&self) -> SCAN_DONE_INT_ENA_R {
        SCAN_DONE_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn done_int_ena(&self) -> DONE_INT_ENA_R {
        DONE_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn active_int_ena(&self) -> ACTIVE_INT_ENA_R {
        ACTIVE_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn inactive_int_ena(&self) -> INACTIVE_INT_ENA_R {
        INACTIVE_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn timeout_int_ena(&self) -> TIMEOUT_INT_ENA_R {
        TIMEOUT_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn approach_loop_done_int_ena(&self) -> APPROACH_LOOP_DONE_INT_ENA_R {
        APPROACH_LOOP_DONE_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field(
                "scan_done_int_ena",
                &format_args!("{}", self.scan_done_int_ena().bit()),
            )
            .field(
                "done_int_ena",
                &format_args!("{}", self.done_int_ena().bit()),
            )
            .field(
                "active_int_ena",
                &format_args!("{}", self.active_int_ena().bit()),
            )
            .field(
                "inactive_int_ena",
                &format_args!("{}", self.inactive_int_ena().bit()),
            )
            .field(
                "timeout_int_ena",
                &format_args!("{}", self.timeout_int_ena().bit()),
            )
            .field(
                "approach_loop_done_int_ena",
                &format_args!("{}", self.approach_loop_done_int_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn scan_done_int_ena(&mut self) -> SCAN_DONE_INT_ENA_W<INT_ENA_SPEC> {
        SCAN_DONE_INT_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn done_int_ena(&mut self) -> DONE_INT_ENA_W<INT_ENA_SPEC> {
        DONE_INT_ENA_W::new(self, 1)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn active_int_ena(&mut self) -> ACTIVE_INT_ENA_W<INT_ENA_SPEC> {
        ACTIVE_INT_ENA_W::new(self, 2)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn inactive_int_ena(&mut self) -> INACTIVE_INT_ENA_W<INT_ENA_SPEC> {
        INACTIVE_INT_ENA_W::new(self, 3)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn timeout_int_ena(&mut self) -> TIMEOUT_INT_ENA_W<INT_ENA_SPEC> {
        TIMEOUT_INT_ENA_W::new(self, 4)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn approach_loop_done_int_ena(&mut self) -> APPROACH_LOOP_DONE_INT_ENA_W<INT_ENA_SPEC> {
        APPROACH_LOOP_DONE_INT_ENA_W::new(self, 5)
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
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
