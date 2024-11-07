#[doc = "Register `REG_MAP3` reader"]
pub type R = crate::R<REG_MAP3_SPEC>;
#[doc = "Register `REG_MAP3` writer"]
pub type W = crate::W<REG_MAP3_SPEC>;
#[doc = "Field `MAP3` reader - x"]
pub type MAP3_R = crate::FieldReader<u32>;
#[doc = "Field `MAP3` writer - x"]
pub type MAP3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - x"]
    #[inline(always)]
    pub fn map3(&self) -> MAP3_R {
        MAP3_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REG_MAP3")
            .field("map3", &self.map3())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - x"]
    #[inline(always)]
    pub fn map3(&mut self) -> MAP3_W<REG_MAP3_SPEC> {
        MAP3_W::new(self, 0)
    }
}
#[doc = "x\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_map3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_map3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REG_MAP3_SPEC;
impl crate::RegisterSpec for REG_MAP3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_map3::R`](R) reader structure"]
impl crate::Readable for REG_MAP3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`reg_map3::W`](W) writer structure"]
impl crate::Writable for REG_MAP3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG_MAP3 to value 0"]
impl crate::Resettable for REG_MAP3_SPEC {
    const RESET_VALUE: u32 = 0;
}
