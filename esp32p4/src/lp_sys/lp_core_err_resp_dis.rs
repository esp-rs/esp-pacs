///Register `LP_CORE_ERR_RESP_DIS` reader
pub type R = crate::R<LP_CORE_ERR_RESP_DIS_SPEC>;
///Register `LP_CORE_ERR_RESP_DIS` writer
pub type W = crate::W<LP_CORE_ERR_RESP_DIS_SPEC>;
///Field `LP_CORE_ERR_RESP_DIS` reader - Set bit0 to disable ibus err resp;Set bit1 to disable dbus err resp; Set bit 2 to disable ahb err resp.
pub type LP_CORE_ERR_RESP_DIS_R = crate::FieldReader;
///Field `LP_CORE_ERR_RESP_DIS` writer - Set bit0 to disable ibus err resp;Set bit1 to disable dbus err resp; Set bit 2 to disable ahb err resp.
pub type LP_CORE_ERR_RESP_DIS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - Set bit0 to disable ibus err resp;Set bit1 to disable dbus err resp; Set bit 2 to disable ahb err resp.
    #[inline(always)]
    pub fn lp_core_err_resp_dis(&self) -> LP_CORE_ERR_RESP_DIS_R {
        LP_CORE_ERR_RESP_DIS_R::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_CORE_ERR_RESP_DIS")
            .field("lp_core_err_resp_dis", &self.lp_core_err_resp_dis())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Set bit0 to disable ibus err resp;Set bit1 to disable dbus err resp; Set bit 2 to disable ahb err resp.
    #[inline(always)]
    #[must_use]
    pub fn lp_core_err_resp_dis(&mut self) -> LP_CORE_ERR_RESP_DIS_W<LP_CORE_ERR_RESP_DIS_SPEC> {
        LP_CORE_ERR_RESP_DIS_W::new(self, 0)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`lp_core_err_resp_dis::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_core_err_resp_dis::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LP_CORE_ERR_RESP_DIS_SPEC;
impl crate::RegisterSpec for LP_CORE_ERR_RESP_DIS_SPEC {
    type Ux = u32;
}
///`read()` method returns [`lp_core_err_resp_dis::R`](R) reader structure
impl crate::Readable for LP_CORE_ERR_RESP_DIS_SPEC {}
///`write(|w| ..)` method takes [`lp_core_err_resp_dis::W`](W) writer structure
impl crate::Writable for LP_CORE_ERR_RESP_DIS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LP_CORE_ERR_RESP_DIS to value 0
impl crate::Resettable for LP_CORE_ERR_RESP_DIS_SPEC {
    const RESET_VALUE: u32 = 0;
}
