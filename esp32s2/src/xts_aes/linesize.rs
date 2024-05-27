///Register `LINESIZE` reader
pub type R = crate::R<LINESIZE_SPEC>;
///Register `LINESIZE` writer
pub type W = crate::W<LINESIZE_SPEC>;
///Field `LINESIZE` reader - Configures the data size of a single encryption. 0: 128 bits. 1: 256 bits. 2: 512 bits.
pub type LINESIZE_R = crate::FieldReader;
///Field `LINESIZE` writer - Configures the data size of a single encryption. 0: 128 bits. 1: 256 bits. 2: 512 bits.
pub type LINESIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - Configures the data size of a single encryption. 0: 128 bits. 1: 256 bits. 2: 512 bits.
    #[inline(always)]
    pub fn linesize(&self) -> LINESIZE_R {
        LINESIZE_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LINESIZE")
            .field("linesize", &self.linesize())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Configures the data size of a single encryption. 0: 128 bits. 1: 256 bits. 2: 512 bits.
    #[inline(always)]
    #[must_use]
    pub fn linesize(&mut self) -> LINESIZE_W<LINESIZE_SPEC> {
        LINESIZE_W::new(self, 0)
    }
}
/**Configures the size of target memory space

You can [`read`](crate::generic::Reg::read) this register and get [`linesize::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`linesize::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LINESIZE_SPEC;
impl crate::RegisterSpec for LINESIZE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`linesize::R`](R) reader structure
impl crate::Readable for LINESIZE_SPEC {}
///`write(|w| ..)` method takes [`linesize::W`](W) writer structure
impl crate::Writable for LINESIZE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LINESIZE to value 0
impl crate::Resettable for LINESIZE_SPEC {
    const RESET_VALUE: u32 = 0;
}
