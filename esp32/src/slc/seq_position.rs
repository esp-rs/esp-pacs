///Register `SEQ_POSITION` reader
pub type R = crate::R<SEQ_POSITION_SPEC>;
///Register `SEQ_POSITION` writer
pub type W = crate::W<SEQ_POSITION_SPEC>;
///Field `SLC0_SEQ_POSITION` reader -
pub type SLC0_SEQ_POSITION_R = crate::FieldReader;
///Field `SLC0_SEQ_POSITION` writer -
pub type SLC0_SEQ_POSITION_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `SLC1_SEQ_POSITION` reader -
pub type SLC1_SEQ_POSITION_R = crate::FieldReader;
///Field `SLC1_SEQ_POSITION` writer -
pub type SLC1_SEQ_POSITION_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7
    #[inline(always)]
    pub fn slc0_seq_position(&self) -> SLC0_SEQ_POSITION_R {
        SLC0_SEQ_POSITION_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15
    #[inline(always)]
    pub fn slc1_seq_position(&self) -> SLC1_SEQ_POSITION_R {
        SLC1_SEQ_POSITION_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEQ_POSITION")
            .field("slc0_seq_position", &self.slc0_seq_position())
            .field("slc1_seq_position", &self.slc1_seq_position())
            .finish()
    }
}
impl W {
    ///Bits 0:7
    #[inline(always)]
    #[must_use]
    pub fn slc0_seq_position(&mut self) -> SLC0_SEQ_POSITION_W<SEQ_POSITION_SPEC> {
        SLC0_SEQ_POSITION_W::new(self, 0)
    }
    ///Bits 8:15
    #[inline(always)]
    #[must_use]
    pub fn slc1_seq_position(&mut self) -> SLC1_SEQ_POSITION_W<SEQ_POSITION_SPEC> {
        SLC1_SEQ_POSITION_W::new(self, 8)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`seq_position::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seq_position::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SEQ_POSITION_SPEC;
impl crate::RegisterSpec for SEQ_POSITION_SPEC {
    type Ux = u32;
}
///`read()` method returns [`seq_position::R`](R) reader structure
impl crate::Readable for SEQ_POSITION_SPEC {}
///`write(|w| ..)` method takes [`seq_position::W`](W) writer structure
impl crate::Writable for SEQ_POSITION_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SEQ_POSITION to value 0x0509
impl crate::Resettable for SEQ_POSITION_SPEC {
    const RESET_VALUE: u32 = 0x0509;
}
