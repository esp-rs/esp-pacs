#[doc = "Register `PRO_ICACHE_PRELOAD_SIZE` reader"]
pub type R = crate::R<PRO_ICACHE_PRELOAD_SIZE_SPEC>;
#[doc = "Register `PRO_ICACHE_PRELOAD_SIZE` writer"]
pub type W = crate::W<PRO_ICACHE_PRELOAD_SIZE_SPEC>;
#[doc = "Field `PRO_ICACHE_PRELOAD_SIZE` reader - The bits are used to configure the length for manual pre-load operation. It should be combined with PRO_ICACHE_PRELOAD_ADDR_REG.."]
pub type PRO_ICACHE_PRELOAD_SIZE_R = crate::FieldReader<u16>;
#[doc = "Field `PRO_ICACHE_PRELOAD_SIZE` writer - The bits are used to configure the length for manual pre-load operation. It should be combined with PRO_ICACHE_PRELOAD_ADDR_REG.."]
pub type PRO_ICACHE_PRELOAD_SIZE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
#[doc = "Field `PRO_ICACHE_PRELOAD_ORDER` reader - The bits are used to configure the direction of manual pre-load operation. 1: descending, 0: ascending."]
pub type PRO_ICACHE_PRELOAD_ORDER_R = crate::BitReader;
#[doc = "Field `PRO_ICACHE_PRELOAD_ORDER` writer - The bits are used to configure the direction of manual pre-load operation. 1: descending, 0: ascending."]
pub type PRO_ICACHE_PRELOAD_ORDER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:9 - The bits are used to configure the length for manual pre-load operation. It should be combined with PRO_ICACHE_PRELOAD_ADDR_REG.."]
    #[inline(always)]
    pub fn pro_icache_preload_size(&self) -> PRO_ICACHE_PRELOAD_SIZE_R {
        PRO_ICACHE_PRELOAD_SIZE_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - The bits are used to configure the direction of manual pre-load operation. 1: descending, 0: ascending."]
    #[inline(always)]
    pub fn pro_icache_preload_order(&self) -> PRO_ICACHE_PRELOAD_ORDER_R {
        PRO_ICACHE_PRELOAD_ORDER_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_ICACHE_PRELOAD_SIZE")
            .field(
                "pro_icache_preload_size",
                &format_args!("{}", self.pro_icache_preload_size().bits()),
            )
            .field(
                "pro_icache_preload_order",
                &format_args!("{}", self.pro_icache_preload_order().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_ICACHE_PRELOAD_SIZE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:9 - The bits are used to configure the length for manual pre-load operation. It should be combined with PRO_ICACHE_PRELOAD_ADDR_REG.."]
    #[inline(always)]
    #[must_use]
    pub fn pro_icache_preload_size(
        &mut self,
    ) -> PRO_ICACHE_PRELOAD_SIZE_W<PRO_ICACHE_PRELOAD_SIZE_SPEC, 0> {
        PRO_ICACHE_PRELOAD_SIZE_W::new(self)
    }
    #[doc = "Bit 10 - The bits are used to configure the direction of manual pre-load operation. 1: descending, 0: ascending."]
    #[inline(always)]
    #[must_use]
    pub fn pro_icache_preload_order(
        &mut self,
    ) -> PRO_ICACHE_PRELOAD_ORDER_W<PRO_ICACHE_PRELOAD_SIZE_SPEC, 10> {
        PRO_ICACHE_PRELOAD_ORDER_W::new(self)
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
#[doc = "register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_icache_preload_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_icache_preload_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_ICACHE_PRELOAD_SIZE_SPEC;
impl crate::RegisterSpec for PRO_ICACHE_PRELOAD_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_icache_preload_size::R`](R) reader structure"]
impl crate::Readable for PRO_ICACHE_PRELOAD_SIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pro_icache_preload_size::W`](W) writer structure"]
impl crate::Writable for PRO_ICACHE_PRELOAD_SIZE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRO_ICACHE_PRELOAD_SIZE to value 0x0200"]
impl crate::Resettable for PRO_ICACHE_PRELOAD_SIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200;
}
