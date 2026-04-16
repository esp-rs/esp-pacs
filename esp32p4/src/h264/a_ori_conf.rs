#[doc = "Register `A_ORI_CONF` reader"]
pub type R = crate::R<A_ORI_CONF_SPEC>;
#[doc = "Register `A_ORI_CONF` writer"]
pub type W = crate::W<A_ORI_CONF_SPEC>;
#[doc = "Field `A_ORI_COLOR_SPACE` reader - Configures video A original picture color space.\\\\0: RGB888\\\\1: RGB565\\\\2: YUV444\\\\3: YUV422\\\\4: YUV420\\\\5: GRAY\\\\Others: Invalid"]
pub type A_ORI_COLOR_SPACE_R = crate::FieldReader;
#[doc = "Field `A_ORI_COLOR_SPACE` writer - Configures video A original picture color space.\\\\0: RGB888\\\\1: RGB565\\\\2: YUV444\\\\3: YUV422\\\\4: YUV420\\\\5: GRAY\\\\Others: Invalid"]
pub type A_ORI_COLOR_SPACE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Configures video A original picture color space.\\\\0: RGB888\\\\1: RGB565\\\\2: YUV444\\\\3: YUV422\\\\4: YUV420\\\\5: GRAY\\\\Others: Invalid"]
    #[inline(always)]
    pub fn a_ori_color_space(&self) -> A_ORI_COLOR_SPACE_R {
        A_ORI_COLOR_SPACE_R::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("A_ORI_CONF")
            .field("a_ori_color_space", &self.a_ori_color_space())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Configures video A original picture color space.\\\\0: RGB888\\\\1: RGB565\\\\2: YUV444\\\\3: YUV422\\\\4: YUV420\\\\5: GRAY\\\\Others: Invalid"]
    #[inline(always)]
    pub fn a_ori_color_space(&mut self) -> A_ORI_COLOR_SPACE_W<'_, A_ORI_CONF_SPEC> {
        A_ORI_COLOR_SPACE_W::new(self, 0)
    }
}
#[doc = "Video A original picture configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`a_ori_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`a_ori_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct A_ORI_CONF_SPEC;
impl crate::RegisterSpec for A_ORI_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`a_ori_conf::R`](R) reader structure"]
impl crate::Readable for A_ORI_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`a_ori_conf::W`](W) writer structure"]
impl crate::Writable for A_ORI_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets A_ORI_CONF to value 0x04"]
impl crate::Resettable for A_ORI_CONF_SPEC {
    const RESET_VALUE: u32 = 0x04;
}
