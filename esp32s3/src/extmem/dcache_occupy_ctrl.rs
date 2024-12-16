#[doc = "Register `DCACHE_OCCUPY_CTRL` reader"]
pub type R = crate::R<DCACHE_OCCUPY_CTRL_SPEC>;
#[doc = "Register `DCACHE_OCCUPY_CTRL` writer"]
pub type W = crate::W<DCACHE_OCCUPY_CTRL_SPEC>;
#[doc = "Field `DCACHE_OCCUPY_ENA` reader - The bit is used to enable occupy operation. It will be cleared by hardware after issuing Auot-Invalidate Operation."]
pub type DCACHE_OCCUPY_ENA_R = crate::BitReader;
#[doc = "Field `DCACHE_OCCUPY_ENA` writer - The bit is used to enable occupy operation. It will be cleared by hardware after issuing Auot-Invalidate Operation."]
pub type DCACHE_OCCUPY_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCACHE_OCCUPY_DONE` reader - The bit is used to indicate occupy operation is finished."]
pub type DCACHE_OCCUPY_DONE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The bit is used to enable occupy operation. It will be cleared by hardware after issuing Auot-Invalidate Operation."]
    #[inline(always)]
    pub fn dcache_occupy_ena(&self) -> DCACHE_OCCUPY_ENA_R {
        DCACHE_OCCUPY_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to indicate occupy operation is finished."]
    #[inline(always)]
    pub fn dcache_occupy_done(&self) -> DCACHE_OCCUPY_DONE_R {
        DCACHE_OCCUPY_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCACHE_OCCUPY_CTRL")
            .field("dcache_occupy_ena", &self.dcache_occupy_ena())
            .field("dcache_occupy_done", &self.dcache_occupy_done())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to enable occupy operation. It will be cleared by hardware after issuing Auot-Invalidate Operation."]
    #[inline(always)]
    pub fn dcache_occupy_ena(&mut self) -> DCACHE_OCCUPY_ENA_W<DCACHE_OCCUPY_CTRL_SPEC> {
        DCACHE_OCCUPY_ENA_W::new(self, 0)
    }
}
#[doc = "******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`dcache_occupy_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcache_occupy_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCACHE_OCCUPY_CTRL_SPEC;
impl crate::RegisterSpec for DCACHE_OCCUPY_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcache_occupy_ctrl::R`](R) reader structure"]
impl crate::Readable for DCACHE_OCCUPY_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcache_occupy_ctrl::W`](W) writer structure"]
impl crate::Writable for DCACHE_OCCUPY_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCACHE_OCCUPY_CTRL to value 0x02"]
impl crate::Resettable for DCACHE_OCCUPY_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
