///Register `LR_ADDR` reader
pub type R = crate::R<LR_ADDR_SPEC>;
///Register `LR_ADDR` writer
pub type W = crate::W<LR_ADDR_SPEC>;
///Field `GLOABLE_LR_ADDR` reader - backup gloable address
pub type GLOABLE_LR_ADDR_R = crate::FieldReader<u32>;
///Field `GLOABLE_LR_ADDR` writer - backup gloable address
pub type GLOABLE_LR_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - backup gloable address
    #[inline(always)]
    pub fn gloable_lr_addr(&self) -> GLOABLE_LR_ADDR_R {
        GLOABLE_LR_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LR_ADDR")
            .field("gloable_lr_addr", &self.gloable_lr_addr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - backup gloable address
    #[inline(always)]
    #[must_use]
    pub fn gloable_lr_addr(&mut self) -> GLOABLE_LR_ADDR_W<LR_ADDR_SPEC> {
        GLOABLE_LR_ADDR_W::new(self, 0)
    }
}
/**gloable lr address regsiter

You can [`read`](crate::generic::Reg::read) this register and get [`lr_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lr_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LR_ADDR_SPEC;
impl crate::RegisterSpec for LR_ADDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`lr_addr::R`](R) reader structure
impl crate::Readable for LR_ADDR_SPEC {}
///`write(|w| ..)` method takes [`lr_addr::W`](W) writer structure
impl crate::Writable for LR_ADDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LR_ADDR to value 0
impl crate::Resettable for LR_ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
