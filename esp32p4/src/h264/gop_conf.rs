#[doc = "Register `GOP_CONF` reader"]
pub type R = crate::R<GOP_CONF_SPEC>;
#[doc = "Register `GOP_CONF` writer"]
pub type W = crate::W<GOP_CONF_SPEC>;
#[doc = "Field `DUAL_STREAM_MODE` reader - Configures whether or not to enable dual stream mode. When this field is set to 1, H264_FRAME_MODE field must be set to 1 too.\\\\0: Normal mode\\\\1: Dual stream mode"]
pub type DUAL_STREAM_MODE_R = crate::BitReader;
#[doc = "Field `DUAL_STREAM_MODE` writer - Configures whether or not to enable dual stream mode. When this field is set to 1, H264_FRAME_MODE field must be set to 1 too.\\\\0: Normal mode\\\\1: Dual stream mode"]
pub type DUAL_STREAM_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GOP_NUM` reader - Configures the frame number of one GOP.\\\\0: The frame number of one GOP is infinite\\\\Others: Actual frame number of one GOP"]
pub type GOP_NUM_R = crate::FieldReader;
#[doc = "Field `GOP_NUM` writer - Configures the frame number of one GOP.\\\\0: The frame number of one GOP is infinite\\\\Others: Actual frame number of one GOP"]
pub type GOP_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Configures whether or not to enable dual stream mode. When this field is set to 1, H264_FRAME_MODE field must be set to 1 too.\\\\0: Normal mode\\\\1: Dual stream mode"]
    #[inline(always)]
    pub fn dual_stream_mode(&self) -> DUAL_STREAM_MODE_R {
        DUAL_STREAM_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:8 - Configures the frame number of one GOP.\\\\0: The frame number of one GOP is infinite\\\\Others: Actual frame number of one GOP"]
    #[inline(always)]
    pub fn gop_num(&self) -> GOP_NUM_R {
        GOP_NUM_R::new(((self.bits >> 1) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GOP_CONF")
            .field("dual_stream_mode", &self.dual_stream_mode())
            .field("gop_num", &self.gop_num())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to enable dual stream mode. When this field is set to 1, H264_FRAME_MODE field must be set to 1 too.\\\\0: Normal mode\\\\1: Dual stream mode"]
    #[inline(always)]
    pub fn dual_stream_mode(&mut self) -> DUAL_STREAM_MODE_W<'_, GOP_CONF_SPEC> {
        DUAL_STREAM_MODE_W::new(self, 0)
    }
    #[doc = "Bits 1:8 - Configures the frame number of one GOP.\\\\0: The frame number of one GOP is infinite\\\\Others: Actual frame number of one GOP"]
    #[inline(always)]
    pub fn gop_num(&mut self) -> GOP_NUM_W<'_, GOP_CONF_SPEC> {
        GOP_NUM_W::new(self, 1)
    }
}
#[doc = "GOP related configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`gop_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gop_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GOP_CONF_SPEC;
impl crate::RegisterSpec for GOP_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gop_conf::R`](R) reader structure"]
impl crate::Readable for GOP_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gop_conf::W`](W) writer structure"]
impl crate::Writable for GOP_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GOP_CONF to value 0"]
impl crate::Resettable for GOP_CONF_SPEC {}
