#[doc = "Register `MEM_ADDR` reader"]
pub type R = crate::R<MEM_ADDR_SPEC>;
#[doc = "Register `MEM_ADDR` writer"]
pub type W = crate::W<MEM_ADDR_SPEC>;
#[doc = "Field `MEM_START_ADDR` reader - x"]
pub type MEM_START_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `MEM_START_ADDR` writer - x"]
pub type MEM_START_ADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - x"]
    #[inline(always)]
    pub fn mem_start_addr(&self) -> MEM_START_ADDR_R {
        MEM_START_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_ADDR")
            .field(
                "mem_start_addr",
                &format_args!("{}", self.mem_start_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MEM_ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - x"]
    #[inline(always)]
    #[must_use]
    pub fn mem_start_addr(&mut self) -> MEM_START_ADDR_W<MEM_ADDR_SPEC, 0> {
        MEM_START_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_ADDR_SPEC;
impl crate::RegisterSpec for MEM_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_addr::R`](R) reader structure"]
impl crate::Readable for MEM_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mem_addr::W`](W) writer structure"]
impl crate::Writable for MEM_ADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MEM_ADDR to value 0"]
impl crate::Resettable for MEM_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
