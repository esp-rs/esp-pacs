#[doc = "Register `REGION14_ADDR_START` reader"]
pub type R = crate::R<REGION14_ADDR_START_SPEC>;
#[doc = "Register `REGION14_ADDR_START` writer"]
pub type W = crate::W<REGION14_ADDR_START_SPEC>;
#[doc = "Field `REGION14_ADDR_START` reader - Start address of region14"]
pub type REGION14_ADDR_START_R = crate::FieldReader<u32>;
#[doc = "Field `REGION14_ADDR_START` writer - Start address of region14"]
pub type REGION14_ADDR_START_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Start address of region14"]
    #[inline(always)]
    pub fn region14_addr_start(&self) -> REGION14_ADDR_START_R {
        REGION14_ADDR_START_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGION14_ADDR_START")
            .field(
                "region14_addr_start",
                &format_args!("{}", self.region14_addr_start().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REGION14_ADDR_START_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start address of region14"]
    #[inline(always)]
    #[must_use]
    pub fn region14_addr_start(&mut self) -> REGION14_ADDR_START_W<REGION14_ADDR_START_SPEC, 0> {
        REGION14_ADDR_START_W::new(self)
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
#[doc = "Region address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region14_addr_start::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region14_addr_start::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGION14_ADDR_START_SPEC;
impl crate::RegisterSpec for REGION14_ADDR_START_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`region14_addr_start::R`](R) reader structure"]
impl crate::Readable for REGION14_ADDR_START_SPEC {}
#[doc = "`write(|w| ..)` method takes [`region14_addr_start::W`](W) writer structure"]
impl crate::Writable for REGION14_ADDR_START_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REGION14_ADDR_START to value 0"]
impl crate::Resettable for REGION14_ADDR_START_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
