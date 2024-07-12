#[doc = "Register `DCACHE_PRELOAD_CTRL` reader"]
pub type R = crate::R<DCACHE_PRELOAD_CTRL_SPEC>;
#[doc = "Register `DCACHE_PRELOAD_CTRL` writer"]
pub type W = crate::W<DCACHE_PRELOAD_CTRL_SPEC>;
#[doc = "Field `DCACHE_PRELOAD_ENA` reader - The bit is used to enable preload operation. It will be cleared by hardware after preload operation done."]
pub type DCACHE_PRELOAD_ENA_R = crate::BitReader;
#[doc = "Field `DCACHE_PRELOAD_ENA` writer - The bit is used to enable preload operation. It will be cleared by hardware after preload operation done."]
pub type DCACHE_PRELOAD_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCACHE_PRELOAD_DONE` reader - The bit is used to indicate preload operation is finished."]
pub type DCACHE_PRELOAD_DONE_R = crate::BitReader;
#[doc = "Field `DCACHE_PRELOAD_ORDER` reader - The bit is used to configure the direction of preload operation. 1: descending, 0: ascending."]
pub type DCACHE_PRELOAD_ORDER_R = crate::BitReader;
#[doc = "Field `DCACHE_PRELOAD_ORDER` writer - The bit is used to configure the direction of preload operation. 1: descending, 0: ascending."]
pub type DCACHE_PRELOAD_ORDER_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The bit is used to enable preload operation. It will be cleared by hardware after preload operation done."]
    #[inline(always)]
    pub fn dcache_preload_ena(&self) -> DCACHE_PRELOAD_ENA_R {
        DCACHE_PRELOAD_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to indicate preload operation is finished."]
    #[inline(always)]
    pub fn dcache_preload_done(&self) -> DCACHE_PRELOAD_DONE_R {
        DCACHE_PRELOAD_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The bit is used to configure the direction of preload operation. 1: descending, 0: ascending."]
    #[inline(always)]
    pub fn dcache_preload_order(&self) -> DCACHE_PRELOAD_ORDER_R {
        DCACHE_PRELOAD_ORDER_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCACHE_PRELOAD_CTRL")
            .field("dcache_preload_ena", &self.dcache_preload_ena())
            .field("dcache_preload_done", &self.dcache_preload_done())
            .field("dcache_preload_order", &self.dcache_preload_order())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to enable preload operation. It will be cleared by hardware after preload operation done."]
    #[inline(always)]
    #[must_use]
    pub fn dcache_preload_ena(&mut self) -> DCACHE_PRELOAD_ENA_W<DCACHE_PRELOAD_CTRL_SPEC> {
        DCACHE_PRELOAD_ENA_W::new(self, 0)
    }
    #[doc = "Bit 2 - The bit is used to configure the direction of preload operation. 1: descending, 0: ascending."]
    #[inline(always)]
    #[must_use]
    pub fn dcache_preload_order(&mut self) -> DCACHE_PRELOAD_ORDER_W<DCACHE_PRELOAD_CTRL_SPEC> {
        DCACHE_PRELOAD_ORDER_W::new(self, 2)
    }
}
#[doc = "******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`dcache_preload_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcache_preload_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCACHE_PRELOAD_CTRL_SPEC;
impl crate::RegisterSpec for DCACHE_PRELOAD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcache_preload_ctrl::R`](R) reader structure"]
impl crate::Readable for DCACHE_PRELOAD_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcache_preload_ctrl::W`](W) writer structure"]
impl crate::Writable for DCACHE_PRELOAD_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCACHE_PRELOAD_CTRL to value 0x02"]
impl crate::Resettable for DCACHE_PRELOAD_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
