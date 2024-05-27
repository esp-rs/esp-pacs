///Register `PRO_DPORT_APB_MASK1` reader
pub type R = crate::R<PRO_DPORT_APB_MASK1_SPEC>;
///Register `PRO_DPORT_APB_MASK1` writer
pub type W = crate::W<PRO_DPORT_APB_MASK1_SPEC>;
///Field `PRODPORT_APB_MASK1` reader -
pub type PRODPORT_APB_MASK1_R = crate::FieldReader<u32>;
///Field `PRODPORT_APB_MASK1` writer -
pub type PRODPORT_APB_MASK1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31
    #[inline(always)]
    pub fn prodport_apb_mask1(&self) -> PRODPORT_APB_MASK1_R {
        PRODPORT_APB_MASK1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_DPORT_APB_MASK1")
            .field("prodport_apb_mask1", &self.prodport_apb_mask1())
            .finish()
    }
}
impl W {
    ///Bits 0:31
    #[inline(always)]
    #[must_use]
    pub fn prodport_apb_mask1(&mut self) -> PRODPORT_APB_MASK1_W<PRO_DPORT_APB_MASK1_SPEC> {
        PRODPORT_APB_MASK1_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`pro_dport_apb_mask1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_dport_apb_mask1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PRO_DPORT_APB_MASK1_SPEC;
impl crate::RegisterSpec for PRO_DPORT_APB_MASK1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`pro_dport_apb_mask1::R`](R) reader structure
impl crate::Readable for PRO_DPORT_APB_MASK1_SPEC {}
///`write(|w| ..)` method takes [`pro_dport_apb_mask1::W`](W) writer structure
impl crate::Writable for PRO_DPORT_APB_MASK1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PRO_DPORT_APB_MASK1 to value 0
impl crate::Resettable for PRO_DPORT_APB_MASK1_SPEC {
    const RESET_VALUE: u32 = 0;
}
