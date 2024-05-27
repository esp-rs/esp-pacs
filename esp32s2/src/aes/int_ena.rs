///Register `INT_ENA` reader
pub type R = crate::R<INT_ENA_SPEC>;
///Register `INT_ENA` writer
pub type W = crate::W<INT_ENA_SPEC>;
///Field `INT_ENA` reader - Set this bit to 1 to enable AES interrupt and 0 to disable interrupt.
pub type INT_ENA_R = crate::BitReader;
///Field `INT_ENA` writer - Set this bit to 1 to enable AES interrupt and 0 to disable interrupt.
pub type INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Set this bit to 1 to enable AES interrupt and 0 to disable interrupt.
    #[inline(always)]
    pub fn int_ena(&self) -> INT_ENA_R {
        INT_ENA_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field("int_ena", &self.int_ena())
            .finish()
    }
}
impl W {
    ///Bit 0 - Set this bit to 1 to enable AES interrupt and 0 to disable interrupt.
    #[inline(always)]
    #[must_use]
    pub fn int_ena(&mut self) -> INT_ENA_W<INT_ENA_SPEC> {
        INT_ENA_W::new(self, 0)
    }
}
/**DMA-AES interrupt enable register

You can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`int_ena::R`](R) reader structure
impl crate::Readable for INT_ENA_SPEC {}
///`write(|w| ..)` method takes [`int_ena::W`](W) writer structure
impl crate::Writable for INT_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets INT_ENA to value 0
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
