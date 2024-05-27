///Register `TEXT_IN[%s]` reader
pub type R = crate::R<TEXT_IN_SPEC>;
///Register `TEXT_IN[%s]` writer
pub type W = crate::W<TEXT_IN_SPEC>;
///Field `TEXT_IN` reader - Stores the source data when the AES accelerator operates in the Typical AES working mode.
pub type TEXT_IN_R = crate::FieldReader<u32>;
///Field `TEXT_IN` writer - Stores the source data when the AES accelerator operates in the Typical AES working mode.
pub type TEXT_IN_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl R {
    ///Bits 0:31 - Stores the source data when the AES accelerator operates in the Typical AES working mode.
    #[inline(always)]
    pub fn text_in(&self) -> TEXT_IN_R {
        TEXT_IN_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TEXT_IN")
            .field("text_in", &self.text_in())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Stores the source data when the AES accelerator operates in the Typical AES working mode.
    #[inline(always)]
    #[must_use]
    pub fn text_in(&mut self) -> TEXT_IN_W<TEXT_IN_SPEC> {
        TEXT_IN_W::new(self, 0)
    }
}
/**Source data register %s

You can [`read`](crate::generic::Reg::read) this register and get [`text_in::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`text_in::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TEXT_IN_SPEC;
impl crate::RegisterSpec for TEXT_IN_SPEC {
    type Ux = u32;
}
///`read()` method returns [`text_in::R`](R) reader structure
impl crate::Readable for TEXT_IN_SPEC {}
///`write(|w| ..)` method takes [`text_in::W`](W) writer structure
impl crate::Writable for TEXT_IN_SPEC {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TEXT_IN[%s] to value 0
impl crate::Resettable for TEXT_IN_SPEC {
    const RESET_VALUE: u32 = 0;
}
