#[doc = "Register `REGISTER452_SYSTEMTIMESECONDSUPDATEREGISTER` reader"]
pub type R = crate::R<REGISTER452_SYSTEMTIMESECONDSUPDATEREGISTER_SPEC>;
#[doc = "Register `REGISTER452_SYSTEMTIMESECONDSUPDATEREGISTER` writer"]
pub type W = crate::W<REGISTER452_SYSTEMTIMESECONDSUPDATEREGISTER_SPEC>;
#[doc = "Field `TSS` reader - TIMESTAMP SECOND THE VALUE IN THIS FIELD INDICATES THE TIME IN SECONDS TO BE INITIALIZED OR ADDED TO THE SYSTEM TIME"]
pub type TSS_R = crate::FieldReader<u32>;
#[doc = "Field `TSS` writer - TIMESTAMP SECOND THE VALUE IN THIS FIELD INDICATES THE TIME IN SECONDS TO BE INITIALIZED OR ADDED TO THE SYSTEM TIME"]
pub type TSS_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - TIMESTAMP SECOND THE VALUE IN THIS FIELD INDICATES THE TIME IN SECONDS TO BE INITIALIZED OR ADDED TO THE SYSTEM TIME"]
    #[inline(always)]
    pub fn tss(&self) -> TSS_R {
        TSS_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER452_SYSTEMTIMESECONDSUPDATEREGISTER")
            .field("tss", &self.tss())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - TIMESTAMP SECOND THE VALUE IN THIS FIELD INDICATES THE TIME IN SECONDS TO BE INITIALIZED OR ADDED TO THE SYSTEM TIME"]
    #[inline(always)]
    pub fn tss(&mut self) -> TSS_W<'_, REGISTER452_SYSTEMTIMESECONDSUPDATEREGISTER_SPEC> {
        TSS_W::new(self, 0)
    }
}
#[doc = "Contains the lower 32 bits of the seconds field to be written to, added to, or subtracted from the System Time value This register is only present when IEEE1588 timestamping is enabled without an external timestamp input\n\nYou can [`read`](crate::Reg::read) this register and get [`register452_systemtimesecondsupdateregister::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register452_systemtimesecondsupdateregister::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER452_SYSTEMTIMESECONDSUPDATEREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER452_SYSTEMTIMESECONDSUPDATEREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register452_systemtimesecondsupdateregister::R`](R) reader structure"]
impl crate::Readable for REGISTER452_SYSTEMTIMESECONDSUPDATEREGISTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register452_systemtimesecondsupdateregister::W`](W) writer structure"]
impl crate::Writable for REGISTER452_SYSTEMTIMESECONDSUPDATEREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER452_SYSTEMTIMESECONDSUPDATEREGISTER to value 0"]
impl crate::Resettable for REGISTER452_SYSTEMTIMESECONDSUPDATEREGISTER_SPEC {}
