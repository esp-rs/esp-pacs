#[doc = "Register `REGION11_ADDR_START` reader"]
pub type R = crate::R<REGION11_ADDR_START_SPEC>;
#[doc = "Register `REGION11_ADDR_START` writer"]
pub type W = crate::W<REGION11_ADDR_START_SPEC>;
#[doc = "Field `REGION11_ADDR_START` reader - Start address of region11"]
pub type REGION11_ADDR_START_R = crate::FieldReader<u32>;
#[doc = "Field `REGION11_ADDR_START` writer - Start address of region11"]
pub type REGION11_ADDR_START_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Start address of region11"]
    #[inline(always)]
    pub fn region11_addr_start(&self) -> REGION11_ADDR_START_R {
        REGION11_ADDR_START_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGION11_ADDR_START")
            .field(
                "region11_addr_start",
                &format_args!("{}", self.region11_addr_start().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REGION11_ADDR_START_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start address of region11"]
    #[inline(always)]
    #[must_use]
    pub fn region11_addr_start(&mut self) -> REGION11_ADDR_START_W<REGION11_ADDR_START_SPEC> {
        REGION11_ADDR_START_W::new(self, 0)
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
#[doc = "Region address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region11_addr_start::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region11_addr_start::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGION11_ADDR_START_SPEC;
impl crate::RegisterSpec for REGION11_ADDR_START_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`region11_addr_start::R`](R) reader structure"]
impl crate::Readable for REGION11_ADDR_START_SPEC {}
#[doc = "`write(|w| ..)` method takes [`region11_addr_start::W`](W) writer structure"]
impl crate::Writable for REGION11_ADDR_START_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGION11_ADDR_START to value 0"]
impl crate::Resettable for REGION11_ADDR_START_SPEC {
    const RESET_VALUE: u32 = 0;
}
