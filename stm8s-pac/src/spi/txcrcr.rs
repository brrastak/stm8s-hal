#[doc = "Register `TXCRCR` reader"]
pub type R = crate::R<TXCRCR_SPEC>;
#[doc = "Register `TXCRCR` writer"]
pub type W = crate::W<TXCRCR_SPEC>;
#[doc = "Field `TXCRC` reader - "]
pub type TXCRC_R = crate::FieldReader;
#[doc = "Field `TXCRC` writer - "]
pub type TXCRC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn txcrc(&self) -> TXCRC_R {
        TXCRC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn txcrc(&mut self) -> TXCRC_W<TXCRCR_SPEC> {
        TXCRC_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SPI Tx CRC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txcrcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txcrcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXCRCR_SPEC;
impl crate::RegisterSpec for TXCRCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`txcrcr::R`](R) reader structure"]
impl crate::Readable for TXCRCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txcrcr::W`](W) writer structure"]
impl crate::Writable for TXCRCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXCRCR to value 0xff"]
impl crate::Resettable for TXCRCR_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
