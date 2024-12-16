#[doc = "Register `APB_PERIPHERAL_0` reader"]
pub type R = crate::R<APB_PERIPHERAL_0_SPEC>;
#[doc = "Register `APB_PERIPHERAL_0` writer"]
pub type W = crate::W<APB_PERIPHERAL_0_SPEC>;
#[doc = "Field `APB_PERIPHERAL_LOCK` reader - Lock register. Setting to 1 locks TX Copy DMA permission control registers."]
pub type APB_PERIPHERAL_LOCK_R = crate::BitReader;
#[doc = "Field `APB_PERIPHERAL_LOCK` writer - Lock register. Setting to 1 locks TX Copy DMA permission control registers."]
pub type APB_PERIPHERAL_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Lock register. Setting to 1 locks TX Copy DMA permission control registers."]
    #[inline(always)]
    pub fn apb_peripheral_lock(&self) -> APB_PERIPHERAL_LOCK_R {
        APB_PERIPHERAL_LOCK_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_PERIPHERAL_0")
            .field("apb_peripheral_lock", &self.apb_peripheral_lock())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Lock register. Setting to 1 locks TX Copy DMA permission control registers."]
    #[inline(always)]
    pub fn apb_peripheral_lock(&mut self) -> APB_PERIPHERAL_LOCK_W<APB_PERIPHERAL_0_SPEC> {
        APB_PERIPHERAL_LOCK_W::new(self, 0)
    }
}
#[doc = "Peripheral access permission control register 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb_peripheral_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb_peripheral_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB_PERIPHERAL_0_SPEC;
impl crate::RegisterSpec for APB_PERIPHERAL_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb_peripheral_0::R`](R) reader structure"]
impl crate::Readable for APB_PERIPHERAL_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb_peripheral_0::W`](W) writer structure"]
impl crate::Writable for APB_PERIPHERAL_0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB_PERIPHERAL_0 to value 0"]
impl crate::Resettable for APB_PERIPHERAL_0_SPEC {
    const RESET_VALUE: u32 = 0;
}
