#[doc = "Register `EXT_MEM_WRITEBACK_BYPASS` reader"]
pub type R = crate::R<EXT_MEM_WRITEBACK_BYPASS_SPEC>;
#[doc = "Register `EXT_MEM_WRITEBACK_BYPASS` writer"]
pub type W = crate::W<EXT_MEM_WRITEBACK_BYPASS_SPEC>;
#[doc = "Field `WRITEBACK_BYPASS` reader - Set 1 to bypass cache writeback request to external memory so that spi will not check its attribute."]
pub type WRITEBACK_BYPASS_R = crate::BitReader;
#[doc = "Field `WRITEBACK_BYPASS` writer - Set 1 to bypass cache writeback request to external memory so that spi will not check its attribute."]
pub type WRITEBACK_BYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set 1 to bypass cache writeback request to external memory so that spi will not check its attribute."]
    #[inline(always)]
    pub fn writeback_bypass(&self) -> WRITEBACK_BYPASS_R {
        WRITEBACK_BYPASS_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXT_MEM_WRITEBACK_BYPASS")
            .field(
                "writeback_bypass",
                &format_args!("{}", self.writeback_bypass().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EXT_MEM_WRITEBACK_BYPASS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to bypass cache writeback request to external memory so that spi will not check its attribute."]
    #[inline(always)]
    #[must_use]
    pub fn writeback_bypass(&mut self) -> WRITEBACK_BYPASS_W<EXT_MEM_WRITEBACK_BYPASS_SPEC> {
        WRITEBACK_BYPASS_W::new(self, 0)
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
#[doc = "******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_mem_writeback_bypass::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_mem_writeback_bypass::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXT_MEM_WRITEBACK_BYPASS_SPEC;
impl crate::RegisterSpec for EXT_MEM_WRITEBACK_BYPASS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ext_mem_writeback_bypass::R`](R) reader structure"]
impl crate::Readable for EXT_MEM_WRITEBACK_BYPASS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ext_mem_writeback_bypass::W`](W) writer structure"]
impl crate::Writable for EXT_MEM_WRITEBACK_BYPASS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXT_MEM_WRITEBACK_BYPASS to value 0"]
impl crate::Resettable for EXT_MEM_WRITEBACK_BYPASS_SPEC {
    const RESET_VALUE: u32 = 0;
}
