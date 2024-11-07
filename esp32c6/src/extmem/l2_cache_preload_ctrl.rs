#[doc = "Register `L2_CACHE_PRELOAD_CTRL` reader"]
pub type R = crate::R<L2_CACHE_PRELOAD_CTRL_SPEC>;
#[doc = "Register `L2_CACHE_PRELOAD_CTRL` writer"]
pub type W = crate::W<L2_CACHE_PRELOAD_CTRL_SPEC>;
#[doc = "Field `L2_CACHE_PRELOAD_ENA` reader - The bit is used to enable preload operation on L2-Cache. It will be cleared by hardware automatically after preload operation is done."]
pub type L2_CACHE_PRELOAD_ENA_R = crate::BitReader;
#[doc = "Field `L2_CACHE_PRELOAD_ENA` writer - The bit is used to enable preload operation on L2-Cache. It will be cleared by hardware automatically after preload operation is done."]
pub type L2_CACHE_PRELOAD_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L2_CACHE_PRELOAD_DONE` reader - The bit is used to indicate whether preload operation is finished or not. 0: not finished. 1: finished."]
pub type L2_CACHE_PRELOAD_DONE_R = crate::BitReader;
#[doc = "Field `L2_CACHE_PRELOAD_ORDER` reader - The bit is used to configure the direction of preload operation. 0: ascending, 1: descending."]
pub type L2_CACHE_PRELOAD_ORDER_R = crate::BitReader;
#[doc = "Field `L2_CACHE_PRELOAD_RGID` reader - The bit is used to set the gid of l2 cache preload."]
pub type L2_CACHE_PRELOAD_RGID_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - The bit is used to enable preload operation on L2-Cache. It will be cleared by hardware automatically after preload operation is done."]
    #[inline(always)]
    pub fn l2_cache_preload_ena(&self) -> L2_CACHE_PRELOAD_ENA_R {
        L2_CACHE_PRELOAD_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to indicate whether preload operation is finished or not. 0: not finished. 1: finished."]
    #[inline(always)]
    pub fn l2_cache_preload_done(&self) -> L2_CACHE_PRELOAD_DONE_R {
        L2_CACHE_PRELOAD_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The bit is used to configure the direction of preload operation. 0: ascending, 1: descending."]
    #[inline(always)]
    pub fn l2_cache_preload_order(&self) -> L2_CACHE_PRELOAD_ORDER_R {
        L2_CACHE_PRELOAD_ORDER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:6 - The bit is used to set the gid of l2 cache preload."]
    #[inline(always)]
    pub fn l2_cache_preload_rgid(&self) -> L2_CACHE_PRELOAD_RGID_R {
        L2_CACHE_PRELOAD_RGID_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_CACHE_PRELOAD_CTRL")
            .field("l2_cache_preload_ena", &self.l2_cache_preload_ena())
            .field("l2_cache_preload_done", &self.l2_cache_preload_done())
            .field("l2_cache_preload_order", &self.l2_cache_preload_order())
            .field("l2_cache_preload_rgid", &self.l2_cache_preload_rgid())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to enable preload operation on L2-Cache. It will be cleared by hardware automatically after preload operation is done."]
    #[inline(always)]
    pub fn l2_cache_preload_ena(&mut self) -> L2_CACHE_PRELOAD_ENA_W<L2_CACHE_PRELOAD_CTRL_SPEC> {
        L2_CACHE_PRELOAD_ENA_W::new(self, 0)
    }
}
#[doc = "L2 Cache preload-operation control register\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_cache_preload_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_cache_preload_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2_CACHE_PRELOAD_CTRL_SPEC;
impl crate::RegisterSpec for L2_CACHE_PRELOAD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_cache_preload_ctrl::R`](R) reader structure"]
impl crate::Readable for L2_CACHE_PRELOAD_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l2_cache_preload_ctrl::W`](W) writer structure"]
impl crate::Writable for L2_CACHE_PRELOAD_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets L2_CACHE_PRELOAD_CTRL to value 0x02"]
impl crate::Resettable for L2_CACHE_PRELOAD_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
