#[doc = "Register `CTRL_REG` reader"]
pub type R = crate::R<CTRL_REG_SPEC>;
#[doc = "Register `CTRL_REG` writer"]
pub type W = crate::W<CTRL_REG_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Exact name and meaning unknown, used initializing the MAC\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_REG_SPEC;
impl crate::RegisterSpec for CTRL_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl_reg::R`](R) reader structure"]
impl crate::Readable for CTRL_REG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl_reg::W`](W) writer structure"]
impl crate::Writable for CTRL_REG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL_REG to value 0"]
impl crate::Resettable for CTRL_REG_SPEC {
    const RESET_VALUE: u32 = 0;
}
