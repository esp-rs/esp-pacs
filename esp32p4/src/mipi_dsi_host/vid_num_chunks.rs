#[doc = "Register `VID_NUM_CHUNKS` reader"]
pub type R = crate::R<VID_NUM_CHUNKS_SPEC>;
#[doc = "Register `VID_NUM_CHUNKS` writer"]
pub type W = crate::W<VID_NUM_CHUNKS_SPEC>;
#[doc = "Field `VID_NUM_CHUNKS` reader - NA"]
pub type VID_NUM_CHUNKS_R = crate::FieldReader<u16>;
#[doc = "Field `VID_NUM_CHUNKS` writer - NA"]
pub type VID_NUM_CHUNKS_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - NA"]
    #[inline(always)]
    pub fn vid_num_chunks(&self) -> VID_NUM_CHUNKS_R {
        VID_NUM_CHUNKS_R::new((self.bits & 0x1fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VID_NUM_CHUNKS")
            .field("vid_num_chunks", &self.vid_num_chunks())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:12 - NA"]
    #[inline(always)]
    pub fn vid_num_chunks(&mut self) -> VID_NUM_CHUNKS_W<VID_NUM_CHUNKS_SPEC> {
        VID_NUM_CHUNKS_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`vid_num_chunks::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vid_num_chunks::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VID_NUM_CHUNKS_SPEC;
impl crate::RegisterSpec for VID_NUM_CHUNKS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vid_num_chunks::R`](R) reader structure"]
impl crate::Readable for VID_NUM_CHUNKS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vid_num_chunks::W`](W) writer structure"]
impl crate::Writable for VID_NUM_CHUNKS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VID_NUM_CHUNKS to value 0"]
impl crate::Resettable for VID_NUM_CHUNKS_SPEC {}
