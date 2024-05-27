///Register `MODEXP_START` writer
pub type W = crate::W<MODEXP_START_SPEC>;
///Field `MODEXP_START` writer - Set this bit to 1 to start the modular exponentiation.
pub type MODEXP_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MODEXP_START_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Set this bit to 1 to start the modular exponentiation.
    #[inline(always)]
    #[must_use]
    pub fn modexp_start(&mut self) -> MODEXP_START_W<MODEXP_START_SPEC> {
        MODEXP_START_W::new(self, 0)
    }
}
/**Modular exponentiation starting bit

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`modexp_start::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MODEXP_START_SPEC;
impl crate::RegisterSpec for MODEXP_START_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`modexp_start::W`](W) writer structure
impl crate::Writable for MODEXP_START_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MODEXP_START to value 0
impl crate::Resettable for MODEXP_START_SPEC {
    const RESET_VALUE: u32 = 0;
}
