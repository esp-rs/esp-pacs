#[doc = "Register `JTAG_CTRL_4` writer"]
pub type W = crate::W<JTAG_CTRL_4_SPEC>;
#[doc = "Field `CANCEL_EFUSE_DISABLE_JTAG_TEMPORARY_4` writer - Stores the 128 to 159 bits of the 256 bits register used to cancel the temporary disable of eFuse to JTAG."]
pub type CANCEL_EFUSE_DISABLE_JTAG_TEMPORARY_4_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<JTAG_CTRL_4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - Stores the 128 to 159 bits of the 256 bits register used to cancel the temporary disable of eFuse to JTAG."]
    #[inline(always)]
    pub fn cancel_efuse_disable_jtag_temporary_4(
        &mut self,
    ) -> CANCEL_EFUSE_DISABLE_JTAG_TEMPORARY_4_W<'_, JTAG_CTRL_4_SPEC> {
        CANCEL_EFUSE_DISABLE_JTAG_TEMPORARY_4_W::new(self, 0)
    }
}
#[doc = "JTAG configuration register 4\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jtag_ctrl_4::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct JTAG_CTRL_4_SPEC;
impl crate::RegisterSpec for JTAG_CTRL_4_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`jtag_ctrl_4::W`](W) writer structure"]
impl crate::Writable for JTAG_CTRL_4_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets JTAG_CTRL_4 to value 0"]
impl crate::Resettable for JTAG_CTRL_4_SPEC {}
