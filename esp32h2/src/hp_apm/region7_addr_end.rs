#[doc = "Register `REGION7_ADDR_END` reader"]
pub type R = crate::R<REGION7_ADDR_END_SPEC>;
#[doc = "Register `REGION7_ADDR_END` writer"]
pub type W = crate::W<REGION7_ADDR_END_SPEC>;
#[doc = "Field `REGION7_ADDR_END` reader - End address of region7"]
pub type REGION7_ADDR_END_R = crate::FieldReader<u32>;
#[doc = "Field `REGION7_ADDR_END` writer - End address of region7"]
pub type REGION7_ADDR_END_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - End address of region7"]
    #[inline(always)]
    pub fn region7_addr_end(&self) -> REGION7_ADDR_END_R {
        REGION7_ADDR_END_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGION7_ADDR_END")
            .field(
                "region7_addr_end",
                &format_args!("{}", self.region7_addr_end().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REGION7_ADDR_END_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - End address of region7"]
    #[inline(always)]
    #[must_use]
    pub fn region7_addr_end(&mut self) -> REGION7_ADDR_END_W<REGION7_ADDR_END_SPEC> {
        REGION7_ADDR_END_W::new(self, 0)
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
#[doc = "Region address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region7_addr_end::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region7_addr_end::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGION7_ADDR_END_SPEC;
impl crate::RegisterSpec for REGION7_ADDR_END_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`region7_addr_end::R`](R) reader structure"]
impl crate::Readable for REGION7_ADDR_END_SPEC {}
#[doc = "`write(|w| ..)` method takes [`region7_addr_end::W`](W) writer structure"]
impl crate::Writable for REGION7_ADDR_END_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGION7_ADDR_END to value 0xffff_ffff"]
impl crate::Resettable for REGION7_ADDR_END_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
