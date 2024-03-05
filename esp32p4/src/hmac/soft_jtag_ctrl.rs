#[doc = "Register `SOFT_JTAG_CTRL` writer"]
pub type W = crate::W<SOFT_JTAG_CTRL_SPEC>;
#[doc = "Field `SOFT_JTAG_CTRL` writer - Turn on JTAG verification."]
pub type SOFT_JTAG_CTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SOFT_JTAG_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Turn on JTAG verification."]
    #[inline(always)]
    #[must_use]
    pub fn soft_jtag_ctrl(&mut self) -> SOFT_JTAG_CTRL_W<SOFT_JTAG_CTRL_SPEC> {
        SOFT_JTAG_CTRL_W::new(self, 0)
    }
}
#[doc = "Jtag register 0.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soft_jtag_ctrl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SOFT_JTAG_CTRL_SPEC;
impl crate::RegisterSpec for SOFT_JTAG_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`soft_jtag_ctrl::W`](W) writer structure"]
impl crate::Writable for SOFT_JTAG_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SOFT_JTAG_CTRL to value 0"]
impl crate::Resettable for SOFT_JTAG_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
