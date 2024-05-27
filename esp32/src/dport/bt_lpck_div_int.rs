///Register `BT_LPCK_DIV_INT` reader
pub type R = crate::R<BT_LPCK_DIV_INT_SPEC>;
///Register `BT_LPCK_DIV_INT` writer
pub type W = crate::W<BT_LPCK_DIV_INT_SPEC>;
///Field `BT_LPCK_DIV_NUM` reader -
pub type BT_LPCK_DIV_NUM_R = crate::FieldReader<u16>;
///Field `BT_LPCK_DIV_NUM` writer -
pub type BT_LPCK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `BTEXTWAKEUP_REQ` reader -
pub type BTEXTWAKEUP_REQ_R = crate::BitReader;
///Field `BTEXTWAKEUP_REQ` writer -
pub type BTEXTWAKEUP_REQ_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:11
    #[inline(always)]
    pub fn bt_lpck_div_num(&self) -> BT_LPCK_DIV_NUM_R {
        BT_LPCK_DIV_NUM_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bit 12
    #[inline(always)]
    pub fn btextwakeup_req(&self) -> BTEXTWAKEUP_REQ_R {
        BTEXTWAKEUP_REQ_R::new(((self.bits >> 12) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BT_LPCK_DIV_INT")
            .field("bt_lpck_div_num", &self.bt_lpck_div_num())
            .field("btextwakeup_req", &self.btextwakeup_req())
            .finish()
    }
}
impl W {
    ///Bits 0:11
    #[inline(always)]
    #[must_use]
    pub fn bt_lpck_div_num(&mut self) -> BT_LPCK_DIV_NUM_W<BT_LPCK_DIV_INT_SPEC> {
        BT_LPCK_DIV_NUM_W::new(self, 0)
    }
    ///Bit 12
    #[inline(always)]
    #[must_use]
    pub fn btextwakeup_req(&mut self) -> BTEXTWAKEUP_REQ_W<BT_LPCK_DIV_INT_SPEC> {
        BTEXTWAKEUP_REQ_W::new(self, 12)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`bt_lpck_div_int::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bt_lpck_div_int::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BT_LPCK_DIV_INT_SPEC;
impl crate::RegisterSpec for BT_LPCK_DIV_INT_SPEC {
    type Ux = u32;
}
///`read()` method returns [`bt_lpck_div_int::R`](R) reader structure
impl crate::Readable for BT_LPCK_DIV_INT_SPEC {}
///`write(|w| ..)` method takes [`bt_lpck_div_int::W`](W) writer structure
impl crate::Writable for BT_LPCK_DIV_INT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BT_LPCK_DIV_INT to value 0xff
impl crate::Resettable for BT_LPCK_DIV_INT_SPEC {
    const RESET_VALUE: u32 = 0xff;
}
