#[doc = "Register `HOST_SLCHOST_WIN_CMD` reader"]
pub type R = crate::R<HOST_SLCHOST_WIN_CMD_SPEC>;
#[doc = "Register `HOST_SLCHOST_WIN_CMD` writer"]
pub type W = crate::W<HOST_SLCHOST_WIN_CMD_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`host_slchost_win_cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_slchost_win_cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_SLCHOST_WIN_CMD_SPEC;
impl crate::RegisterSpec for HOST_SLCHOST_WIN_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_slchost_win_cmd::R`](R) reader structure"]
impl crate::Readable for HOST_SLCHOST_WIN_CMD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`host_slchost_win_cmd::W`](W) writer structure"]
impl crate::Writable for HOST_SLCHOST_WIN_CMD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOST_SLCHOST_WIN_CMD to value 0"]
impl crate::Resettable for HOST_SLCHOST_WIN_CMD_SPEC {
    const RESET_VALUE: u32 = 0;
}
