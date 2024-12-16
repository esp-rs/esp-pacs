#[doc = "Register `JTAG_CTRL_7` writer"]
pub type W = crate::W<JTAG_CTRL_7_SPEC>;
#[doc = "Field `CANCEL_EFUSE_DISABLE_JTAG_TEMPORARY_7` writer - Stores the 0 to 224 bits of the 255 bits register used to cancel the temporary disable of eFuse to JTAG."]
pub type CANCEL_EFUSE_DISABLE_JTAG_TEMPORARY_7_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<JTAG_CTRL_7_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - Stores the 0 to 224 bits of the 255 bits register used to cancel the temporary disable of eFuse to JTAG."]
    #[inline(always)]
    pub fn cancel_efuse_disable_jtag_temporary_7(
        &mut self,
    ) -> CANCEL_EFUSE_DISABLE_JTAG_TEMPORARY_7_W<JTAG_CTRL_7_SPEC> {
        CANCEL_EFUSE_DISABLE_JTAG_TEMPORARY_7_W::new(self, 0)
    }
}
#[doc = "JTAG configuration register 7\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jtag_ctrl_7::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct JTAG_CTRL_7_SPEC;
impl crate::RegisterSpec for JTAG_CTRL_7_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`jtag_ctrl_7::W`](W) writer structure"]
impl crate::Writable for JTAG_CTRL_7_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets JTAG_CTRL_7 to value 0"]
impl crate::Resettable for JTAG_CTRL_7_SPEC {
    const RESET_VALUE: u32 = 0;
}
