#[doc = "Register `REG_UPDATE` writer"]
pub type W = crate::W<REG_UPDATE_SPEC>;
#[doc = "Field `RX_REG_UPDATE` writer - Set this bit to update rx register configuration."]
pub type RX_REG_UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REG_UPDATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 31 - Set this bit to update rx register configuration."]
    #[inline(always)]
    pub fn rx_reg_update(&mut self) -> RX_REG_UPDATE_W<'_, REG_UPDATE_SPEC> {
        RX_REG_UPDATE_W::new(self, 31)
    }
}
#[doc = "Parallel IO FIFO configuration register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_update::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REG_UPDATE_SPEC;
impl crate::RegisterSpec for REG_UPDATE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`reg_update::W`](W) writer structure"]
impl crate::Writable for REG_UPDATE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_UPDATE to value 0"]
impl crate::Resettable for REG_UPDATE_SPEC {}
