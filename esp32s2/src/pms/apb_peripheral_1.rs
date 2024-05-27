///Register `APB_PERIPHERAL_1` reader
pub type R = crate::R<APB_PERIPHERAL_1_SPEC>;
///Register `APB_PERIPHERAL_1` writer
pub type W = crate::W<APB_PERIPHERAL_1_SPEC>;
///Field `APB_PERIPHERAL_SPLIT_BURST` reader - Setting to 1 splits the data phase of the last access and the address phase of following access.
pub type APB_PERIPHERAL_SPLIT_BURST_R = crate::BitReader;
///Field `APB_PERIPHERAL_SPLIT_BURST` writer - Setting to 1 splits the data phase of the last access and the address phase of following access.
pub type APB_PERIPHERAL_SPLIT_BURST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Setting to 1 splits the data phase of the last access and the address phase of following access.
    #[inline(always)]
    pub fn apb_peripheral_split_burst(&self) -> APB_PERIPHERAL_SPLIT_BURST_R {
        APB_PERIPHERAL_SPLIT_BURST_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_PERIPHERAL_1")
            .field(
                "apb_peripheral_split_burst",
                &self.apb_peripheral_split_burst(),
            )
            .finish()
    }
}
impl W {
    ///Bit 0 - Setting to 1 splits the data phase of the last access and the address phase of following access.
    #[inline(always)]
    #[must_use]
    pub fn apb_peripheral_split_burst(
        &mut self,
    ) -> APB_PERIPHERAL_SPLIT_BURST_W<APB_PERIPHERAL_1_SPEC> {
        APB_PERIPHERAL_SPLIT_BURST_W::new(self, 0)
    }
}
/**Peripheral access permission control register 1.

You can [`read`](crate::generic::Reg::read) this register and get [`apb_peripheral_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb_peripheral_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct APB_PERIPHERAL_1_SPEC;
impl crate::RegisterSpec for APB_PERIPHERAL_1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`apb_peripheral_1::R`](R) reader structure
impl crate::Readable for APB_PERIPHERAL_1_SPEC {}
///`write(|w| ..)` method takes [`apb_peripheral_1::W`](W) writer structure
impl crate::Writable for APB_PERIPHERAL_1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets APB_PERIPHERAL_1 to value 0x01
impl crate::Resettable for APB_PERIPHERAL_1_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
