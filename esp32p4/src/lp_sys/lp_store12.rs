///Register `LP_STORE12` reader
pub type R = crate::R<LP_STORE12_SPEC>;
///Register `LP_STORE12` writer
pub type W = crate::W<LP_STORE12_SPEC>;
///Field `LP_SCRATCH12` reader - need_des
pub type LP_SCRATCH12_R = crate::FieldReader<u32>;
///Field `LP_SCRATCH12` writer - need_des
pub type LP_SCRATCH12_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - need_des
    #[inline(always)]
    pub fn lp_scratch12(&self) -> LP_SCRATCH12_R {
        LP_SCRATCH12_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_STORE12")
            .field("lp_scratch12", &self.lp_scratch12())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - need_des
    #[inline(always)]
    #[must_use]
    pub fn lp_scratch12(&mut self) -> LP_SCRATCH12_W<LP_STORE12_SPEC> {
        LP_SCRATCH12_W::new(self, 0)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`lp_store12::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_store12::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LP_STORE12_SPEC;
impl crate::RegisterSpec for LP_STORE12_SPEC {
    type Ux = u32;
}
///`read()` method returns [`lp_store12::R`](R) reader structure
impl crate::Readable for LP_STORE12_SPEC {}
///`write(|w| ..)` method takes [`lp_store12::W`](W) writer structure
impl crate::Writable for LP_STORE12_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LP_STORE12 to value 0
impl crate::Resettable for LP_STORE12_SPEC {
    const RESET_VALUE: u32 = 0;
}
