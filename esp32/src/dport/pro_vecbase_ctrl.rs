#[doc = "Register `PRO_VECBASE_CTRL` reader"]
pub type R = crate::R<PRO_VECBASE_CTRL_SPEC>;
#[doc = "Register `PRO_VECBASE_CTRL` writer"]
pub type W = crate::W<PRO_VECBASE_CTRL_SPEC>;
#[doc = "Field `PRO_OUT_VECBASE_SEL` reader - "]
pub type PRO_OUT_VECBASE_SEL_R = crate::FieldReader;
#[doc = "Field `PRO_OUT_VECBASE_SEL` writer - "]
pub type PRO_OUT_VECBASE_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pro_out_vecbase_sel(&self) -> PRO_OUT_VECBASE_SEL_R {
        PRO_OUT_VECBASE_SEL_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_VECBASE_CTRL")
            .field("pro_out_vecbase_sel", &self.pro_out_vecbase_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pro_out_vecbase_sel(&mut self) -> PRO_OUT_VECBASE_SEL_W<PRO_VECBASE_CTRL_SPEC> {
        PRO_OUT_VECBASE_SEL_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_vecbase_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pro_vecbase_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_VECBASE_CTRL_SPEC;
impl crate::RegisterSpec for PRO_VECBASE_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_vecbase_ctrl::R`](R) reader structure"]
impl crate::Readable for PRO_VECBASE_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pro_vecbase_ctrl::W`](W) writer structure"]
impl crate::Writable for PRO_VECBASE_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRO_VECBASE_CTRL to value 0"]
impl crate::Resettable for PRO_VECBASE_CTRL_SPEC {}
